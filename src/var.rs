pub(crate) const VK_CODE: i32 = 0xC0;

// pub(crate) static mut DEBUG_CONSOLE: bool = false;

pub(crate) static mut SANDLL_ADDR: i64 = 0;

pub(crate) static mut WINDOW_HANDLE: isize = 0;

pub(crate) static mut IS_SHOW_UI: bool = true;

pub(crate) mod fishing {
    // target 5 + 3
    // size 37
    pub(crate) const PAT: &str = "0F B7 48 18 66 83 F9 03";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod auto_press {
    // target 5 + 1
    // size 25
    pub(crate) const PAT: &str = "66 F7 D2 66 23 D0";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut AUTO_PRESS_MARK: i64 = 0;
}

pub(crate) mod walk_through_walls {
    // target 5 + 1
    pub(crate) const PAT: &str = "48 8B F2 48 85 C9";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod friendship_mul {
    // target 5 + 1
    pub(crate) const PAT: &str = "44 8B CA 4D 85 DB";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod instant_crop_growth {
    // target 5 + 1
    pub(crate) const PAT: &str = "8B 10 D1 EA 83 E2 7F 74";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod skill_exp_mul {
    // target 5 + 1
    pub(crate) const PAT: &str = "4C 63 C2 0F B7 CE";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod farm {
    // target 5 + 3
    pub(crate) const PAT: &str = "48 83 C3 08 66 41 3B FF";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;

    pub(crate) mod soil_quality {
        pub(crate) static mut MARK: i64 = 0;
        pub(crate) static mut TOGGLE: bool = false;
    }
    pub(crate) mod watering_plots {
        pub(crate) static mut MARK: i64 = 0;
        pub(crate) static mut TOGGLE: bool = false;
    }
    pub(crate) mod tilth_plots {
        pub(crate) static mut MARK: i64 = 0;
        pub(crate) static mut TOGGLE: bool = false;
    }

    pub(crate) mod plant_plots {

        pub(crate) static mut MARK: i64 = 0;
        pub(crate) static mut TOGGLE: bool = false;
        pub(crate) static mut CROP_PROP: crate::var::CropProp = crate::var::CropProp {
            ty: 0,
            growth_stage_lv: 0x10,
        };
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub(crate) struct CropProp {
    ty: u8,
    growth_stage_lv: u8,
}

impl CropProp {
    pub(crate) fn set_crop_type(&mut self, ct: CropType) {
        self.ty = (ct as u8) << 1;
    }

    pub(crate) unsafe fn set_crop_growth_stage(&mut self, stage: CropGrowthStage) {
        // let stage_lv_ptr = (self as *mut CropProp as *mut u8).byte_add(1);
        // *stage_lv_ptr &= 0b0000_1111;
        // *stage_lv_ptr |= (stage as u8) << 4;
        self.growth_stage_lv &= 0b0000_1111;
        self.growth_stage_lv |= (stage as u8) << 4;
    }

    pub(crate) fn set_crop_level(&mut self, level: CropLevel) {
        self.growth_stage_lv &= 0b0111_0000;
        self.growth_stage_lv |= level as u8;
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub(crate) enum CropType {
    无 = 0,

    石头 = 1, // 可捡
    岩石 = 2, // 可砸
    树枝 = 3, // 可捡
    树桩 = 4, // 可劈
    木材 = 5, // 可砸，什么都不会出
    毒沼 = 6, // 可砸，什么都不会出

    // 矿石 = 7, // 锤子砸会闪退
    药草 = 8,    // 可捡
    解毒草 = 9,  // 可捡
    黑草 = 10,   // 可捡
    枯草 = 11,   // 可捡
    黄草 = 14,   // 可捡
    苦橙草 = 15, // 可捡

    // 种子 = 16, // 不可捡。名字就叫 “种子
    杂草 = 17,   // 可捡
    季节岩 = 18, // 可砸
    花卉 = 19,   // 可摧毁

    水晶 = 20, // 可砸，出的不知道是不是buff

    // 苹果 = 21, //  可砸，什么都不会出
    // 苹果 = 22    同上
    // 苹果 = 23    同上
    草莓 = 24,     // Strawberry
    卷心菜 = 25,   // Cabbage
    樱芜菁 = 26,   // Pink Turnip
    洋葱 = 27,     // Onion
    托伊药草 = 28, // Toyherb
    月落草 = 29,   // Moondrop Flower
    樱草 = 30,     // Cherry Grass
    灯草 = 31,     // Lamp Grass
    青水晶 = 33,   // Blue Crystal Flower
    金卷心菜 = 34, // Golden King Cabbage
    少女蜜瓜 = 35, // Pink Melon

    竹笋 = 36, // 可割

    南瓜 = 37,     // Pumpkin
    黄瓜 = 38,     // Cucumber
    玉米 = 39,     // Corn
    番茄 = 40,     // Tomato
    茄子 = 41,     // Eggplant
    菠萝 = 42,     // Pineapple
    粉红猫 = 43,   // Pink Cat
    铁千轮 = 44,   // Ironleaf
    四叶草 = 45,   // 4-Leaf Clover
    原之焰火 = 46, // Fireflower
    绿水晶 = 47,   // Green Crystal Flower
    金南瓜 = 48,   // Golden Pumpkin

    蓝草 = 49, // 可捡
    绿草 = 50, // 可捡
    紫草 = 51, // 可捡
    靛草 = 52, // 可捡

    红叶花 = 59,     // Autumn Grass
    剧毒蒲公英 = 60, // Pom-Pom Grass
    红水晶 = 61,     // Red Crystal Flower
    金马铃薯 = 62,   // Golden Potato
    芜菁 = 63,       // Turnip
    白萝卜 = 64,     // Radish
    葱 = 65,         // Leek
    白菜 = 66,       // Napa Cabbage
    树形草 = 67,     // Noel Grass
    白水晶 = 68,     // White Crystal Flower
    金芜青 = 69,     // Golden Turnip
    火热果实 = 70,   // Hot-Hot Fruit

    白草 = 71, // 可捡
               //无效 = 72  从72开始的编号都是无效的东西
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]

pub(crate) enum CropLevel {
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

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub(crate) enum CropGrowthStage {
    // 无 = 0,
    一阶段 = 0x1,
    二阶段 = 0x2,
    三阶段 = 0x3,
    四阶段 = 0x4,
    五阶段 = 0x5,
}
