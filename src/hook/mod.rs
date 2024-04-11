pub(crate) mod asm;

pub(crate) static mut COIN_ADDR: *mut u32 = ::core::ptr::null_mut();
pub(crate) static mut COIN_LAST: u32 = 0;
pub(crate) static mut COIN_MAX: bool = false;

pub(crate) static mut WOOD_ADDR: *mut u16 = ::core::ptr::null_mut();
pub(crate) static mut WOOD_LAST: u16 = 0;
pub(crate) static mut WOOD_MAX: bool = false;

pub(crate) static mut SAVE_LOAD_HOOK: AsmHook = AsmHook::new();
pub(crate) static mut SAVE_LOAD_MARK: i64 = 0;

pub(crate) static mut FISHING_HOOK: AsmHook = AsmHook::new();

pub(crate) static mut AUTO_PRESS_HOOK: AsmHook = AsmHook::new();
pub(crate) static mut AUTO_PRESS_MARK: i64 = 0;

pub(crate) static mut WALK_THROUGH_WALLS_HOOK: AsmHook = AsmHook::new();

pub(crate) static mut FRIENDSHIP_MUL_HOOK: AsmHook = AsmHook::new();

pub(crate) static mut INSTANT_CROP_GROWTH_HOOK: AsmHook = AsmHook::new();

pub(crate) static mut SKILL_EXP_MUL_HOOK: AsmHook = AsmHook::new();

pub(crate) static mut INF_MISSION_HOOK: AsmHook = AsmHook::new();

pub(crate) static mut COMBAT_EXP_MUL_HOOK: AsmHook = AsmHook::new();

pub(crate) static mut TAME_HOOK: AsmHook = AsmHook::new();

pub(crate) static mut NO_DEBUFF_HOOK: AsmHook = AsmHook::new();

// pub(crate) static mut DAMAGE_MUL_HOOK: AsmHook = AsmHook::new();

// -----------------------------------------------------------

pub(crate) static mut FARM_HOOK: AsmHook = AsmHook::new();

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

pub(crate) static mut TIME_HOOK: AsmHook = AsmHook::new();
pub(crate) static mut TIME_POINTER: *mut Time = ::core::ptr::null_mut();

pub(crate) unsafe fn create_hook(mod_addr: *mut ::core::ffi::c_void, mod_data: &[u8]) {
    COIN_ADDR = (crate::SANDLL_HANDLE + 0x2AD192C) as *mut u32;
    WOOD_ADDR = (crate::SANDLL_HANDLE + 0x2AD1930) as *mut u16;

    SAVE_LOAD_HOOK.create(
        mod_addr,
        mod_data,
        "66 C1 E0 05 44 0F B6 CA 66 83 C0 04 44 0F B7 C0 4C 03 C1",
        8,
        crate::hook::asm::save_load as *mut ::core::ffi::c_void,
    );

    AUTO_PRESS_HOOK.create(
        mod_addr,
        mod_data,
        "66 F7 D2 66 23 D0",
        6,
        crate::hook::asm::auto_press as *mut ::core::ffi::c_void,
    );

    FISHING_HOOK.create(
        mod_addr,
        mod_data,
        "0F B7 48 18 66 83 F9 03",
        8,
        crate::hook::asm::auto_fishing as *mut ::core::ffi::c_void,
    );

    WALK_THROUGH_WALLS_HOOK.create(
        mod_addr,
        mod_data,
        "48 8B F2 48 85 C9",
        6,
        crate::hook::asm::walk_through_walls as *mut ::core::ffi::c_void,
    );

    FRIENDSHIP_MUL_HOOK.create(
        mod_addr,
        mod_data,
        "44 8B CA 4D 85 DB",
        6,
        crate::hook::asm::friendship_mul as *mut ::core::ffi::c_void,
    );

    INSTANT_CROP_GROWTH_HOOK.create(
        mod_addr,
        mod_data,
        "8B 10 D1 EA 83 E2 7F 74",
        7,
        crate::hook::asm::crop_instant_growth as *mut ::core::ffi::c_void,
    );

    SKILL_EXP_MUL_HOOK.create(
        mod_addr,
        mod_data,
        "4C 63 C2 0F B7 CE",
        6,
        crate::hook::asm::skill_exp_mul as *mut ::core::ffi::c_void,
    );

    FARM_HOOK.create(
        mod_addr,
        mod_data,
        "48 83 C3 08 66 41 3B FF",
        8,
        crate::hook::asm::farm as *mut ::core::ffi::c_void,
    );

    TIME_HOOK.create(
        mod_addr,
        mod_data,
        "03 D0 41 01 51 04",
        6,
        crate::hook::asm::time as *mut ::core::ffi::c_void,
    );

    INF_MISSION_HOOK.create(
        mod_addr,
        mod_data,
        "48 8B 5A 08 41 8D 49 FF",
        8,
        crate::hook::asm::inf_mission as *mut ::core::ffi::c_void,
    );

    COMBAT_EXP_MUL_HOOK.create(
        mod_addr,
        mod_data,
        "41 23 CB 41 03 C8",
        6,
        crate::hook::asm::combat_exp_mul as *mut ::core::ffi::c_void,
    );

    TAME_HOOK.create(
        mod_addr,
        mod_data,
        "48 C1 E9 20 83 E1 7F 66",
        7,
        crate::hook::asm::tame as *mut ::core::ffi::c_void,
    );

    NO_DEBUFF_HOOK.create(
        mod_addr,
        mod_data,
        "BD 00 10 00 00 85 6B 54",
        5,
        crate::hook::asm::no_debuff as *mut ::core::ffi::c_void,
    );

    // DAMAGE_MUL_HOOK.create(
    //     mod_addr,
    //     mod_data,
    //     "8B F0 89 44 24 48",
    //     6,
    //     crate::hook::asm::damage_mul as *mut ::core::ffi::c_void,
    // );
}

