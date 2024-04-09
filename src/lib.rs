mod hook;
mod ui;

#[link(name = "user32")]
extern "system" {
    pub(crate) fn GetAsyncKeyState(vKey: i32) -> u16;
}

pub(crate) static mut SANDLL_ADDR: i64 = 0;

#[no_mangle]
unsafe extern "system" fn DllMain(
    h_module: isize,
    ul_reason_for_call: u32,
    _lp_reserved: *mut core::ffi::c_void,
) -> i32 {
    if ul_reason_for_call == 1 {
        std::thread::spawn(move || {
            let now = ::std::time::Instant::now();
            let dur = ::std::time::Duration::from_secs(1);

            let mod_info;

            loop {
                if now.elapsed().as_secs() < 30 {
                    if let (Ok(mi), Ok(mi1)) = (
                        vcheat::internal::get_mod_info("Live2DCubismCore.dll"),
                        vcheat::internal::get_mod_info("Sandll.dll"),
                    ) {
                        if mi.handle != 0 && mi1.handle != 0 {
                            mod_info = mi1;
                            break;
                        }
                    }
                } else {
                    vcheat::internal::free_dll_exit_thread(h_module, 0);
                }

                ::std::thread::sleep(dur);
            }

            ::std::thread::sleep(::std::time::Duration::from_secs(3));

            crate::SANDLL_ADDR = mod_info.addr as i64;

            let mut mod_data = vec![0u8; mod_info.size as usize];

            std::ptr::copy(
                mod_info.addr.cast::<u8>(),
                mod_data.as_mut_ptr(),
                mod_info.size as usize,
            );

            minhook_raw::initialize();

            crate::hook::create_hook(mod_info.addr, &mod_data);

            drop(mod_data);

            if let Err(_) = ::hudhook::Hudhook::builder()
                .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(crate::ui::renderloop::RenderLoop)
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
