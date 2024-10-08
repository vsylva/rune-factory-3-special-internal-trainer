pub(crate) struct Hook {
    pub(crate) 金币地址: *mut u32,
    pub(crate) 金币旧值: u32,
    pub(crate) 金币max开关: bool,

    pub(crate) 木头地址: *mut u16,
    pub(crate) 木头旧值: u16,
    pub(crate) 木头max开关: bool,

    pub(crate) 自动钓鱼: 汇编Hook,

    pub(crate) 自动按键: 汇编Hook,
    pub(crate) 自动按键标签: u64,

    pub(crate) 穿墙: 汇编Hook,

    pub(crate) 伤害倍率: 汇编Hook,

    pub(crate) 居民友谊倍率: 汇编Hook,

    pub(crate) 作物立即长成: 汇编Hook,

    pub(crate) 技能经验倍率: 汇编Hook,

    pub(crate) 无限委托: 汇编Hook,

    pub(crate) 战斗经验倍率: 汇编Hook,

    pub(crate) 立即驯服: 汇编Hook,
    pub(crate) 无负面状态: 汇编Hook,

    pub(crate) 农田: 汇编Hook,

    pub(crate) 土壤质量开关: bool,
    pub(crate) 土壤质量标签: u64,

    pub(crate) 自动耕作开关: bool,
    pub(crate) 自动耕作标签: u64,

    pub(crate) 自动浇水开关: bool,
    pub(crate) 自动浇水标签: u64,

    pub(crate) 自动种植开关: bool,
    pub(crate) 自动种植标签: u64,

    pub(crate) 作物属性: crate::trainer::作物属性_结构体,

    pub(crate) 时间: 汇编Hook,

    pub(crate) 时间指针: *mut 时间_结构体,
}

pub struct 汇编Hook {
    pub 目标地址: usize,
    pub 开关: bool,
}

#[repr(C)]
pub(crate) struct 时间_结构体 {
    pub(crate) 秒: u8,
    __: [u8; 3],
    pub(crate) 时: u8,
    ___: [u8; 3],
    pub(crate) 天: u8,
    ____: [u8; 3],
    pub(crate) 季节: u8,
    _____: [u8; 3],
    pub(crate) 年: u8,
    ______: [u8; 3],
    _______: [u8; 32],
    pub(crate) 流速: u32,
}

impl Hook {
    const fn new() -> Self {
        Hook {
            金币地址: ::core::ptr::null_mut(),
            金币旧值: 0,
            金币max开关: false,

            木头地址: ::core::ptr::null_mut(),
            木头旧值: 0,
            木头max开关: false,

            自动钓鱼: 汇编Hook::new(),
            自动按键: 汇编Hook::new(),
            自动按键标签: 0,

            穿墙: 汇编Hook::new(),

            伤害倍率: 汇编Hook::new(),

            居民友谊倍率: 汇编Hook::new(),

            作物立即长成: 汇编Hook::new(),

            技能经验倍率: 汇编Hook::new(),

            无限委托: 汇编Hook::new(),

            战斗经验倍率: 汇编Hook::new(),

            立即驯服: 汇编Hook::new(),

            无负面状态: 汇编Hook::new(),

            农田: 汇编Hook::new(),

            土壤质量开关: false,
            土壤质量标签: 0,

            自动耕作开关: false,
            自动耕作标签: 0,

            自动浇水开关: false,
            自动浇水标签: 0,

            自动种植开关: false,
            自动种植标签: 0,

            作物属性: crate::trainer::作物属性_结构体 {
                类型: 0,
                状态: crate::trainer::作物状态_联合体 {
                    生长阶段: 0,
                },
            },

            时间: 汇编Hook::new(),
            时间指针: ::core::ptr::null_mut(),
        }
    }
}

impl 汇编Hook {
    const fn new() -> Self {
        Self {
            目标地址: 0,
            开关: false,
        }
    }

    unsafe fn 创建(
        模块地址: usize,
        模块大小: usize,
        特征码: &str,
        原指令大小: usize,
        跳转地址: *mut ::core::ffi::c_void,
    ) -> ::core::option::Option<Self> {
        let mut hook = Self {
            目标地址: 0,
            开关: false,
        };

        hook.目标地址 = libmem::sig_scan(特征码, 模块地址, 模块大小)?;

        let 原指令的下一指令地址 = hook.目标地址 + 原指令大小;

        let mut 扫描结束的偏移 = 0;

        for i in 0..0xFF {
            let ptr = 跳转地址.cast::<u8>().byte_add(i);

            if ptr.read() == 0x90 {
                let parts = std::slice::from_raw_parts(ptr, 4);

                if parts.iter().all(|nop| *nop == 0x90) {
                    扫描结束的偏移 = i;
                    break;
                }
            }
        }

        let mut 远跳转指令 = Vec::new();

        远跳转指令.push(0xFF);
        远跳转指令.push(0x25);
        远跳转指令.push(0x0);
        远跳转指令.push(0x0);
        远跳转指令.push(0x0);
        远跳转指令.push(0x0);

        远跳转指令.extend_from_slice((原指令的下一指令地址 as isize).to_le_bytes().as_ref());

        libmem::write_memory_ex(
            &libmem::find_process("Rune Factory 3 Special.exe")?,
            跳转地址.byte_add(扫描结束的偏移) as usize,
            远跳转指令.as_slice(),
        )?;

        if hudhook::mh::MH_CreateHook(
            hook.目标地址 as *mut core::ffi::c_void,
            跳转地址,
            ::core::ptr::null_mut(),
        ) != hudhook::mh::MH_STATUS::MH_OK
        {
            return None;
        }

        Some(hook)
    }

