pub(crate) unsafe fn on_frame(ui: &hudhook::imgui::Ui) {
    if ui.collapsing_header("功能", hudhook::imgui::TreeNodeFlags::empty()) {
        super::component::set_gold(ui);
        super::component::set_wood(ui);
        super::component::fishing_toggle(ui);
        super::component::walk_through_walls_toggle(ui);
        super::component::friendship_mul_toggle(ui);
        super::component::skill_exp_mul_toggle(ui);

        super::component::crop_instant_growth_toggle(ui);
        super::component::time_pause_toggle(ui);
    }

    if ui.collapsing_header("农田功能", hudhook::imgui::TreeNodeFlags::empty()) {
        super::component::farm_toggle(ui);

        if crate::hook::farm::HOOK.get_toggle() {
            super::component::tilth_plots_toggle(ui);
            super::component::soil_quality_toggle(ui);
            super::component::watering_plots_toggle(ui);
            super::component::plant_plots_toggle(ui);

            if crate::hook::farm::plant_plots::TOGGLE {
                super::component::crop_type_set(ui);
                super::component::crop_level_set(ui);
                super::component::crop_growth_stage_set(ui);
                super::component::clear_crop(ui);
            }
        }
    }
}
