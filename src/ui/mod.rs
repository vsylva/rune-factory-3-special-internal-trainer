mod component;
mod draw_ui;
mod setting;

pub(crate) static mut WINDOW_HANDLE: isize = 0;
pub(crate) static mut IS_SHOW_UI: bool = true;

static mut CROP_TYPE_SELECTED: crate::hook::CropType = crate::hook::CropType::无;
static mut CROP_TYPE_LIST: Vec<crate::hook::CropType> = Vec::new();

static mut CROP_LEVEL_SELECTED: crate::hook::CropLevel = crate::hook::CropLevel::LV1;
static mut CROP_LEVEL_LIST: Vec<crate::hook::CropLevel> = Vec::new();

static mut CROP_GROWTH_STAGE_SELECTED: crate::hook::CropGrowthStage =
    crate::hook::CropGrowthStage::一阶段;
static mut CROP_GROWTH_STAGE_LIST: Vec<crate::hook::CropGrowthStage> = Vec::new();

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
)]
pub(crate) enum TimeSlowMul {
    暂停时间 = 0,
    百分之一 = 0x3D,
    十分之一 = 0x266,
    四分之一 = 0x600,
    二分之一 = 0xC00,
    默认 = 0x1800,
    一点五 = 0x2400,
    两点零 = 0x3000,
}

impl Into<TimeSlowMul> for u32 {
    fn into(self) -> TimeSlowMul {
        match self {
            0 => TimeSlowMul::暂停时间,
            0x3D => TimeSlowMul::百分之一,
            0x266 => TimeSlowMul::十分之一,
            0x600 => TimeSlowMul::四分之一,
            0xC00 => TimeSlowMul::二分之一,
            0x1800 => TimeSlowMul::默认,
            0x2400 => TimeSlowMul::一点五,
            0x3000 => TimeSlowMul::两点零,
            _ => unreachable!(),
        }
    }
}

pub(crate) struct RenderLoop;

impl hudhook::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _loader: hudhook::TextureLoader<'a>,
    ) {
        unsafe {
            setting::set_dark_red_style(_ctx);
            setting::set_font(_ctx, 20.0);
            _ctx.set_ini_filename(None);

            for crop_type in <crate::hook::CropType as strum::IntoEnumIterator>::iter() {
                CROP_TYPE_LIST.push(crop_type)
            }

            for crop_level in <crate::hook::CropLevel as strum::IntoEnumIterator>::iter() {
                CROP_LEVEL_LIST.push(crop_level)
            }

            for crop_growth_stage in
                <crate::hook::CropGrowthStage as strum::IntoEnumIterator>::iter()
            {
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

            let windows_name = "Rune Factory 3 Special\0"
                .encode_utf16()
                .collect::<Vec<u16>>();

            WINDOW_HANDLE = crate::FindWindowW(::core::ptr::null_mut(), windows_name.as_ptr());
        }
    }

    fn should_block_messages(&self, _io: &hudhook::imgui::Io) -> bool {
        unsafe {
            if crate::IsIconic(WINDOW_HANDLE) != 0 {
                return false;
            }

            (*hudhook::imgui::sys::igGetIO()).MouseDrawCursor = IS_SHOW_UI;

            if IS_SHOW_UI {
                return true;
            }
        }
        false
    }

    fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        unsafe {
            // VK ~
            if is_key_down_once(0xC0) {
                IS_SHOW_UI = !IS_SHOW_UI;
            }

            if !IS_SHOW_UI {
                return;
            }

            ui.window(format!("符文工房3修改器\t[~]键打开/关闭菜单"))
                .title_bar(true)
                .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
                .resizable(true)
                .collapsible(true)
                .movable(true)
                .build(|| {
                    draw_ui::on_frame(ui);
                });
        }
    }
}

pub(crate) unsafe fn is_key_down_once(virtual_key_code: i32) -> bool {
    static WAS_KEY_DOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    if (crate::GetAsyncKeyState(virtual_key_code) & 0x8000) != 0 {
        if !WAS_KEY_DOWN.load(std::sync::atomic::Ordering::SeqCst) {
            WAS_KEY_DOWN.store(true, std::sync::atomic::Ordering::SeqCst);
            return true;
        }
    } else {
        WAS_KEY_DOWN.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    false
}
