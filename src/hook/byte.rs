#[allow(unused)]
pub(crate) struct Hook {
    target_addr: *mut ::core::ffi::c_void,
    source: Vec<u8>,
    patch: Vec<u8>,
    offset: usize,
    is_enable: bool,
}

#[allow(unused)]
impl Hook {
    pub(crate) const fn new() -> Self {
        Self {
            target_addr: ::core::ptr::null_mut(),
            source: Vec::new(),
            patch: Vec::new(),
            offset: 0,
            is_enable: false,
        }
    }

    pub(crate) unsafe fn get_data(
        &mut self,
        mod_addr: *mut ::core::ffi::c_void,
        mod_data: &[u8],
        pat: &str,
        source: Vec<u8>,
        patch: Vec<u8>,
        offset: usize,
    ) -> &mut Self {
        let pat_offset = vcheat::pat_find(pat, mod_data).unwrap();

        self.target_addr = mod_addr.add(pat_offset);
        self.source = source;
        self.offset = offset;
        self.patch = patch;
        self
    }

    pub(crate) unsafe fn enable(&mut self) {
        let mbi = vcheat::internal::query_mem(self.target_addr).unwrap();

        let readable = mbi.state != vcheat::types::mem_alloc::COMMIT
            || mbi.protect == vcheat::types::mem_protect::NOACCESS;

        let mut prev_protect = 0;

        if !readable {
            prev_protect = vcheat::internal::protect_mem(
                self.target_addr,
                0x1000,
                vcheat::types::mem_protect::EXECUTE_READ_WRITE,
            )
            .unwrap();
        }

        vcheat::write_mem_t(
            vcheat::internal::get_proc_handle(),
            self.target_addr,
            self.patch.as_ptr().add(self.offset),
            self.patch.len(),
        )
        .unwrap();

        if !readable {
            vcheat::internal::protect_mem(self.target_addr, 0x1000, prev_protect).unwrap();
        }
    }

    pub(crate) unsafe fn disable(&mut self) {
        let mbi = vcheat::internal::query_mem(self.target_addr).unwrap();

        let readable = mbi.state != vcheat::types::mem_alloc::COMMIT
            || mbi.protect == vcheat::types::mem_protect::NOACCESS;

        let mut prev_protect = 0;

        if !readable {
            prev_protect = vcheat::internal::protect_mem(
                self.target_addr,
                0x1000,
                vcheat::types::mem_protect::EXECUTE_READ_WRITE,
            )
            .unwrap();
        }

        vcheat::write_mem_t(
            vcheat::internal::get_proc_handle(),
            self.target_addr,
            self.source.as_ptr(),
            self.source.len(),
        )
        .unwrap();

        if !readable {
            vcheat::internal::protect_mem(self.target_addr, 0x1000, prev_protect).unwrap();
        }
    }

    pub(crate) fn get_swtich_mut(&mut self) -> &mut bool {
        &mut self.is_enable
    }

    pub(crate) unsafe fn swtich(&mut self) {
        if self.is_enable {
            self.enable()
        } else {
            self.disable();
        }
    }
}
