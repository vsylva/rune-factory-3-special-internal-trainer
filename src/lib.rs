mod asm;
mod gui;
mod hook;
mod util;
mod var;

#[link(name = "user32")]
extern "system" {
    pub(crate) fn GetAsyncKeyState(vKey: i32) -> u16;
    pub(crate) fn IsIconic(hWnd: isize) -> i32;
    pub(crate) fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> isize;
}

#[no_mangle]
unsafe extern "system" fn DllMain(
    h_module: isize,
    ul_reason_for_call: u32,
    _lp_reserved: *mut core::ffi::c_void,
) -> i32 {
    if ul_reason_for_call == 1 {
        std::thread::spawn(move || {
            if !util::is_dll_loaded("Sandll.dll", 1, 10) {
                vcheat::internal::free_dll_exit_thread(h_module, 0);
            }

            init_hook();

            if !util::is_dll_loaded("steam_api64.dll", 1, 10) {
                vcheat::internal::free_dll_exit_thread(h_module, 0);
            }

            init_hudhook(h_module);
        });
    } else if ul_reason_for_call == 0 {
    }

    1
}

unsafe fn init_hook() {
    let mod_info = vcheat::internal::get_mod_info("Sandll.dll").unwrap();

    var::SANDLL_ADDR = mod_info.addr as i64;

    let mod_data = vcheat::read_mem(
        vcheat::internal::get_proc_handle(),
        mod_info.addr,
        mod_info.size as usize,
    )
    .unwrap();

    minhook_raw::initialize();

    hook::hooks(mod_info.addr, &mod_data);
}

unsafe fn init_hudhook(h_module: isize) {
    var::WINDOW_HANDLE = util::get_window_handle("Rune Factory 3 Special\0");

    if let Err(e) = ::hudhook::Hudhook::builder()
        .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(gui::MyRenderLoop)
        .with_hmodule(hudhook::windows::Win32::Foundation::HINSTANCE(h_module))
        .build()
        .apply()
    {
        ::hudhook::tracing::error!("Couldn't apply hooks: {e:?}");
        ::hudhook::eject();
    }
}
