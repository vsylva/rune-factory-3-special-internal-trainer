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
