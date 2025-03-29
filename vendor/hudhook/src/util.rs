//! General-purpose utilities. These are used across the [`crate`] but have
//! proven useful in client code as well.

use std::{
    ffi::{OsString, c_void},
    mem::ManuallyDrop,
    os::windows::ffi::OsStringExt,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use windows::{
    Win32::{
        Foundation::{HANDLE, HMODULE, HWND, MAX_PATH, RECT},
        Graphics::Direct3D12::{
            D3D12_FENCE_FLAG_NONE, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_0,
            D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES, D3D12_RESOURCE_BARRIER_FLAG_NONE,
            D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_STATES,
            D3D12_RESOURCE_TRANSITION_BARRIER, D3D12GetDebugInterface, ID3D12Debug, ID3D12Device,
            ID3D12Fence, ID3D12Resource,
        },
        System::{
            LibraryLoader::{
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
                GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, GetModuleFileNameW,
                GetModuleHandleExA,
            },
            Memory::{
                MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
                PAGE_PROTECTION_FLAGS, PAGE_READONLY, PAGE_READWRITE, VirtualQuery,
            },
            SystemInformation::{GetSystemInfo, SYSTEM_INFO},
            Threading::{CREATE_EVENT, CreateEventExW, WaitForSingleObjectEx},
        },
        UI::WindowsAndMessaging::GetClientRect,
    },
    core::s,
};

/// Helper for fallible [`windows`] APIs that have an out-param with a default
/// value.
///
/// # Example
///
/// ```
/// let swap_chain_desc = try_out_param(|sd| unsafe { self.swap_chain.GetDesc1(sd) })?;
/// ```
pub fn try_out_param<T, F, E, O>(mut f: F) -> Result<T, E>
where
    T: Default,
    F: FnMut(&mut T) -> Result<O, E>,
{
    let mut t: T = Default::default();
    match f(&mut t) {
        Ok(_) => Ok(t),
        Err(e) => Err(e),
    }
}

/// Helper for fallible [`windows`] APIs that have an optional pointer
/// out-param.
///
/// # Example
///
/// ```
/// let dev: ID3D12Device =
///     try_out_ptr(|v| unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, v) })
///         .expect("D3D12CreateDevice failed");
/// ```
pub fn try_out_ptr<T, F, E, O>(mut f: F) -> Result<T, E>
where
    F: FnMut(&mut Option<T>) -> Result<O, E>,
{
    let mut t: Option<T> = None;
    match f(&mut t) {
        Ok(_) => Ok(t.unwrap()),
        Err(e) => Err(e),
    }
}

/// Helper for fallible [`windows`] APIs that have an optional pointer
/// out-param and an optional pointer err-param.
///
/// # Example
///
/// ```
/// let blob: ID3DBlob = util::try_out_err_blob(|v, err_blob| {
///     D3D12SerializeRootSignature(
///         &root_signature_desc,
///         D3D_ROOT_SIGNATURE_VERSION_1_0,
///         v,
///         Some(err_blob),
///     )
/// })
/// .map_err(print_err_blob("Compiling vertex shader"))?;
/// ```
pub fn try_out_err_blob<T1, T2, F, E, O>(mut f: F) -> Result<T1, (E, T2)>
where
    F: FnMut(&mut Option<T1>, &mut Option<T2>) -> Result<O, E>,
{
    let mut t1: Option<T1> = None;
    let mut t2: Option<T2> = None;
    match f(&mut t1, &mut t2) {
        Ok(_) => Ok(t1.unwrap()),
        Err(e) => Err((e, t2.unwrap())),
    }
}

/// Helper for infallible APIs that have out-params, like OpenGL 3.
///
/// # Example
///
/// ```
/// let vertex_buffer = out_param(|x| unsafe { gl.GenBuffers(1, x) });
/// ```
pub fn out_param<T: Default, F>(f: F) -> T
where
    F: FnOnce(&mut T),
{
    let mut val = Default::default();
    f(&mut val);
    val
}

/// Enables the Direct3D12 debug interface.
///
/// It will not panic if the interface is not available. Call this from your
/// application before a DirectX 12 device is initialized. It could fail in
/// DirectX 12 host applications that will have initialized their device
/// already, but should not fail in other host applications.
pub fn enable_debug_interface() {
    let debug_interface: Result<ID3D12Debug, _> =
        try_out_ptr(|v| unsafe { D3D12GetDebugInterface(v) });

    match debug_interface {
        Ok(debug_interface) => unsafe { debug_interface.EnableDebugLayer() },
        _ => (),
    }
}

/// Helper that returns width and height of a given
/// [`windows::Win32::Foundation::HWND`].
pub fn win_size(hwnd: HWND) -> (i32, i32) {
    let mut rect = RECT::default();
    unsafe { GetClientRect(hwnd, &mut rect).unwrap() };
    (rect.right - rect.left, rect.bottom - rect.top)
}

/// Returns the path of the current module.
pub fn get_dll_path() -> Option<PathBuf> {
    let mut hmodule = HMODULE(0);
    if let Err(_) = unsafe {
        GetModuleHandleExA(
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            s!("DllMain"),
            &mut hmodule,
        )
    } {
        return None;
    }

    let mut sz_filename = [0u16; MAX_PATH as usize];
    let len = unsafe { GetModuleFileNameW(hmodule, &mut sz_filename) } as usize;

    Some(OsString::from_wide(&sz_filename[..len]).into())
}

/// Creates a [`D3D12_RESOURCE_BARRIER`].
///
/// Use this function and the associated [`drop_barrier`] for correctly managing
/// barrier resources.
///
/// RAII was not used due to the complicated signature of
/// [`windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList::ResourceBarrier`].
pub fn create_barrier(
    resource: &ID3D12Resource,
    before: D3D12_RESOURCE_STATES,
    after: D3D12_RESOURCE_STATES,
) -> D3D12_RESOURCE_BARRIER {
    D3D12_RESOURCE_BARRIER {
        Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
        Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
        Anonymous: D3D12_RESOURCE_BARRIER_0 {
            Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                pResource: ManuallyDrop::new(Some(resource.clone())),
                Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                StateBefore: before,
                StateAfter: after,
            }),
        },
    }
}

