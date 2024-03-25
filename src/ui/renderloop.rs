use super::window::window;
use crate::ui::{
    CropGrowthStage,
    CropLevel,
    CropType,
    Season,
    TimeSlowMul,
    CROP_GROWTH_STAGE_LIST,
    CROP_LEVEL_LIST,
    CROP_TYPE_LIST,
    TIME_DAY_LIST,
    TIME_HOUR_LIST,
    TIME_SEASON_LIST,
    TIME_SECOND_LIST,
    TIME_SLOW_MUL_LIST,
    TIME_YEAR_LIST,
};

pub(crate) struct RenderLoop;

impl hudhook::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _loader: hudhook::TextureLoader<'a>,
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

    fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        unsafe { window(ui) }
    }
}
