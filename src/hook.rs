#[derive(Clone, Copy)]
pub(crate) struct Hook {
    pub(crate) target_addr: *mut std::ffi::c_void,
    pub(crate) target_back_addr: *mut std::ffi::c_void,
    pub(crate) detour_fn_addr: *mut std::ffi::c_void,
    pub(crate) is_enable: bool,
}

impl Hook {
    pub(crate) unsafe fn new() -> Self {
        ::core::mem::zeroed()
    }

    pub(crate) unsafe fn build_target(
        &mut self,
        mod_addr: *mut ::core::ffi::c_void,
        mod_data: &[u8],
        pat: &str,
        occupied: usize,
    ) -> &mut Self {
        let pat_offset = vcheat::pat_find(pat, mod_data).unwrap();

        self.target_addr = mod_addr.add(pat_offset);
        self.target_back_addr = self.target_addr.add(occupied);

        self
    }

    pub(crate) unsafe fn build_detour(
        &mut self,
        detour_fn_addr: *mut ::core::ffi::c_void,
        scan_nop_max_size: usize,
    ) -> &mut Self {
        self.detour_fn_addr = detour_fn_addr;

        let mut detour_fn_end_offset = 0;

        let mut mask = 0;

        for i in 0..scan_nop_max_size {
            if detour_fn_addr.add(i).cast::<u8>().read() == 0x90 {
                mask += 1;
                if mask == 4 {
                    detour_fn_end_offset = i - 4;
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
            detour_fn_addr.add(detour_fn_end_offset + 1),
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

    pub(crate) fn enable(&mut self) {
        minhook_raw::enable_hook(self.target_addr);
        self.is_enable = true;
    }

    pub(crate) fn disable(&mut self) {
        minhook_raw::disable_hook(self.target_addr);

        self.is_enable = false;
    }
}

pub(crate) unsafe fn hooks(mod_addr: *mut ::core::ffi::c_void, mod_data: &[u8]) {
    crate::var::auto_press::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::var::auto_press::PAT, 6)
        .build_detour(crate::asm::auto_press as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::var::fishing::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::var::fishing::PAT, 8)
        .build_detour(crate::asm::fishing as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::var::walk_through_walls::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::var::walk_through_walls::PAT, 6)
        .build_detour(
            crate::asm::walk_through_walls as *mut ::core::ffi::c_void,
            64,
        )
        .create_hook()
        .to_owned();

    crate::var::friendship_mul::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::var::friendship_mul::PAT, 6)
        .build_detour(crate::asm::friendship_mul as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::var::instant_crop_growth::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::var::instant_crop_growth::PAT, 7)
        .build_detour(
            crate::asm::instant_crop_growth as *mut ::core::ffi::c_void,
            64,
        )
        .create_hook()
        .to_owned();

    crate::var::skill_exp_mul::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::var::skill_exp_mul::PAT, 6)
        .build_detour(crate::asm::skill_exp_mul as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::var::farm::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::var::farm::PAT, 8)
        .build_detour(crate::asm::farm as *mut ::core::ffi::c_void, 0xFF)
        .create_hook()
        .to_owned();
}
