pub(crate) struct 修改器 {
    pub(crate) 显示界面: bool,

    pub(crate) 选择的作物: 作物类型,
    pub(crate) 作物类型列表: &'static [作物类型],
    pub(crate) 选择的作物等级: 作物等级,
    pub(crate) 作物等级列表: &'static [作物等级],
    pub(crate) 选择的作物生长阶段: 作物生长阶段,
    pub(crate) 作物生长阶段列表: &'static [作物生长阶段],

    pub(crate) 选择的秒: u8,
    pub(crate) 秒列表: Vec<u8>,
    pub(crate) 选择的时: u8,
    pub(crate) 时列表: Vec<u8>,
    pub(crate) 选择的天: u8,
    pub(crate) 天列表: Vec<u8>,
    pub(crate) 选择的季节: 季节,
    pub(crate) 季节列表: &'static [季节],
    pub(crate) 选择的年: u8,
    pub(crate) 年列表: Vec<u8>,
    pub(crate) 选择的流速: 时间流速,
    pub(crate) 时间流速列表: &'static [时间流速],
}

#[repr(C)]
pub(crate) struct 作物属性_结构体 {
    pub(crate) 类型: u8,
    pub(crate) 状态: 作物状态_联合体,
}

#[repr(C)]
pub(crate) union 作物状态_联合体 {
    pub(crate) 生长阶段: u8,
    pub(crate) 等级: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub(crate) enum 季节 {
    春 = 0,
    夏 = 1,
    秋 = 2,
    冬 = 3,
}

impl ::core::fmt::Display for 季节 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::春 => write!(f, "春"),
            Self::夏 => write!(f, "夏"),
            Self::秋 => write!(f, "秋"),
            Self::冬 => write!(f, "冬"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum 时间流速 {
    暂停时间 = 0,
    百分之一 = 0x3D,
    十分之一 = 0x266,
    四分之一 = 0x600,
    二分之一 = 0xC00,
    默认 = 0x1800,
    一点五 = 0x2400,
    两点零 = 0x3000,
}

impl ::core::fmt::Display for 时间流速 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::暂停时间 => write!(f, "暂停"),
            Self::百分之一 => write!(f, "0.01倍"),
            Self::十分之一 => write!(f, "0.1倍"),
            Self::四分之一 => write!(f, "0.25倍"),
            Self::二分之一 => write!(f, "0.5倍"),
            Self::默认 => write!(f, "默认"),
            Self::一点五 => write!(f, "1.5倍"),
            Self::两点零 => write!(f, "2.0倍"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum 作物类型 {
    无 = 0,

    石头 = 1, // 可捡
    岩石 = 2, // 可砸
    树枝 = 3, // 可捡
    树桩 = 4, // 可劈
    木材 = 5, // 可砸，什么都不会出
    毒沼 = 6, // 可砸，什么都不会出

    // 矿石 = 7, // 砸会闪退
    药草 = 8,    // 可捡
    解毒草 = 9,  // 可捡
    黑草 = 10,   // 可捡
    枯草 = 11,   // 可捡
    黄草 = 14,   // 可捡
    苦橙草 = 15, // 可捡

    // 种子 = 16, // 不可捡，名字就叫 “种子
    杂草 = 17,   // 可捡
    季节岩 = 18, // 可砸
    花卉 = 19,   // 可摧毁

    水晶 = 20, // 可砸，出的不知道是不是buff

    // 苹果 = 21, //  可砸，什么都不会出
    // 苹果 = 22    同上
    // 苹果 = 23    同上
    草莓 = 24,
    卷心菜 = 25,
    樱芜菁 = 26,
    洋葱 = 27,
    托伊药草 = 28,
    月落草 = 29,
    樱草 = 30,
    灯草 = 31,
    金刚花 = 32,
    青水晶 = 33,
    金卷心菜 = 34,
    少女蜜瓜 = 35,

    竹笋 = 36, // 可割

    南瓜 = 37,
    黄瓜 = 38,
    玉米 = 39,
    番茄 = 40,
    茄子 = 41,
    菠萝 = 42,
    粉红猫 = 43,
    铁千轮 = 44,
    四叶草 = 45,
    原之焰火 = 46,
    绿水晶 = 47,
    金南瓜 = 48,

    蓝草 = 49, // 可捡
    绿草 = 50, // 可捡
    紫草 = 51, // 可捡
    靛草 = 52, // 可捡

    番薯 = 53,
    马铃薯 = 54,
    胡萝卜 = 55,
    青椒 = 56,
    菠菜 = 57,
    魅蓝草 = 58,
    红叶花 = 59,
    剧毒蒲公英 = 60,
    红水晶 = 61,
    金马铃薯 = 62,
    芜菁 = 63,
    白萝卜 = 64,
    葱 = 65,
    白菜 = 66,
    树形草 = 67,
    白水晶 = 68,
    金芜青 = 69,
    火热果实 = 70,

    白草 = 71, /* 可捡
                * 从72开始的编号都是无效的东西 */
}

impl ::core::fmt::Display for 作物类型 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::无 => write!(f, "无"),
            Self::石头 => write!(f, "石头"),
            Self::岩石 => write!(f, "岩石"),
            Self::树枝 => write!(f, "树枝"),
            Self::树桩 => write!(f, "树桩"),
            Self::木材 => write!(f, "木材"),
            Self::毒沼 => write!(f, "毒沼"),
            Self::药草 => write!(f, "药草"),
            Self::解毒草 => write!(f, "解毒草"),
            Self::黑草 => write!(f, "黑草"),
            Self::枯草 => write!(f, "枯草"),
            Self::黄草 => write!(f, "黄草"),
            Self::苦橙草 => write!(f, "苦橙草"),
            Self::杂草 => write!(f, "杂草"),
            Self::季节岩 => write!(f, "季节岩"),
            Self::花卉 => write!(f, "花卉"),
            Self::水晶 => write!(f, "水晶"),
            Self::草莓 => write!(f, "草莓"),
            Self::卷心菜 => write!(f, "卷心菜"),
            Self::樱芜菁 => write!(f, "樱芜菁"),
            Self::洋葱 => write!(f, "洋葱"),
            Self::托伊药草 => write!(f, "托伊药草"),
            Self::月落草 => write!(f, "月落草"),
            Self::樱草 => write!(f, "樱草"),
            Self::灯草 => write!(f, "灯草"),
            Self::金刚花 => write!(f, "金刚花"),
            Self::青水晶 => write!(f, "青水晶"),
            Self::金卷心菜 => write!(f, "金卷心菜"),
            Self::少女蜜瓜 => write!(f, "少女蜜瓜"),
            Self::竹笋 => write!(f, "竹笋"),
            Self::南瓜 => write!(f, "南瓜"),
            Self::黄瓜 => write!(f, "黄瓜"),
            Self::玉米 => write!(f, "玉米"),
            Self::番茄 => write!(f, "番茄"),
            Self::茄子 => write!(f, "茄子"),
            Self::菠萝 => write!(f, "菠萝"),
            Self::粉红猫 => write!(f, "粉红猫"),
            Self::铁千轮 => write!(f, "铁千轮"),
            Self::四叶草 => write!(f, "四叶草"),
            Self::原之焰火 => write!(f, "原之焰火"),
            Self::绿水晶 => write!(f, "绿水晶"),
            Self::金南瓜 => write!(f, "金南瓜"),
            Self::蓝草 => write!(f, "蓝草"),
            Self::绿草 => write!(f, "绿草"),
            Self::紫草 => write!(f, "紫草"),
            Self::靛草 => write!(f, "靛草"),
            Self::番薯 => write!(f, "番薯"),
            Self::马铃薯 => write!(f, "马铃薯"),
            Self::胡萝卜 => write!(f, "胡萝卜"),
            Self::青椒 => write!(f, "青椒"),
            Self::菠菜 => write!(f, "菠菜"),
            Self::魅蓝草 => write!(f, "魅蓝草"),
            Self::红叶花 => write!(f, "红叶花"),
            Self::剧毒蒲公英 => write!(f, "暂停"),
            Self::红水晶 => write!(f, "红水晶"),
            Self::金马铃薯 => write!(f, "金马铃薯"),
            Self::芜菁 => write!(f, "芜菁"),
            Self::白萝卜 => write!(f, "白萝卜"),
            Self::葱 => write!(f, "葱"),
            Self::白菜 => write!(f, "白菜"),
            Self::树形草 => write!(f, "树形草"),
            Self::白水晶 => write!(f, "白水晶"),
            Self::金芜青 => write!(f, "金芜青"),
            Self::火热果实 => write!(f, "火热果实"),
            Self::白草 => write!(f, "白草"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum 作物等级 {
    LV1 = 0,
    LV2 = 1,
    LV3 = 2,
    LV4 = 3,
    LV5 = 4,
    LV6 = 5,
    LV7 = 6,
    LV8 = 7,
    LV9 = 8,
    LV10 = 9,
}

impl ::core::fmt::Display for 作物等级 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LV1 => write!(f, "LV1"),
            Self::LV2 => write!(f, "LV2"),
            Self::LV3 => write!(f, "LV3"),
            Self::LV4 => write!(f, "LV4"),
            Self::LV5 => write!(f, "LV5"),
            Self::LV6 => write!(f, "LV6"),
            Self::LV7 => write!(f, "LV7"),
            Self::LV8 => write!(f, "LV8"),
            Self::LV9 => write!(f, "LV9"),
            Self::LV10 => write!(f, "LV10"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum 作物生长阶段 {
    // 无 = 0,
    一阶段 = 0x1,
    二阶段 = 0x2,
    三阶段 = 0x3,
    四阶段 = 0x4,
    五阶段 = 0x5,
}

impl ::core::fmt::Display for 作物生长阶段 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::一阶段 => write!(f, "一阶段"),
            Self::二阶段 => write!(f, "二阶段"),
            Self::三阶段 => write!(f, "三阶段"),
            Self::四阶段 => write!(f, "四阶段"),
            Self::五阶段 => write!(f, "五阶段"),
        }
    }
}

impl hudhook::ImguiRenderLoop for 修改器 {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        unsafe {
            self.initialize(_ctx, _render_context);
        }
    }

    fn before_render<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        unsafe { self.before_render(_ctx, _render_context) };
    }

    fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        unsafe { self.render(ui) };
    }
}

impl 修改器 {
    unsafe fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        crate::hook::初始化(crate::SANNDLL信息.base, crate::SANNDLL信息.size);

        _ctx.set_ini_filename(None);

        _ctx.style_mut().use_light_colors();

        hudhook::imgui::sys::ImFontAtlas_AddFontFromFileTTF(
            hudhook::imgui::internal::RawCast::raw_mut(_ctx.fonts()),
            "C:\\windows\\fonts\\simhei.ttf\0".as_ptr().cast(),
            20.0,
            ::std::ptr::null(),
            hudhook::imgui::sys::ImFontAtlas_GetGlyphRangesChineseFull(
                hudhook::imgui::internal::RawCast::raw_mut(_ctx.fonts()),
            ),
        );

        self.作物类型列表 = &[
            作物类型::无,
            作物类型::石头,
            作物类型::岩石,
            作物类型::树枝,
            作物类型::树桩,
            作物类型::木材,
            作物类型::毒沼,
            作物类型::药草,
            作物类型::解毒草,
            作物类型::黑草,
            作物类型::枯草,
            作物类型::黄草,
            作物类型::苦橙草,
            作物类型::杂草,
            作物类型::季节岩,
            作物类型::花卉,
            作物类型::水晶,
            作物类型::草莓,
            作物类型::卷心菜,
            作物类型::樱芜菁,
            作物类型::洋葱,
            作物类型::托伊药草,
            作物类型::月落草,
            作物类型::樱草,
            作物类型::灯草,
            作物类型::金刚花,
            作物类型::青水晶,
            作物类型::金卷心菜,
            作物类型::少女蜜瓜,
            作物类型::竹笋,
            作物类型::南瓜,
            作物类型::黄瓜,
            作物类型::玉米,
            作物类型::番茄,
            作物类型::茄子,
            作物类型::菠萝,
            作物类型::粉红猫,
            作物类型::铁千轮,
            作物类型::四叶草,
            作物类型::原之焰火,
            作物类型::绿水晶,
            作物类型::金南瓜,
            作物类型::蓝草,
            作物类型::绿草,
            作物类型::紫草,
            作物类型::靛草,
            作物类型::番薯,
            作物类型::马铃薯,
            作物类型::胡萝卜,
            作物类型::青椒,
            作物类型::菠菜,
            作物类型::魅蓝草,
            作物类型::红叶花,
            作物类型::剧毒蒲公英,
            作物类型::红水晶,
            作物类型::金马铃薯,
            作物类型::芜菁,
            作物类型::白萝卜,
            作物类型::葱,
            作物类型::白菜,
            作物类型::树形草,
            作物类型::白水晶,
            作物类型::金芜青,
            作物类型::火热果实,
            作物类型::白草,
        ];

