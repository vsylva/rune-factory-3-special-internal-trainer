pub(crate) mod asm;
pub(crate) mod byte;
pub(crate) mod inline;

// target 8
pub(crate) static mut SAVE_LOAD_HOOK: crate::hook::inline::Hook = unsafe { ::core::mem::zeroed() };
pub(crate) static mut SAVE_LOAD_MARK: i64 = 0;

pub(crate) static mut COIN_ADDR: *mut u32 = ::core::ptr::null_mut();
pub(crate) static mut COIN_LAST: u32 = 0;
pub(crate) static mut COIN_MAX: bool = false;

pub(crate) static mut WOOD_ADDR: *mut u16 = ::core::ptr::null_mut();
pub(crate) static mut WOOD_LAST: u16 = 0;
pub(crate) static mut WOOD_MAX: bool = false;

// target 5 + 3
// size 37
pub(crate) static mut FISHING_HOOK: crate::hook::inline::Hook = unsafe { ::core::mem::zeroed() };

// target 5 + 1
// size 25
pub(crate) static mut AUTO_PRESS_HOOK: crate::hook::inline::Hook = unsafe { ::core::mem::zeroed() };
pub(crate) static mut AUTO_PRESS_MARK: i64 = 0;

// target 5 + 1
pub(crate) static mut WALK_THROUGH_WALLS_HOOK: crate::hook::inline::Hook =
    unsafe { ::core::mem::zeroed() };

// target 5 + 1
pub(crate) static mut FRIENDSHIP_MUL_HOOK: crate::hook::inline::Hook =
    unsafe { ::core::mem::zeroed() };

// target 5 + 1
pub(crate) static mut INSTANT_CROP_GROWTH_HOOK: crate::hook::inline::Hook =
    unsafe { ::core::mem::zeroed() };

// target 5 + 1
pub(crate) static mut SKILL_EXP_MUL_HOOK: crate::hook::inline::Hook =
    unsafe { ::core::mem::zeroed() };

// target 5 + 3
pub(crate) static mut FARM_HOOK: crate::hook::inline::Hook = unsafe { ::core::mem::zeroed() };

pub(crate) static mut SOIL_QUALITY_TOGGLE: bool = false;
pub(crate) static mut SOIL_QUALITY_MARK: i64 = 0;
pub(crate) static mut SOIL_QUALITY_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(SOIL_QUALITY_MARK) } as *const i64;

pub(crate) static mut WATERING_PLOTS_TOGGLE: bool = false;
pub(crate) static mut WATERING_PLOTS_MARK: i64 = 0;
pub(crate) static mut WATERING_PLOTS_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(WATERING_PLOTS_MARK) } as *const i64;

pub(crate) static mut TILTH_PLOTS_TOGGLE: bool = false;
pub(crate) static mut TILTH_PLOTS_MARK: i64 = 0;
pub(crate) static mut TILTH_PLOTS_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(TILTH_PLOTS_MARK) } as *const i64;

pub(crate) static mut PLANT_PLOTS_TOGGLE: bool = false;
pub(crate) static mut PLANT_PLOTS_MARK: i64 = 0;
pub(crate) static mut PLANT_PLOTS_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(PLANT_PLOTS_MARK) } as *const i64;
pub(crate) static mut CROP_PROP: crate::ui::CropProp = crate::ui::CropProp {
    ty: 0,
    growth_stage_and_lv: 0x10,
};
pub(crate) static mut CROP_PROP_POINTER: *const crate::ui::CropProp =
    unsafe { std::ptr::addr_of!(CROP_PROP) } as *const crate::ui::CropProp;

// target 6
pub(crate) static mut TIME_HOOK: crate::hook::inline::Hook = unsafe { ::core::mem::zeroed() };
pub(crate) static mut TIME_POINTER: *mut crate::hook::inline::Time = ::core::ptr::null_mut();
pub(crate) static mut TIME_POINTER_POINTER: *const *mut crate::hook::inline::Time =
    unsafe { ::core::ptr::addr_of!(TIME_POINTER) };
