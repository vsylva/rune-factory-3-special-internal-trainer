#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn load_save() {
    std::arch::asm!(
        "
    shl ax,05
    movzx r9d,dl
    ",
        options(nomem, nostack)
    );

    std::arch::asm!("push rax", options(nomem, nostack));

    std::arch::asm!(
    "
    mov rax, 0x1
    ",
    out("rax") crate::hook::SAVE_LOAD_MARK,
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
pub(crate) unsafe extern "system" fn fishing() {
    std::arch::asm!(
        "
        movzx ecx,word ptr [rax+0x18]
        cmp cx, 05
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
        je 0f
        mov rax, 1
        ",
        out("rax") crate::hook::AUTO_PRESS_MARK,
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        jmp 1f
        0:
        mov rax,0
        ",
        out("rax") crate::hook::AUTO_PRESS_MARK,
        options(nomem, nostack));

    std::arch::asm!(
        "
        1:
        cmp cx,03
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

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn auto_press() {
    std::arch::asm!(
        "
        not dx
        and dx,ax
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
        cmp rax, 1
        ",
        in("rax") crate::hook::AUTO_PRESS_MARK,
        options(nomem,nostack)
    );

    std::arch::asm!(
        "
        jne 0f
        mov dx,2
        0:
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

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn walk_through_walls() {
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
        in("rax") crate::SANDLL_ADDR_POINTER,
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
    std::arch::asm!(
        "
        push rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        mov eax,128
        mov r9d,edx
        imul r9d, eax
        test r11,r11
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

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn crop_instant_growth() {
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
    std::arch::asm!(
        "
        push rax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        mov rax,128
        imul rdx, rax
        movsxd  r8,edx
        movzx ecx,si
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

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn farm() {
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
        je 0f

        mov rax, 0x00
        mov eax, dword ptr [rbx+0x04]
        or eax, 0x0038003E
        xor eax, 0x0038000E
        mov [rbx+04], eax

        0:
        ",
        in("r11") crate::hook::SOIL_QUALITY_MARK,
        options(nostack,nomem)
    );

    std::arch::asm!(
        "
        cmp r12,0
        je 1f

        mov rax, 0x00
        mov al, byte ptr[rbx + 0x03]
        or al, 0x10
        and al, 0xFB
        mov byte ptr [rbx + 0x03], al

        1:
        ",
        in("r12") crate::hook::WATERING_PLOTS_MARK       ,
        options(nostack,nomem)
    );

    std::arch::asm!(
        "
        cmp r13,  0
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
        ",
        in("r13") crate::hook::TILTH_PLOTS_MARK      ,
        options(nostack,nomem)
    );

    std::arch::asm!(
        "
        cmp r14,0
        je 3f

        mov rax, 0x00
        mov al, byte ptr [rbx + 0x03]
        and al, 0x08
        cmp al, 0x08
       ",


        in("r14") crate::hook::PLANT_PLOTS_MARK,

        options(nostack,nomem)
    );

    std::arch::asm!(
        "
        jne 3f

        mov rax, 0x00
        mov ax, word ptr [r15]
        mov word ptr [rbx + 00], ax

        3:
        ",
        in("r15") crate::hook::CROP_PROP_POINTER,
        options(nostack,nomem)
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
        options(nomem, nostack, noreturn)
    );
}

#[allow(unused)]
#[inline(never)]
pub(crate) unsafe extern "system" fn time(){
    std::arch::asm!(
        "
        push rax
        ",
        options(nostack, nomem)
    );

    std::arch::asm!(
        "
        lea rax, [r9+04]
        mov [r15], rax
        ",
        in("r15") crate::hook::TIME_POINTER_POINTER,
        options(nomem,nostack)
    );

    std::arch::asm!(
        "
        pop rax
        pop r15
        ",
        options(nostack, nomem)
    );

    std::arch::asm!(
        "
        add edx,eax
        add [r9+04],edx
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