/// Drops a [`D3D12_RESOURCE_BARRIER`].
///
/// Use this function and the associated [`create_barrier`] for correctly
/// managing barrier resources.
///
/// RAII was not used due to the complicated signature of
/// [`windows::Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList::ResourceBarrier`].
pub fn drop_barrier(barrier: D3D12_RESOURCE_BARRIER) {
    let transition = ManuallyDrop::into_inner(unsafe { barrier.Anonymous.Transition });
    let _ = ManuallyDrop::into_inner(transition.pResource);
}

/// Wrapper around [`windows::Win32::Graphics::Direct3D12::ID3D12Fence`].
pub struct Fence {
    fence: ID3D12Fence,
    value: AtomicU64,
    event: HANDLE,
}

impl Fence {
    /// Construct the fence.
    pub fn new(device: &ID3D12Device) -> windows::core::Result<Self> {
        let fence = unsafe { device.CreateFence(0, D3D12_FENCE_FLAG_NONE) }?;
        let value = AtomicU64::new(0);
        let event = unsafe { CreateEventExW(None, None, CREATE_EVENT(0), 0x1f0003) }?;

        Ok(Fence {
            fence,
            value,
            event,
        })
    }

    /// Retrieve the underlying fence object to pass to the D3D12 APIs.
    pub fn fence(&self) -> &ID3D12Fence {
        &self.fence
    }

    /// Retrieve the current fence value.
    pub fn value(&self) -> u64 {
        self.value.load(Ordering::SeqCst)
    }

