#[derive(Clone, Copy)]
pub(crate) struct AsmHook {
    pub(crate) target_addr: *mut std::ffi::c_void,
    target_back_addr: *mut std::ffi::c_void,
    detour_fn_addr: *mut std::ffi::c_void,
    is_enable: bool,
}

impl AsmHook {
    pub(crate) unsafe fn set_data(
        &mut self,
        mod_addr: *mut ::core::ffi::c_void,
        mod_data: &[u8],
        pat: &str,
        occupied: usize,
    ) -> &mut Self {
        let pat_offset = vcheat::pat_find(pat, mod_data).unwrap();

        self.target_addr = mod_addr.byte_add(pat_offset);
        self.target_back_addr = self.target_addr.byte_add(occupied);

        self
    }

    pub(crate) unsafe fn gen_detour(
        &mut self,
        detour_fn_addr: *mut ::core::ffi::c_void,
        scan_nop_max_size: usize,
    ) -> &mut Self {
        self.detour_fn_addr = detour_fn_addr;

        let mut detour_fn_end_offset = 0;

        for i in 0..scan_nop_max_size {
            let ptr = detour_fn_addr.cast::<u8>().byte_add(i);

            if ptr.read() == 0x90 {
                let parts = std::slice::from_raw_parts(ptr, 4);

                if parts.iter().all(|nop| *nop == 0x90) {
                    detour_fn_end_offset = i;
                    break;
                }
            }
        }

        let mut jmp_target_addr_shell_code = Vec::new();

        jmp_target_addr_shell_code.push(0xFF);
        jmp_target_addr_shell_code.push(0x25);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);

        jmp_target_addr_shell_code
            .extend_from_slice((self.target_back_addr as isize).to_le_bytes().as_ref());

        vcheat::write_mem(
            vcheat::internal::get_proc_handle(),
            detour_fn_addr.byte_add(detour_fn_end_offset),
            &jmp_target_addr_shell_code,
        )
        .unwrap();

        self
    }

    pub(crate) unsafe fn create_hook(&self) -> &Self {
        minhook_raw::create_hook(
            self.target_addr,
            self.detour_fn_addr,
            ::core::ptr::null_mut(),
        );

        self
    }

    pub(crate) fn get_swtich_mut(&mut self) -> &mut bool {
        &mut self.is_enable
    }

    pub(crate) fn get_swtich(&self) -> bool {
        self.is_enable
    }

    pub(crate) fn enable(&mut self) {
        minhook_raw::enable_hook(self.target_addr);
    }

    pub(crate) fn disable(&mut self) {
        minhook_raw::disable_hook(self.target_addr);
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
