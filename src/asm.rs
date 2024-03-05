static mut SANDLL_ADDR_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(crate::var::SANDLL_ADDR) } as *const i64;

static mut AUTO_PRESS_B_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(crate::var::auto_press::AUTO_PRESS_MARK) } as *const i64;

static mut SOIL_QUALITY_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(crate::var::farm::soil_quality::MARK) } as *const i64;

static mut WATERING_PLOTS_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(crate::var::farm::watering_plots::MARK) } as *const i64;

static mut TILTT_PLOTS_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(crate::var::farm::tilth_plots::MARK) } as *const i64;

static mut PLANT_PLOTS_MARK_POINTER: *const i64 =
    unsafe { std::ptr::addr_of!(crate::var::farm::plant_plots::MARK) } as *const i64;

static mut CROP_PROP_POINTER: *const crate::var::CropProp =
    unsafe { std::ptr::addr_of!(crate::var::farm::plant_plots::CROP_PROP) }
        as *const crate::var::CropProp;

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn fishing() {
    std::arch::asm!(
        "movzx ecx,word ptr [rax+0x18]",
        "cmp cx, 05",
        options(nomem, nostack)
    );

    std::arch::asm!("push rax", options(nomem, nostack));

    std::arch::asm!(
    "je 0f",
    "mov dword ptr [rax],1",
    "jmp 1f",
    "0:",
    "mov dword ptr [rax],0",
    "1:",
    "cmp cx,03",
    in("rax") AUTO_PRESS_B_MARK_POINTER,
    options(nomem,nostack)
    );

    std::arch::asm!("pop rax", options(nomem, nostack));

    std::arch::asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack)
    );
}

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn auto_press() {
    std::arch::asm!("not dx", "and dx,ax", options(nomem, nostack));

    std::arch::asm!("push rax", options(nomem, nostack));

    std::arch::asm!(
            "cmp dword ptr [rax],1",
            "jne 0f",
            "mov dx,2",
            "0:",
            in("rax") AUTO_PRESS_B_MARK_POINTER,
            options(nomem,nostack)
    );

    std::arch::asm!("pop rax", options(nomem, nostack));

    std::arch::asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack)
    );
}

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn walk_through_walls() {
    std::arch::asm!(" mov rsi,rdx", options(nomem, nostack));

    std::arch::asm!("push rax", options(nomem, nostack));
    std::arch::asm!("push rcx", options(nomem, nostack));

    std::arch::asm!(
        "
        mov rax, [rax]
        mov rcx,[rax + 0x311F78]
        lea rax, [rcx]
        cmp rbx, rax
        ",
        in("rax") SANDLL_ADDR_POINTER,
        options(nomem, nostack)
    );

    std::arch::asm!("pop rcx", options(nomem, nostack));
    std::arch::asm!("pop rax", options(nomem, nostack));

    std::arch::asm!(
        "
        je 0f
        test rcx,rcx
        0:
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack)
    );
}

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn friendship_mul() {
    std::arch::asm!("push rax", options(nomem, nostack));

    std::arch::asm!(
        "
        mov eax,128
        mov r9d,edx
        imul r9d, eax
        test r11,r11
        ",
        options(nomem, nostack)
    );

    std::arch::asm!("pop rax", options(nomem, nostack));

    std::arch::asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack)
    );
}

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn instant_crop_growth() {
    std::arch::asm!(
        "
        mov edx, [rax]
        or edx, 0x00005000
        and edx, 0xFFFFDFFF
        mov [rax], edx
        shr edx, 0x1
        and edx, 0x7F
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack)
    );
}

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn skill_exp_mul() {
    std::arch::asm!("push rax", options(nomem, nostack));

    std::arch::asm!(
        "
        mov rax,128
        imul rdx, rax
        movsxd  r8,edx
        movzx ecx,si
        ",
        options(nomem, nostack)
    );

    std::arch::asm!("pop rax", options(nomem, nostack));

    std::arch::asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack)
    );
}

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn farm() {
    std::arch::asm!("push rax", options(nomem, nostack));
    std::arch::asm!("push r11", options(nomem, nostack));

    std::arch::asm!(
        "
        cmp dword ptr [r11],0
        je 0f
          mov rax, 0x00
          mov eax, dword ptr [rbx+0x04]
          or eax, 0x0038003E
          xor eax, 0x0038000E
          mov [rbx+04], eax

        0:
        cmp dword ptr [r12],0
        je 1f
          mov rax, 0x00
          mov al, byte ptr[rbx + 0x03]
          or al, 0x10
          and al, 0xFB
          mov byte ptr [rbx + 0x03], al

        1:
        cmp dword ptr [r13],0
        je 2f
          mov rax, 0x00
          mov al, byte ptr [rbx + 0x03]
          and al, 0x08
          cmp al, 0x08
        je 2f
          mov al, byte ptr[rbx + 0x03]
          or al, 0x08
          mov byte ptr [rbx+0x03], al
          mov byte ptr [rbx], 0x00

        2:
        cmp dword ptr [r14],0
        je 3f
          mov rax, 0x00
          mov al, byte ptr [rbx + 0x03]
          and al, 0x08
          cmp al, 0x08
        jne 3f
          mov rax, 0x00
          mov ax, word ptr [r15]
          mov word ptr [rbx + 00], ax

        3:
        ",

        // SoilQuality = 0
        // WateringPlots = 1
        // Tilt = 2
        // Plant = 3

        in("r11") SOIL_QUALITY_MARK_POINTER,
        in("r12") WATERING_PLOTS_MARK_POINTER,
        in("r13") TILTT_PLOTS_MARK_POINTER,
        in("r14") PLANT_PLOTS_MARK_POINTER,
        in("r15") CROP_PROP_POINTER,
        options(nostack,nomem)
    );

    std::arch::asm!("pop r11", options(nomem, nostack));
    std::arch::asm!("pop rax", options(nomem, nostack));
    std::arch::asm!("pop r12", options(nomem, nostack));
    std::arch::asm!("pop r13", options(nomem, nostack));
    std::arch::asm!("pop r14", options(nomem, nostack));
    std::arch::asm!("pop r15", options(nomem, nostack));

    std::arch::asm!(
        "
        add rbx, 0x8
        cmp di, r15w
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
        options(nomem, nostack, noreturn)
    );
}