pub struct AsmHook {
    pub target_addr: *mut ::core::ffi::c_void,
    pub is_enabled: bool,
}

impl AsmHook {
    pub const fn new() -> Self {
        Self {
            target_addr: ::core::ptr::null_mut(),
            is_enabled: false,
        }
    }
    pub unsafe fn create(
        &mut self,
        mod_addr: *mut ::core::ffi::c_void,
        mod_data: &[u8],
        pat: &str,
        occupied: usize,
        detour_addr: *mut ::core::ffi::c_void,
    ) {
        let pat_offset = vcheat::pat_find(pat, mod_data).unwrap();

        self.target_addr = mod_addr.byte_add(pat_offset);

        let back_addr = self.target_addr.byte_add(occupied);

        let mut end_offset = 0;

        for i in 0..0xFF {
            let ptr = detour_addr.cast::<u8>().byte_add(i);

            if ptr.read() == 0x90 {
                let parts = std::slice::from_raw_parts(ptr, 4);

                if parts.iter().all(|nop| *nop == 0x90) {
                    end_offset = i;
                    break;
                }
            }
        }

        let mut back_shell_code = Vec::new();

        back_shell_code.push(0xFF);
        back_shell_code.push(0x25);
        back_shell_code.push(0x0);
        back_shell_code.push(0x0);
        back_shell_code.push(0x0);
        back_shell_code.push(0x0);

        back_shell_code.extend_from_slice((back_addr as isize).to_le_bytes().as_ref());

        vcheat::write_mem(
            vcheat::internal::get_proc_handle(),
            detour_addr.byte_add(end_offset),
            &back_shell_code,
        )
        .unwrap();

        minhook_raw::create_hook(self.target_addr, detour_addr, ::core::ptr::null_mut());
    }

    pub fn switch(&mut self) {
        if self.is_enabled {
            minhook_raw::enable_hook(self.target_addr);
        } else {
            minhook_raw::disable_hook(self.target_addr);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub(crate) struct Time {
    pub(crate) second: [u8; 4],
    pub(crate) hour: [u8; 4],
    pub(crate) day: [u8; 4],
    pub(crate) season: [u8; 4],
    pub(crate) year: [u8; 4],
    _pading: [u8; 32],
    pub(crate) slow_time_mul: u32,
}

impl Time {
    pub(crate) fn set_second(&mut self, second: u8) {
        (*self).second[0] = second;
    }

    pub(crate) fn set_hour(&mut self, hour: u8) {
        (*self).hour[0] = hour;
    }

    pub(crate) fn set_day(&mut self, day: u8) {
        (*self).day[0] = day;
    }

    pub(crate) fn set_season(&mut self, season: u8) {
        (*self).season[0] = season;
    }

    pub(crate) fn set_year(&mut self, year: u8) {
        (*self).year[0] = year;
    }

    pub(crate) fn set_slow_mul(&mut self, time_slow_mul: u32) {
        self.slow_time_mul = time_slow_mul;
    }
}
