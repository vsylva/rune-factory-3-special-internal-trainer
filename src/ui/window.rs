use super::{component, IS_SHOW_UI};
use crate::hook::{FARM_HOOK, PLANT_PLOTS_TOGGLE, SAVE_LOAD_HOOK, SAVE_LOAD_MARK, TIME_HOOK};

pub(crate) unsafe fn on_frame(ui: &hudhook::imgui::Ui) {
    if ui.collapsing_header("功能", hudhook::imgui::TreeNodeFlags::empty()) {
        component::set_gold(ui);
        component::set_wood(ui);
        component::fishing_swtich(ui);
        component::walk_through_walls_swtich(ui);
        component::friendship_mul_swtich(ui);
        component::skill_exp_mul_swtich(ui);
        component::combat_exp_mul_swtich(ui);
        component::inf_mission(ui);
        component::tame_swtich(ui);
        component::no_debuff(ui);
        // component::damage_mul_swtich(ui);
        component::crop_instant_growth_swtich(ui);
    }

    if ui.collapsing_header("农田", hudhook::imgui::TreeNodeFlags::empty()) {
        component::farm_swtich(ui);

        if FARM_HOOK.is_enabled {
            component::tilth_plots_swtich(ui);
            component::soil_quality_swtich(ui);
            component::watering_plots_swtich(ui);
            component::plant_plots_swtich(ui);

            if PLANT_PLOTS_TOGGLE {
                component::crop_type_set(ui);
                component::crop_level_set(ui);
                component::crop_growth_stage_set(ui);
                component::clear_crop(ui);
            }
        }
    }

    if ui.collapsing_header("时间", hudhook::imgui::TreeNodeFlags::empty()) {
        crate::ui::component::time_swtich(ui);

        if TIME_HOOK.is_enabled {
            component::time_second_set(ui);
            component::time_hour_set(ui);
            component::time_day_set(ui);
            component::time_season_set(ui);
            component::time_year_set(ui);
            component::time_slow_mul_set(ui);
            component::time_pause(ui);
            ui.same_line();
            component::time_default(ui);
        }
    }
}

pub(crate) unsafe fn window(ui: &hudhook::imgui::Ui) {
    if is_key_down_once(0xC0) {
        IS_SHOW_UI = !IS_SHOW_UI;
    }

    if !IS_SHOW_UI {
        (*hudhook::imgui::sys::igGetIO()).MouseDrawCursor = false;
        return;
    }

    (*hudhook::imgui::sys::igGetIO()).MouseDrawCursor = true;

    ui.window("符文工房3修改器\t[~]键打开/关闭菜单")
        .title_bar(true)
        .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .build(|| {
            static ONCE: ::std::sync::Once = ::std::sync::Once::new();
            ONCE.call_once(|| {
                SAVE_LOAD_HOOK.is_enabled = true;
                SAVE_LOAD_HOOK.switch();
            });

            if SAVE_LOAD_MARK == 0 {
                component::loading_bar("等待载入存档...\0");

                return;
            }

            static ONCE1: ::std::sync::Once = ::std::sync::Once::new();
            ONCE1.call_once(|| {
                SAVE_LOAD_HOOK.is_enabled = false;
                SAVE_LOAD_HOOK.switch();

                minhook_raw::remove_hook(SAVE_LOAD_HOOK.target_addr);
            });

            on_frame(ui)
        });
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