    pub(crate) unsafe fn 打开(&mut self) {
        let _ = hudhook::mh::MH_EnableHook(self.目标地址 as *mut core::ffi::c_void);
    }

    pub(crate) unsafe fn 切换开关(&mut self) {
        if self.开关 {
            let _ = hudhook::mh::MH_EnableHook(self.目标地址 as *mut core::ffi::c_void);
        } else {
            let _ = hudhook::mh::MH_DisableHook(self.目标地址 as *mut core::ffi::c_void);
        }
    }
}
pub(crate) static mut HOOK: Hook = Hook::new();

pub(crate) unsafe fn 初始化(mod_addr: usize, mod_size: usize) -> Option<()> {
    HOOK.金币地址 = (crate::SANNDLL信息.base + 0x2AD192C) as *mut u32;
    HOOK.木头地址 = (crate::SANNDLL信息.base + 0x2AD1930) as *mut u16;

    HOOK.自动按键 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "66 F7 D2 66 23 D0",
        6,
        自动按提钓竿的键 as *mut ::core::ffi::c_void,
    )?;

    HOOK.自动钓鱼 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "0F B7 48 18 66 83 F9 03",
        8,
        自动钓鱼 as *mut ::core::ffi::c_void,
    )?;

    HOOK.穿墙 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "48 8B F2 48 85 C9",
        6,
        穿墙 as *mut ::core::ffi::c_void,
    )?;

    HOOK.居民友谊倍率 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "44 8B CA 4D 85 DB",
        6,
        居民友谊倍率 as *mut ::core::ffi::c_void,
    )?;

    HOOK.作物立即长成 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "8B 10 D1 EA 83 E2 7F 74",
        7,
        作物立即长成 as *mut ::core::ffi::c_void,
    )?;

    HOOK.技能经验倍率 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "4C 63 C2 0F B7 CE",
        6,
        技能经验倍率 as *mut ::core::ffi::c_void,
    )?;

    HOOK.农田 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "48 83 C3 08 66 41 3B FF",
        8,
        农田 as *mut ::core::ffi::c_void,
    )?;
    HOOK.农田.打开();

    HOOK.时间 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "03 D0 41 01 51 04",
        6,
        时间 as *mut ::core::ffi::c_void,
    )?;
    HOOK.时间.打开();

    HOOK.无限委托 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "48 8B 5A 08 41 8D 49 FF",
        8,
        无限委托 as *mut ::core::ffi::c_void,
    )?;

    HOOK.战斗经验倍率 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "41 23 CB 41 03 C8",
        6,
        战斗经验倍率 as *mut ::core::ffi::c_void,
    )?;

    HOOK.立即驯服 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "48 C1 E9 20 83 E1 7F 66",
        7,
        立即驯服魔物 as *mut ::core::ffi::c_void,
    )?;

    HOOK.无负面状态 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "BD 00 10 00 00 85 6B 54",
        5,
        无负面状态 as *mut ::core::ffi::c_void,
    )?;

    HOOK.伤害倍率 = 汇编Hook::创建(
        mod_addr,
        mod_size,
        "8B F0 89 44 24 48",
        6,
        伤害倍率 as *mut ::core::ffi::c_void,
    )?;

    Some(())
}

unsafe fn 自动钓鱼() {
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
        in("rax") ::std::ptr::addr_of!(HOOK.自动按键标签),
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        jmp 3f
        2:
        mov word ptr [rax], 0x0
        ",
        in("rax") ::std::ptr::addr_of!(HOOK.自动按键标签),
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

unsafe fn 自动按提钓竿的键() {
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
        in("rax") HOOK.自动按键标签,
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

unsafe fn 穿墙() {
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
        in("rax") std::ptr::addr_of_mut!(crate::SANNDLL信息.base),
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

unsafe fn 居民友谊倍率() {
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

unsafe fn 作物立即长成() {
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

unsafe fn 技能经验倍率() {
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

unsafe fn 农田() {
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
        in("r11") HOOK.土壤质量标签,
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
        in("r12") HOOK.自动浇水标签       ,
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
        in("r13") HOOK.自动耕作标签      ,
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


        in("r14") HOOK.自动种植标签,

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
        in("r15") std::ptr::addr_of!(HOOK.作物属性),
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

unsafe fn 时间() {
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
        in("r15") ::std::ptr::addr_of!(HOOK.时间指针),
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

unsafe fn 无限委托() {
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

unsafe fn 战斗经验倍率() {
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

unsafe fn 立即驯服魔物() {
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

unsafe fn 无负面状态() {
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

unsafe fn 伤害倍率() {
    std::arch::asm!(
        "
        push r10
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        mov r10d, 0x64

        imul eax, r10d
        mov esi, eax
        mov [rsp + 0x48], eax
        ",
        options(nomem, nostack)
    );

    std::arch::asm!(
        "
        pop r10
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
