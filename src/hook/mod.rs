pub(crate) mod asm;
pub(crate) mod inline;

pub(crate) static mut SAVE_LOAD_HOOK: crate::hook::inline::AsmHook =
    unsafe { ::core::mem::zeroed() };
pub(crate) static mut SAVE_LOAD_MARK: i64 = 0;

pub(crate) static mut COIN_ADDR: *mut u32 = ::core::ptr::null_mut();
pub(crate) static mut COIN_LAST: u32 = 0;
pub(crate) static mut COIN_MAX: bool = false;

pub(crate) static mut WOOD_ADDR: *mut u16 = ::core::ptr::null_mut();
pub(crate) static mut WOOD_LAST: u16 = 0;
pub(crate) static mut WOOD_MAX: bool = false;

pub(crate) static mut FISHING_HOOK: crate::hook::inline::AsmHook = unsafe { ::core::mem::zeroed() };

pub(crate) static mut AUTO_PRESS_HOOK: crate::hook::inline::AsmHook =
    unsafe { ::core::mem::zeroed() };
pub(crate) static mut AUTO_PRESS_MARK: i64 = 0;

pub(crate) static mut WALK_THROUGH_WALLS_HOOK: crate::hook::inline::AsmHook =
    unsafe { ::core::mem::zeroed() };

pub(crate) static mut FRIENDSHIP_MUL_HOOK: crate::hook::inline::AsmHook =
    unsafe { ::core::mem::zeroed() };

pub(crate) static mut INSTANT_CROP_GROWTH_HOOK: crate::hook::inline::AsmHook =
    unsafe { ::core::mem::zeroed() };

pub(crate) static mut SKILL_EXP_MUL_HOOK: crate::hook::inline::AsmHook =
    unsafe { ::core::mem::zeroed() };

pub(crate) static mut INF_MISSION_HOOK: crate::hook::inline::AsmHook =
    unsafe { ::core::mem::zeroed() };

pub(crate) static mut FARM_HOOK: crate::hook::inline::AsmHook = unsafe { ::core::mem::zeroed() };

pub(crate) static mut SOIL_QUALITY_TOGGLE: bool = false;
pub(crate) static mut SOIL_QUALITY_MARK: i64 = 0;

pub(crate) static mut WATERING_PLOTS_TOGGLE: bool = false;
pub(crate) static mut WATERING_PLOTS_MARK: i64 = 0;

pub(crate) static mut TILTH_PLOTS_TOGGLE: bool = false;
pub(crate) static mut TILTH_PLOTS_MARK: i64 = 0;

pub(crate) static mut PLANT_PLOTS_TOGGLE: bool = false;
pub(crate) static mut PLANT_PLOTS_MARK: i64 = 0;
pub(crate) static mut CROP_PROP: crate::ui::CropProp = crate::ui::CropProp {
    type_: 0,
    data: crate::ui::CropPropType { stage: 0 },
};

pub(crate) static mut TIME_HOOK: crate::hook::inline::AsmHook = unsafe { ::core::mem::zeroed() };
pub(crate) static mut TIME_POINTER: *mut crate::hook::inline::Time = ::core::ptr::null_mut();

pub(crate) unsafe fn create_hook(mod_addr: *mut ::core::ffi::c_void, mod_data: &[u8]) {
    COIN_ADDR = (crate::SANDLL_ADDR + 0x2AD192C) as *mut u32;
    WOOD_ADDR = (crate::SANDLL_ADDR + 0x2AD1930) as *mut u16;

    crate::hook::SAVE_LOAD_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(
            mod_addr,
            mod_data,
            "66 C1 E0 05 44 0F B6 CA 66 83 C0 04 44 0F B7 C0 4C 03 C1",
            8,
        )
        .gen_detour(crate::hook::asm::save_load as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::AUTO_PRESS_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "66 F7 D2 66 23 D0", 6)
        .gen_detour(crate::hook::asm::auto_press as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::FISHING_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "0F B7 48 18 66 83 F9 03", 8)
        .gen_detour(crate::hook::asm::fishing as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::WALK_THROUGH_WALLS_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "48 8B F2 48 85 C9", 6)
        .gen_detour(
            crate::hook::asm::walk_through_walls as *mut ::core::ffi::c_void,
            64,
        )
        .create_hook()
        .to_owned();

    crate::hook::FRIENDSHIP_MUL_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "44 8B CA 4D 85 DB", 6)
        .gen_detour(
            crate::hook::asm::friendship_mul as *mut ::core::ffi::c_void,
            64,
        )
        .create_hook()
        .to_owned();

    crate::hook::INSTANT_CROP_GROWTH_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "8B 10 D1 EA 83 E2 7F 74", 7)
        .gen_detour(
            crate::hook::asm::crop_instant_growth as *mut ::core::ffi::c_void,
            64,
        )
        .create_hook()
        .to_owned();

    crate::hook::SKILL_EXP_MUL_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "4C 63 C2 0F B7 CE", 6)
        .gen_detour(
            crate::hook::asm::skill_exp_mul as *mut ::core::ffi::c_void,
            64,
        )
        .create_hook()
        .to_owned();

    crate::hook::FARM_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "48 83 C3 08 66 41 3B FF", 8)
        .gen_detour(crate::hook::asm::farm as *mut ::core::ffi::c_void, 0xFF)
        .create_hook()
        .to_owned();

    crate::hook::TIME_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "03 D0 41 01 51 04", 6)
        .gen_detour(crate::hook::asm::time as *mut ::core::ffi::c_void, 0xFF)
        .create_hook()
        .to_owned();

    crate::hook::INF_MISSION_HOOK = ::core::mem::zeroed::<crate::hook::inline::AsmHook>()
        .set_data(mod_addr, mod_data, "48 8B 5A 08 41 8D 49 FF", 8)
        .gen_detour(
            crate::hook::asm::inf_mission as *mut ::core::ffi::c_void,
            64,
        )
        .create_hook()
        .to_owned();
}
