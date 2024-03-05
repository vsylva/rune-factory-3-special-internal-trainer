pub(crate) unsafe fn is_key_down_once(virtual_key_code: i32) -> bool {
    static WAS_KEY_DOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    if (crate::GetAsyncKeyState(virtual_key_code) & 0x8000) != 0 {
        if !WAS_KEY_DOWN.load(std::sync::atomic::Ordering::SeqCst) {
            WAS_KEY_DOWN.store(true, std::sync::atomic::Ordering::SeqCst);
            return true;
        }
    } else {
        WAS_KEY_DOWN.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    false
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

pub(crate) unsafe fn get_window_handle(windows_name: &str) -> isize {
    let windows_name = windows_name.encode_utf16().collect::<Vec<u16>>();
    crate::FindWindowW(::core::ptr::null_mut(), windows_name.as_ptr())
}
