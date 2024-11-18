mod hook;
mod trainer;

static mut SANNDLL信息: libmem::Module = libmem::Module {
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
    _lp_reserved: *mut ::core::ffi::c_void,
) -> i32 {
    if ul_reason_for_call == 1 {
        hudhook::windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls(
            hudhook::windows::Win32::Foundation::HMODULE(h_module),
        )
        .unwrap();

        ::std::thread::spawn(move || init(h_module));
    }

    1
}

unsafe fn init(h_module: isize) {
    let 修改器temp = trainer::修改器 {
        显示界面: true,
        选择的作物: trainer::作物类型::无,
        作物类型列表: &[],
        选择的作物等级: trainer::作物等级::LV1,
        作物等级列表: &[],
        选择的作物生长阶段: trainer::作物生长阶段::一阶段,
        作物生长阶段列表: &[],
        选择的秒: 0,
        秒列表: Vec::new(),
        选择的时: 0,
        时列表: Vec::new(),
        选择的天: 1,
        天列表: Vec::new(),
        选择的季节: trainer::季节::春,
        季节列表: &[],
        选择的年: 1,
        年列表: Vec::new(),
        选择的流速: trainer::时间流速::默认,
        时间流速列表: &[],
    };

    let time_begin = ::std::time::Instant::now();
    let time_one_sec = ::std::time::Duration::from_secs(1);

    while time_begin.elapsed().as_secs() < 60 {
        if let (Some(live2d), Some(sandll)) = (
            libmem::find_module("Live2DCubismCore.dll"),
            libmem::find_module("SanDLL.dll"),
        ) {
            if live2d.base != 0 && sandll.base != 0 {
                SANNDLL信息 = sandll;
                break;
            }
        }

        ::std::thread::sleep(time_one_sec);
    }

    if ::hudhook::Hudhook::builder()
        .with::<hudhook::hooks::dx11::ImguiDx11Hooks>(修改器temp)
        .with_hmodule(hudhook::windows::Win32::Foundation::HINSTANCE(h_module))
        .build()
        .apply()
        .is_err()
    {
        ::hudhook::eject();
    }
}
