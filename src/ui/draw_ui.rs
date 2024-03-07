pub(crate) unsafe fn on_frame(ui: &hudhook::imgui::Ui) {
    if ui.collapsing_header("功能", hudhook::imgui::TreeNodeFlags::empty()) {
        super::component::set_gold(ui);
        super::component::set_wood(ui);
        super::component::fishing_swtich(ui);
        super::component::walk_through_walls_swtich(ui);
        super::component::friendship_mul_swtich(ui);
        super::component::skill_exp_mul_swtich(ui);

        super::component::crop_instant_growth_swtich(ui);
    }

    if ui.collapsing_header("农田", hudhook::imgui::TreeNodeFlags::empty()) {
        super::component::farm_swtich(ui);

        if crate::hook::farm::HOOK.get_swtich() {
            super::component::tilth_plots_swtich(ui);
            super::component::soil_quality_swtich(ui);
            super::component::watering_plots_swtich(ui);
            super::component::plant_plots_swtich(ui);

            if crate::hook::farm::plant_plots::TOGGLE {
                super::component::crop_type_set(ui);
                super::component::crop_level_set(ui);
                super::component::crop_growth_stage_set(ui);
                super::component::clear_crop(ui);
            }
        }
    }

    static mut TIME_POINTER_INITED: bool = false;

    if ui.collapsing_header("时间", hudhook::imgui::TreeNodeFlags::empty()) {
        crate::ui::component::time_manager_swtich(ui);

        if crate::hook::time::HOOK.get_swtich() {
            if !TIME_POINTER_INITED {
                progress_bar("等待进入游戏...\0");
                TIME_POINTER_INITED = !crate::hook::time::POINTER.is_null()
            } else {
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
}

unsafe fn progress_bar(s: &str) {
    static mut PROGRESS: f32 = 0.0f32;
    static mut ANIMATE: bool = true;
    static mut PROGRESS_DIR: f32 = 1.0f32;
    if ANIMATE {
        PROGRESS += PROGRESS_DIR * 0.4f32 * hudhook::imgui::sys::igGetIO().read().DeltaTime;
        if PROGRESS >= 1.1f32 {
            PROGRESS = 1.1f32;
            PROGRESS_DIR *= -1.0f32;
        }
        if PROGRESS <= -0.1f32 {
            PROGRESS = -0.1f32;
            PROGRESS_DIR *= -1.0f32;
        }

        hudhook::imgui::sys::igProgressBar(
            PROGRESS,
            hudhook::imgui::sys::ImVec2 {
                x: 0.0f32,
                y: 0.0f32,
            },
            s.as_ptr().cast(),
        );
    }
}
