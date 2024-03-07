mod hook;
mod ui;

#[link(name = "user32")]
extern "system" {
    pub(crate) fn GetAsyncKeyState(vKey: i32) -> u16;
    pub(crate) fn IsIconic(hWnd: isize) -> i32;
    pub(crate) fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> isize;
}

pub(crate) static mut SANDLL_ADDR: i64 = 0;
static mut SANDLL_ADDR_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(crate::SANDLL_ADDR) } as *const i64;

#[no_mangle]
unsafe extern "system" fn DllMain(
    h_module: isize,
    ul_reason_for_call: u32,
    _lp_reserved: *mut core::ffi::c_void,
) -> i32 {
    if ul_reason_for_call == 1 {
        std::thread::spawn(move || {
            if !is_dll_loaded("Live2DCubismCore.dll", 1, 30) {
                vcheat::internal::free_dll_exit_thread(h_module, 0);
            }

            if !is_dll_loaded("steam_api64.dll", 1, 10) {
                vcheat::internal::free_dll_exit_thread(h_module, 0);
            }

            if !is_dll_loaded("Sandll.dll", 1, 10) {
                vcheat::internal::free_dll_exit_thread(h_module, 0);
            }

            init_hook();
            init_hudhook(h_module);
        });
    } else if ul_reason_for_call == 0 {
    }

    1
}

unsafe fn init_hook() {
    let mod_info = vcheat::internal::get_mod_info("Sandll.dll").unwrap();

    crate::SANDLL_ADDR = mod_info.addr as i64;

    let mod_data = vcheat::read_mem(
        vcheat::internal::get_proc_handle(),
        mod_info.addr,
        mod_info.size as usize,
    )
    .unwrap();

    minhook_raw::initialize();

    crate::hook::install_hook(mod_info.addr, &mod_data);
}

unsafe fn init_hudhook(h_module: isize) {
    if let Err(e) = ::hudhook::Hudhook::builder()
        .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(ui::RenderLoop)
        .with_hmodule(hudhook::windows::Win32::Foundation::HINSTANCE(h_module))
        .build()
        .apply()
    {
        ::hudhook::tracing::error!("Couldn't apply hooks: {e:?}");
        ::hudhook::eject();
    }
}

pub(crate) unsafe fn is_dll_loaded(dll_name: &str, interval_sec: u64, end_sec: u64) -> bool {
    let now = ::std::time::Instant::now();
    let dur = ::std::time::Duration::from_secs(interval_sec);

    while now.elapsed().as_secs() < end_sec {
        if let Ok(ok) = vcheat::internal::get_mod_info(dll_name) {
            if ok.handle != 0 {
                return true;
            }
        }

        ::std::thread::sleep(dur);
    }

    false
}
