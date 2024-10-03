pub(crate) unsafe fn save_load() {
    std::arch::asm!(
        "
    shl ax, 0x5
    movzx r9d, dl
    ",
        options(nomem, nostack)
    );

    std::arch::asm!("push rax", options(nomem, nostack));

    std::arch::asm!(
    "
    mov word ptr[rax], 0x1
    ",
    in("rax") ::std::ptr::addr_of!(crate::hook::SAVE_LOAD_MARK),
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

pub(crate) unsafe fn auto_fishing() {
    std::arch::asm!(
        "
        movzx ecx, word ptr [rax + 0x18]
        cmp cx, 0x5
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        push rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        je 2f
        mov word ptr [rax], 0x1
        ",
        in("rax") ::std::ptr::addr_of!(crate::hook::AUTO_PRESS_MARK),
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        jmp 3f
        2:
        mov word ptr [rax], 0x0
        ",
        in("rax") ::std::ptr::addr_of!(crate::hook::AUTO_PRESS_MARK),
        options(nomem, nostack));

    std::arch::asm!(
        "
        3:
        cmp cx, 0x3
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        pop rax
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

pub(crate) unsafe fn auto_press() {
    std::arch::asm!(
        "
        not dx
        and dx, ax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        push rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        cmp rax, 0x1
        ",
        in("rax") crate::hook::AUTO_PRESS_MARK,
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        jne 2f
        mov dx, 0x2

        2:
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        pop rax
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

pub(crate) unsafe fn walk_through_walls() {
    std::arch::asm!(
        "
        mov rsi, rdx
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        push rax
        push rcx
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        mov rax, [rax]
        mov rcx, [rax + 0x311F78]
        lea rax, [rcx]
        cmp rbx, rax
        ",
        in("rax") std::ptr::addr_of_mut!(crate::MOD_SANDLL.base),
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        pop rcx
        pop rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        je 2f
        test rcx, rcx
        2:
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

pub(crate) unsafe fn friendship_mul() {
    std::arch::asm!(
        "
        push rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        mov eax, 0x64
        mov r9d, edx
        imul r9d, eax
        test r11, r11
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        pop rax
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

pub(crate) unsafe fn crop_instant_growth() {
    std::arch::asm!(
        "
        mov edx, [rax]
        or edx, 0x5000
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

pub(crate) unsafe fn skill_exp_mul() {
    std::arch::asm!(
        "
        push rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        mov rax, 0x64
        imul rdx, rax
        movsxd  r8, edx
        movzx ecx, si
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        pop rax
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

pub(crate) unsafe fn farm() {
    std::arch::asm!(
        "
        push rax
        push r11
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        cmp r11,0
        je 2f

        mov rax, 0x0
        mov eax, dword ptr [rbx + 0x4]
        or eax, 0x38003E
        xor eax, 0x38000E
        mov [rbx + 0x4], eax

        2:
        ",
        in("r11") crate::hook::SOIL_QUALITY_MARK,
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        cmp r12, 0x0
        je 3f

        mov rax, 0x0
        mov al, byte ptr[rbx + 0x3]
        or al, 0x10
        and al, 0xFB
        mov byte ptr [rbx + 0x3], al

        3:
        ",
        in("r12") crate::hook::WATERING_PLOTS_MARK       ,
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        cmp r13, 0x0
        je 4f

        mov rax, 0x0
        mov al, byte ptr [rbx + 0x3]
        and al, 0x8
        cmp al, 0x8
        je 4f

        mov al, byte ptr[rbx + 0x3]
        or al, 0x8
        mov byte ptr [rbx + 0x3], al
        mov byte ptr [rbx], 0x0

        4:
        ",
        in("r13") crate::hook::TILTH_PLOTS_MARK      ,
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        cmp r14, 0x0
        je 5f

        mov rax, 0x0
        mov al, byte ptr [rbx + 0x3]
        and al, 0x8
        cmp al, 0x8
       ",


        in("r14") crate::hook::PLANT_PLOTS_MARK,

        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        jne 5f

        mov rax, 0x0
        mov ax, word ptr [r15]
        mov word ptr [rbx + 0x0], ax

        5:
        ",
        in("r15") std::ptr::addr_of!(crate::hook::CROP_PROP),
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        pop r11
        pop rax
        pop r12
        pop r13
        pop r14
        pop r15
        ",
        options(nomem, nostack)
    );

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
        options(nomem, nostack)
    );
}

pub(crate) unsafe fn time() {
    std::arch::asm!(
        "
        push rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        lea rax, [r9 + 0x4]
        mov [r15], rax
        ",
        in("r15") ::std::ptr::addr_of!(crate::hook::TIME_POINTER),
        options(nostack)
    );

    std::arch::asm!(
        "
        pop rax
        pop r15
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        add edx, eax
        add [r9 + 0x4], edx
        ",
        options(nostack)
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

pub(crate) unsafe fn inf_mission() {
    std::arch::asm!(
        "
        mov rbx, [rdx + 0x8]
        lea ecx, [r9 - 0x1]

        push rax

        mov rax, 0x0
        mov eax, [rbx + 0x4]
        or eax, 0x7000000
        xor eax, 0x7000000
        mov [rbx + 0x4], eax

        pop rax
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

pub(crate) unsafe fn combat_exp_mul() {
    std::arch::asm!(
        "
        push rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        mov eax, 0x64

        and ecx, r11d
        imul r8d, eax
        add ecx, r8d
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        pop rax
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

pub(crate) unsafe fn tame() {
    std::arch::asm!(
        "
        shr rcx, 0x20
        and ecx, 0x7F
        mov r8w, cx
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

pub(crate) unsafe fn no_debuff() {
    std::arch::asm!(
        "
        push rax
        mov rax, 0x0
        mov ax, [rbx + 0x55]
        and ax, 0xFC0F
        mov [rbx + 0x55],ax
        pop rax
        mov ebp, 0x1000
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

//
// pub(crate) unsafe  fn damage_mul() {
//     std::arch::asm!(
//         "
//         push r10
//         ",
//         options(nomem, nostack)
//     );

//     std::arch::asm!(
//         "
//         mov r10d, 0x64

//         imul eax, r10d
//         mov esi, eax
//         mov [rsp + 0x48], eax
//         ",
//         options(nomem, nostack)
//     );

//     std::arch::asm!(
//         "
//         pop r10
//         ",
//         options(nomem, nostack)
//     );

//     std::arch::asm!(
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         "nop",
//         options(nomem, nostack)
//     );
// }
