use hudhook_mini::RenderContext;

mod component;

mod style;
mod window;

pub(crate) static mut IS_SHOW_UI: bool = true;

static mut CROP_TYPE_SELECTED: CropType = CropType::无;
static mut CROP_TYPE_LIST: Vec<CropType> = Vec::new();

static mut CROP_LEVEL_SELECTED: CropLevel = CropLevel::LV1;
static mut CROP_LEVEL_LIST: Vec<CropLevel> = Vec::new();

static mut CROP_GROWTH_STAGE_SELECTED: CropGrowthStage = CropGrowthStage::一阶段;
static mut CROP_GROWTH_STAGE_LIST: Vec<CropGrowthStage> = Vec::new();

static mut TIME_SECOND_SELECTED: u8 = 0;
static mut TIME_SECOND_LIST: Vec<u8> = Vec::new();
static mut TIME_HOUR_SELECTED: u8 = 0;
static mut TIME_HOUR_LIST: Vec<u8> = Vec::new();
static mut TIME_DAY_SELECTED: u8 = 1;
static mut TIME_DAY_LIST: Vec<u8> = Vec::new();
static mut TIME_SEASON_SELECTED: Season = Season::春;
static mut TIME_SEASON_LIST: Vec<Season> = Vec::new();
static mut TIME_YEAR_SELECTED: u8 = 1;
static mut TIME_YEAR_LIST: Vec<u8> = Vec::new();
static mut TIME_SLOW_MUL_SELECTED: TimeSlowMul = TimeSlowMul::默认;
static mut TIME_SLOW_MUL_LIST: Vec<TimeSlowMul> = Vec::new();

pub(crate) struct RenderLoop;

impl hudhook_mini::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook_mini::imgui::Context,
        _render_context: &'a mut dyn RenderContext,
    ) {
        unsafe {
            _ctx.set_ini_filename(None);

            crate::ui::style::set_dark_red_style(_ctx);
            crate::ui::style::set_font(_ctx, 20.0);

            for crop_type in <CropType as strum::IntoEnumIterator>::iter() {
                CROP_TYPE_LIST.push(crop_type)
            }

            for crop_level in <CropLevel as strum::IntoEnumIterator>::iter() {
                CROP_LEVEL_LIST.push(crop_level)
            }

            for crop_growth_stage in <CropGrowthStage as strum::IntoEnumIterator>::iter() {
                CROP_GROWTH_STAGE_LIST.push(crop_growth_stage)
            }

            for second in 0..60 {
                TIME_SECOND_LIST.push(second);
            }

            for hour in 0..24 {
                TIME_HOUR_LIST.push(hour);
            }

            for day in 1..31 {
                TIME_DAY_LIST.push(day);
            }

            for season in <Season as strum::IntoEnumIterator>::iter() {
                TIME_SEASON_LIST.push(season);
            }

            for year in 1..100 {
                TIME_YEAR_LIST.push(year);
            }

            for time_slow_mul in <TimeSlowMul as strum::IntoEnumIterator>::iter() {
                TIME_SLOW_MUL_LIST.push(time_slow_mul)
            }
        }
    }
    fn render(&mut self, ui: &mut hudhook_mini::imgui::Ui) {
        unsafe { window::window(ui) }
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
pub(crate) enum Season {
    春 = 0,
    夏 = 1,
    秋 = 2,
    冬 = 3,
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
    strum_macros::EnumString,
)]
pub(crate) enum TimeSlowMul {
    暂停时间 = 0,
    #[strum(serialize = "0.01倍")]
    百分之一 = 0x3D,
    #[strum(serialize = "0.1倍")]
    十分之一 = 0x266,
    #[strum(serialize = "0.25倍")]
    四分之一 = 0x600,
    #[strum(serialize = "0.5倍")]
    二分之一 = 0xC00,
    默认 = 0x1800,
    #[strum(serialize = "1.5倍")]
    一点五 = 0x2400,
    #[strum(serialize = "2.0倍")]
    两点零 = 0x3000,
}

#[repr(C)]
pub(crate) struct CropProp {
    pub(crate) type_: u8,
    pub(crate) data: CropPropType,
}

#[repr(C)]
pub(crate) union CropPropType {
    pub(crate) stage: u8,
    pub(crate) level: u8,
}

impl CropProp {
    pub(crate) fn set_crop_type(&mut self, ct: CropType) {
        if ct as u8 == 0 {
            self.type_ = 0;
        }
        self.type_ = (ct as u8) << 1;
    }

    pub(crate) unsafe fn set_crop_growth_stage(&mut self, stage: CropGrowthStage) {
        self.data.stage &= 0b0000_1111;
        self.data.stage |= (stage as u8) << 4;
    }

    pub(crate) unsafe fn set_crop_level(&mut self, level: CropLevel) {
        self.data.level &= 0b0111_0000;
        self.data.level |= level as u8;
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

    白草 = 71, // 可捡
               //从72开始的编号都是无效的东西
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
