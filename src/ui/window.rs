use super::IS_SHOW_UI;
use crate::hook::{FARM_HOOK, PLANT_PLOTS_TOGGLE, SAVE_LOAD_HOOK, SAVE_LOAD_MARK, TIME_HOOK};

pub(crate) unsafe fn window(ui: &hudhook::imgui::Ui) {
    if is_key_down_once(0xC0) {
        IS_SHOW_UI = !IS_SHOW_UI;
    }

    if !IS_SHOW_UI {
        (*hudhook::imgui::sys::igGetIO()).MouseDrawCursor = false;
        return;
    }

    (*hudhook::imgui::sys::igGetIO()).MouseDrawCursor = true;

    static ONCE: ::std::sync::Once = ::std::sync::Once::new();
    ONCE.call_once(|| SAVE_LOAD_HOOK.enable());

    ui.window(format!("符文工房3修改器\t[~]键打开/关闭菜单"))
        .title_bar(true)
        .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .build(|| {
            if SAVE_LOAD_MARK == 0 {
                super::component::progress_bar("等待进入游戏...\0");
            } else {
                on_frame(ui);
            }
        });
}

pub(crate) unsafe fn on_frame(ui: &hudhook::imgui::Ui) {
    if ui.collapsing_header("功能", hudhook::imgui::TreeNodeFlags::empty()) {
        super::component::set_gold(ui);
        super::component::set_wood(ui);
        super::component::fishing_swtich(ui);
        super::component::walk_through_walls_swtich(ui);
        super::component::friendship_mul_swtich(ui);
        super::component::skill_exp_mul_swtich(ui);
        super::component::inf_mission(ui);
        super::component::crop_instant_growth_swtich(ui);
    }

    if ui.collapsing_header("农田", hudhook::imgui::TreeNodeFlags::empty()) {
        super::component::farm_swtich(ui);

        if FARM_HOOK.get_swtich() {
            super::component::tilth_plots_swtich(ui);
            super::component::soil_quality_swtich(ui);
            super::component::watering_plots_swtich(ui);
            super::component::plant_plots_swtich(ui);

            if PLANT_PLOTS_TOGGLE {
                super::component::crop_type_set(ui);
                super::component::crop_level_set(ui);
                super::component::crop_growth_stage_set(ui);
                super::component::clear_crop(ui);
            }
        }
    }

    if ui.collapsing_header("时间", hudhook::imgui::TreeNodeFlags::empty()) {
        crate::ui::component::time_swtich(ui);

        if TIME_HOOK.get_swtich() {
            super::component::time_second_set(ui);
            super::component::time_hour_set(ui);
            super::component::time_day_set(ui);
            super::component::time_season_set(ui);
            super::component::time_year_set(ui);
            super::component::time_slow_mul_set(ui);
            super::component::time_pause(ui);
            ui.same_line();
            super::component::time_default(ui);
        }
    }
}

pub(crate) unsafe fn is_key_down_once(virtual_key_code: i32) -> bool {
    static mut WAS_KEY_DOWN: bool = false;

    if (crate::GetAsyncKeyState(virtual_key_code) & 0x8000) != 0 {
        if !WAS_KEY_DOWN {
            WAS_KEY_DOWN = true;
            return true;
        }
    } else {
        WAS_KEY_DOWN = false;
    }

    false
}
