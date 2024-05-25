use super::component;
use crate::hook::{FARM_HOOK, PLANT_PLOTS_TOGGLE, TIME_HOOK};

pub(crate) unsafe fn on_frame(ui: &hudhook::imgui::Ui) {
    if let Some(tab_bar) = ui.tab_bar("#tab_bar") {
        if let Some(tab_item) = ui.tab_item("功能") {
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

            tab_item.end();
        }

        if let Some(tab_item) = ui.tab_item("农田") {
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

            tab_item.end();
        }

        if let Some(tab_item) = ui.tab_item("时间") {
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

            tab_item.end();
        }

        tab_bar.end();
    }
}