    /// Atomically increase the fence value.
    pub fn incr(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    /// Wait for completion of the fence.
    pub fn wait(&self) -> windows::core::Result<()> {
        let value = self.value();
        unsafe {
            if self.fence.GetCompletedValue() < value {
                self.fence.SetEventOnCompletion(value, self.event)?;
                WaitForSingleObjectEx(self.event, u32::MAX, false);
            }
        }

        Ok(())
    }
}

/// Returns a slice of **up to** `limit` elements of type `T` starting at `ptr`.
///
/// If the memory protection of some pages in this region prevents reading from
/// it, the slice is truncated to the first `N` consecutive readable elements.
///
/// # Safety
///
/// - `ptr` must not be a null pointer and must be properly aligned.
/// - Ignoring memory protection, the memory at `ptr` must be valid for at least
///   `limit` elements of type `T` (see [`std::slice::from_raw_parts`]).
pub unsafe fn readable_region<T>(ptr: *const T, limit: usize) -> &'static [T] {
    /// Check if the page pointed to by `ptr` is readable.
    unsafe fn is_readable(
        ptr: *const c_void,
        memory_basic_info: &mut MEMORY_BASIC_INFORMATION,
    ) -> bool {
        // If the page protection has any of these flags set, we can read from it
        const PAGE_READABLE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(
            PAGE_READONLY.0 | PAGE_READWRITE.0 | PAGE_EXECUTE_READ.0 | PAGE_EXECUTE_READWRITE.0,
        );

        (unsafe {
            VirtualQuery(
                Some(ptr),
                memory_basic_info,
                size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        } != 0)
            && (memory_basic_info.Protect & PAGE_READABLE).0 != 0
    }

    // This is probably 0x1000 (4096) bytes
    let page_size_bytes = {
        let mut system_info = SYSTEM_INFO::default();
        unsafe { GetSystemInfo(&mut system_info) };
        system_info.dwPageSize as usize
    };
    let page_align_mask = page_size_bytes - 1;

    // Calculate the starting address of the first and last pages that need to be
    // readable in order to read `limit` elements of type `T` from `ptr`
    let first_page_addr = (ptr as usize) & !page_align_mask;
    let last_page_addr = (ptr as usize + (limit * size_of::<T>()) - 1) & !page_align_mask;

    let mut memory_basic_info = MEMORY_BASIC_INFORMATION::default();
    for page_addr in (first_page_addr..=last_page_addr).step_by(page_size_bytes) {
        if unsafe { is_readable(page_addr as _, &mut memory_basic_info) } {
            continue;
        }

        // If this page is not readable, we can read from `ptr`
        // up to (not including) the start of this page
        //
        // Note: `page_addr` can be less than `ptr` if `ptr` is not page-aligned
        let num_readable = page_addr.saturating_sub(ptr as usize) / size_of::<T>();

        // SAFETY:
        // - `ptr` is a valid pointer to `limit` elements of type `T`
        // - `num_readable` is always less than or equal to `limit`
        return std::slice::from_raw_parts(ptr, num_readable);
    }

    // SAFETY:
    // - `ptr` is a valid pointer to `limit` elements of type `T` and is properly
    //   aligned
    std::slice::from_raw_parts(ptr, limit)
}

#[cfg(test)]
mod tests {
    use windows::Win32::System::Memory::{MEM_COMMIT, PAGE_NOACCESS, VirtualAlloc, VirtualProtect};

    use super::*;

    #[test]
    fn test_readable_region() -> windows::core::Result<()> {
        const PAGE_SIZE: usize = 0x1000;

        let region = unsafe { VirtualAlloc(None, 2 * PAGE_SIZE, MEM_COMMIT, PAGE_READWRITE) };
        if region.is_null() {
            return Err(windows::core::Error::from_win32());
        }

        // Make the second page unreadable
        let mut old_protect = PAGE_PROTECTION_FLAGS::default();
        unsafe {
            VirtualProtect(
                (region as usize + PAGE_SIZE) as _,
                PAGE_SIZE,
                PAGE_NOACCESS,
                &mut old_protect,
            )
        }?;
        assert_eq!(old_protect, PAGE_READWRITE);

        let slice = unsafe { readable_region::<u8>(region as _, PAGE_SIZE) };
        assert_eq!(slice.len(), PAGE_SIZE);

        let slice = unsafe { readable_region::<u8>(region as _, PAGE_SIZE + 1) };
        assert_eq!(slice.len(), PAGE_SIZE);

        let slice = unsafe { readable_region::<u8>((region as usize + PAGE_SIZE) as _, 1) };
        assert!(slice.is_empty());

        let slice = unsafe { readable_region::<u8>((region as usize + PAGE_SIZE - 1) as _, 2) };
        assert_eq!(slice.len(), 1);

        Ok(())
    }
}