        self.作物等级列表 = &[
            作物等级::LV1,
            作物等级::LV2,
            作物等级::LV3,
            作物等级::LV4,
            作物等级::LV5,
            作物等级::LV6,
            作物等级::LV7,
            作物等级::LV8,
            作物等级::LV9,
            作物等级::LV10,
        ];

        self.作物生长阶段列表 = &[
            作物生长阶段::一阶段,
            作物生长阶段::二阶段,
            作物生长阶段::三阶段,
            作物生长阶段::四阶段,
            作物生长阶段::五阶段,
        ];

        self.季节列表 = &[季节::春, 季节::夏, 季节::秋, 季节::冬];

        self.时间流速列表 = &[
            时间流速::暂停时间,
            时间流速::百分之一,
            时间流速::十分之一,
            时间流速::四分之一,
            时间流速::二分之一,
            时间流速::默认,
            时间流速::一点五,
            时间流速::两点零,
        ];

        for 秒 in 0..60 {
            self.秒列表.push(秒);
        }

        for 时 in 0..24 {
            self.时列表.push(时);
        }

        for 天 in 1..31 {
            self.天列表.push(天);
        }

        for 年 in 1..100 {
            self.年列表.push(年);
        }
    }

    unsafe fn before_render<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        if 按键被按下一次(0xC0) {
            self.显示界面 = !self.显示界面;
        }

        if !self.显示界面 {
            _ctx.io_mut().mouse_draw_cursor = false;

            return;
        }

        _ctx.io_mut().mouse_draw_cursor = true;
    }

    unsafe fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        if !self.显示界面 {
            return;
        }

        ui.window("[~]键")
            .title_bar(true)
            .size([600.0, 450.0], hudhook::imgui::Condition::FirstUseEver)
            .build(|| {
                if (*crate::hook::HOOK_MUT).时间指针.is_null() {
                    ui.text_colored([1.0, 0.0, 0.0, 1.0], "等待开始游戏......");

                    return;
                };

                self.每帧渲染(ui)
            });
    }

    pub(crate) unsafe fn 每帧渲染(&mut self, ui: &hudhook::imgui::Ui) {
        let Some(tab_bar) = ui.tab_bar("#tab_bar") else {
            return;
        };

        if let Some(tab_item) = ui.tab_item("功能") {
            if ui.checkbox("最高金币", &mut (*crate::hook::HOOK_MUT).金币max开关) {
                if !hudhook::windows::Win32::System::Memory::IsBadReadPtr(
                    Some((*crate::hook::HOOK_MUT).金币地址.cast()),
                    4,
                )
                .as_bool()
                {
                    if (*crate::hook::HOOK_MUT).金币max开关 {
                        (*crate::hook::HOOK_MUT).金币旧值 =
                            (*crate::hook::HOOK_MUT).金币地址.read();

                        (*crate::hook::HOOK_MUT).金币地址.write(9999999);
                    } else {
                        (*crate::hook::HOOK_MUT)
                            .金币地址
                            .write((*crate::hook::HOOK_MUT).金币旧值);
                    }
                }
            }

            if ui.checkbox("最高木材", &mut (*crate::hook::HOOK_MUT).木头max开关) {
                if !hudhook::windows::Win32::System::Memory::IsBadReadPtr(
                    Some((*crate::hook::HOOK_MUT).木头地址.cast()),
                    4,
                )
                .as_bool()
                {
                    if (*crate::hook::HOOK_MUT).木头max开关 {
                        (*crate::hook::HOOK_MUT).木头旧值 =
                            (*crate::hook::HOOK_MUT).木头地址.read();

                        (*crate::hook::HOOK_MUT).木头地址.write(0x3FFF);
                    } else {
                        (*crate::hook::HOOK_MUT)
                            .木头地址
                            .write((*crate::hook::HOOK_MUT).木头旧值);
                    }
                }
            }

            if ui.checkbox("钓鱼自动提竿", &mut (*crate::hook::HOOK_MUT).自动钓鱼.开关)
            {
                (*crate::hook::HOOK_MUT).自动钓鱼.切换开关();

                (*crate::hook::HOOK_MUT).自动按键.开关 = (*crate::hook::HOOK_MUT).自动钓鱼.开关;

                (*crate::hook::HOOK_MUT).自动按键.切换开关();
            }

            if ui.checkbox("穿墙", &mut (*crate::hook::HOOK_MUT).穿墙.开关) {
                (*crate::hook::HOOK_MUT).穿墙.切换开关();
            }

            if ui.checkbox(
                "百倍送礼友谊",
                &mut (*crate::hook::HOOK_MUT).居民友谊倍率.开关,
            ) {
                (*crate::hook::HOOK_MUT).居民友谊倍率.切换开关()
            }

            if ui.checkbox(
                "百倍技能经验",
                &mut (*crate::hook::HOOK_MUT).技能经验倍率.开关,
            ) {
                (*crate::hook::HOOK_MUT).技能经验倍率.切换开关()
            }

            if ui.checkbox(
                "百倍战斗经验",
                &mut (*crate::hook::HOOK_MUT).战斗经验倍率.开关,
            ) {
                (*crate::hook::HOOK_MUT).战斗经验倍率.切换开关()
            }

            if ui.checkbox("100%驯服魔物", &mut (*crate::hook::HOOK_MUT).立即驯服.开关) {
                (*crate::hook::HOOK_MUT).立即驯服.切换开关()
            }

            if ui.checkbox("无限委托", &mut (*crate::hook::HOOK_MUT).无限委托.开关) {
                (*crate::hook::HOOK_MUT).无限委托.切换开关()
            }

            if ui.checkbox("无负面状态", &mut (*crate::hook::HOOK_MUT).无负面状态.开关)
            {
                (*crate::hook::HOOK_MUT).无负面状态.切换开关()
            }

            if ui.checkbox("百倍伤害", &mut (*crate::hook::HOOK_MUT).伤害倍率.开关) {
                (*crate::hook::HOOK_MUT).伤害倍率.切换开关()
            }

            tab_item.end();
        }

        if let Some(tab_item) = ui.tab_item("农田") {
            if ui.checkbox(
                "作物即时成熟",
                &mut (*crate::hook::HOOK_MUT).作物立即长成.开关,
            ) {
                (*crate::hook::HOOK_MUT).作物立即长成.切换开关()
            }

            if ui.checkbox("耕作所有土地", &mut (*crate::hook::HOOK_MUT).自动耕作开关) {
                if (*crate::hook::HOOK_MUT).自动耕作开关 {
                    (*crate::hook::HOOK_MUT).自动耕作标签 = 1;
                } else {
                    (*crate::hook::HOOK_MUT).自动耕作标签 = 0;
                }
            }

            if ui.checkbox("土地状态最优", &mut (*crate::hook::HOOK_MUT).土壤质量开关) {
                if (*crate::hook::HOOK_MUT).土壤质量开关 {
                    (*crate::hook::HOOK_MUT).土壤质量标签 = 1;
                } else {
                    (*crate::hook::HOOK_MUT).土壤质量标签 = 0;
                }
            }

            if ui.checkbox("自动浇水", &mut (*crate::hook::HOOK_MUT).自动浇水开关) {
                if (*crate::hook::HOOK_MUT).自动浇水开关 {
                    (*crate::hook::HOOK_MUT).自动浇水标签 = 1;
                } else {
                    (*crate::hook::HOOK_MUT).自动浇水标签 = 0;
                }
            }

            if ui.checkbox("自动种植", &mut (*crate::hook::HOOK_MUT).自动种植开关) {
                if (*crate::hook::HOOK_MUT).自动种植开关 {
                    (*crate::hook::HOOK_MUT).自动种植标签 = 1;
                } else {
                    (*crate::hook::HOOK_MUT).自动种植标签 = 0;
                }
            }

            if (*crate::hook::HOOK_MUT).自动种植开关 {
                if let Some(cb) = ui.begin_combo("种子类型", self.选择的作物.to_string()) {
                    for current in self.作物类型列表 {
                        if self.选择的作物 == *current {
                            ui.set_item_default_focus();
                        }

                        if ui
                            .selectable_config(current.to_string())
                            .selected(self.选择的作物 == *current)
                            .build()
                        {
                            self.选择的作物 = *current;
                        }
                    }

                    cb.end();
                }

                ui.same_line();

                if ui.button("设置##类型") {
                    (*crate::hook::HOOK_MUT)
                        .作物属性
                        .设置作物类型(self.选择的作物);
                }

                if let Some(cb) = ui.begin_combo("种子等级", self.选择的作物等级.to_string())
                {
                    for current in self.作物等级列表 {
                        if self.选择的作物等级 == *current {
                            ui.set_item_default_focus();
                        }

                        if ui
                            .selectable_config(current.to_string())
                            .selected(self.选择的作物等级 == *current)
                            .build()
                        {
                            self.选择的作物等级 = *current;
                        }
                    }

                    cb.end();
                }

                ui.same_line();

                if ui.button("设置##等级") {
                    (*crate::hook::HOOK_MUT)
                        .作物属性
                        .设置作物等级(self.选择的作物等级);
                }

                if let Some(cb) = ui.begin_combo("成长阶段", self.选择的作物生长阶段.to_string())
                {
                    for current in self.作物生长阶段列表 {
                        if self.选择的作物生长阶段 == *current {
                            ui.set_item_default_focus();
                        }

                        if ui
                            .selectable_config(current.to_string())
                            .selected(self.选择的作物生长阶段 == *current)
                            .build()
                        {
                            self.选择的作物生长阶段 = *current;
                        }
                    }

                    cb.end();
                }

                ui.same_line();

                if ui.button("设置##阶段") {
                    (*crate::hook::HOOK_MUT)
                        .作物属性
                        .设置作物生长阶段(self.选择的作物生长阶段);
                }

                if ui.button("清除农田作物") {
                    self.选择的作物 = 作物类型::无;

                    (*crate::hook::HOOK_MUT).作物属性.设置作物类型(作物类型::无);
                }
            }

            tab_item.end();
        }

        if let Some(tab_item) = ui.tab_item("时间") {
            if let Some(cb) = ui.begin_combo("秒", self.选择的秒.to_string()) {
                for current in &self.秒列表 {
                    if self.选择的秒 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(self.选择的秒 == *current)
                        .build()
                    {
                        self.选择的秒 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##秒") {
                (*(*crate::hook::HOOK_MUT).时间指针).秒 = self.选择的秒;
            }

            if let Some(cb) = ui.begin_combo("小时", self.选择的时.to_string()) {
                for current in &self.时列表 {
                    if self.选择的时 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(self.选择的时 == *current)
                        .build()
                    {
                        self.选择的时 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##时") {
                (*(*crate::hook::HOOK_MUT).时间指针).时 = self.选择的时;
            }

            if let Some(cb) = ui.begin_combo("天", self.选择的天.to_string()) {
                for current in &self.天列表 {
                    if self.选择的天 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(self.选择的天 == *current)
                        .build()
                    {
                        self.选择的天 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##天") {
                (*(*crate::hook::HOOK_MUT).时间指针).天 = self.选择的天;
            }

            if let Some(cb) = ui.begin_combo("季节", self.选择的季节.to_string()) {
                for current in self.季节列表 {
                    if self.选择的季节 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(self.选择的季节 == *current)
                        .build()
                    {
                        self.选择的季节 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##季节") {
                (*(*crate::hook::HOOK_MUT).时间指针).季节 = self.选择的季节 as u8;
            }

            if let Some(cb) = ui.begin_combo("年", self.选择的年.to_string()) {
                for current in &self.年列表 {
                    if self.选择的年 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(self.选择的年 == *current)
                        .build()
                    {
                        self.选择的年 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##年") {
                (*(*crate::hook::HOOK_MUT).时间指针).年 = self.选择的年;
            }

            if let Some(cb) = ui.begin_combo("流速", self.选择的流速.to_string()) {
                for current in self.时间流速列表 {
                    if self.选择的流速 == *current {
                        ui.set_item_default_focus();
                    }

                    if ui
                        .selectable_config(current.to_string())
                        .selected(self.选择的流速 == *current)
                        .build()
                    {
                        self.选择的流速 = *current;
                    }
                }

                cb.end();
            }

            ui.same_line();

            if ui.button("设置##流速") {
                (*(*crate::hook::HOOK_MUT).时间指针).流速 = self.选择的流速 as u32;
            }

            if ui.button("暂停时间") {
                self.选择的流速 = 时间流速::暂停时间;

                (*(*crate::hook::HOOK_MUT).时间指针).流速 = self.选择的流速 as u32;
            }

            ui.same_line();

            if ui.button("恢复时间") {
                self.选择的流速 = 时间流速::默认;

                (*(*crate::hook::HOOK_MUT).时间指针).流速 = self.选择的流速 as u32;
            }

            tab_item.end();
        }

        tab_bar.end();
    }
}

impl 作物属性_结构体 {
    pub(crate) fn 设置作物类型(&mut self, v: 作物类型) {
        if v as u8 == 0 {
            self.类型 = 0;
        }

        self.类型 = (v as u8) << 1;
    }

    pub(crate) unsafe fn 设置作物生长阶段(&mut self, v: 作物生长阶段) {
        self.状态.生长阶段 &= 0b0000_1111;

        self.状态.生长阶段 |= (v as u8) << 4;
    }

    pub(crate) unsafe fn 设置作物等级(&mut self, v: 作物等级) {
        self.状态.等级 &= 0b0111_0000;

        self.状态.等级 |= v as u8;
    }
}

pub(crate) unsafe fn 按键被按下一次(virtual_key_code: i32) -> bool {
    static mut 按键按下: bool = false;

    if (hudhook::windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(virtual_key_code)
        as u16
        & 0x8000)
        != 0
    {
        if !按键按下 {
            按键按下 = true;

            return true;
        }
    } else if 按键按下 {
        按键按下 = false;
    }

    false
}
