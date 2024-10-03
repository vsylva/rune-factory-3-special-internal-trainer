mod hook;
mod ui;

static mut MOD_SANDLL: libmem::Module = libmem::Module {
    base: 0,
    end: 0,
    size: 0,
    path: String::new(),
    name: String::new(),
};

#[no_mangle]
unsafe extern "system" fn DllMain(
    h_module: isize,
    ul_reason_for_call: u32,
    _lp_reserved: *mut core::ffi::c_void,
) -> i32 {
    if ul_reason_for_call == 1 {
        hudhook::windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls(
            hudhook::windows::Win32::Foundation::HMODULE(h_module),
        )
        .unwrap();

        std::thread::spawn(move || {
            let time_begin = ::std::time::Instant::now();
            let time_one_sec = ::std::time::Duration::from_secs(1);

            loop {
                ::std::thread::sleep(time_one_sec);

                if time_begin.elapsed().as_secs() > 30 {
                    return;
                }

                if let (Some(mi), Some(mi1)) = (
                    libmem::find_module("Live2DCubismCore.dll"),
                    libmem::find_module("SanDLL.dll"),
                ) {
                    if mi.base != 0 && mi1.base != 0 {
                        crate::MOD_SANDLL = mi1;
                        break;
                    }
                }
            }

            minhook_raw::initialize();

            crate::hook::create_hook(MOD_SANDLL.base, MOD_SANDLL.size);

            if let Err(_) = ::hudhook::Hudhook::builder()
                .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(ui::Ui)
                .with_hmodule(hudhook::windows::Win32::Foundation::HINSTANCE(h_module))
                .build()
                .apply()
            {
                ::hudhook::eject();
            }
        });
    } else if ul_reason_for_call == 0 {
    }

    1
}
