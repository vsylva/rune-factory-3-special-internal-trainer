pub(crate) unsafe fn set_gold(ui: &hudhook::imgui::Ui) {
    ui.text("金币: ");

    ui.same_line();
    ui.text(format!("{}", *crate::hook::COIN_ADDR));

    ui.same_line();
    if ui.button("+100k") {
        *crate::hook::COIN_ADDR += 100_000;
    }

    if *crate::hook::COIN_ADDR > 99_999 {
        ui.same_line();
        if ui.button("-100k") {
            *crate::hook::COIN_ADDR -= 100_000;
        }
    }
}

pub(crate) unsafe fn set_wood(ui: &hudhook::imgui::Ui) {
    ui.text("木材: ");

    ui.same_line();
    ui.text(format!("{}", *crate::hook::WOOD_ADDR));

    ui.same_line();
    if ui.button("+1000") {
        *crate::hook::WOOD_ADDR += 1000;
    }

    if *crate::hook::WOOD_ADDR > 999 {
        ui.same_line();
        if ui.button("-1000") {
            *crate::hook::WOOD_ADDR -= 1000;
        }
    }
}

pub(crate) unsafe fn fishing_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox("钓鱼自动提竿", crate::hook::fishing::HOOK.get_swtich_mut()) {
        if crate::hook::fishing::HOOK.get_swtich() {
            crate::hook::fishing::HOOK.enable();
            crate::hook::auto_press::HOOK.enable();
        } else {
            crate::hook::fishing::HOOK.disable();
            crate::hook::auto_press::HOOK.disable();
        }
    }
}
pub(crate) unsafe fn walk_through_walls_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "穿墙",
        crate::hook::walk_through_walls::HOOK.get_swtich_mut(),
    ) {
        if crate::hook::walk_through_walls::HOOK.get_swtich() {
            crate::hook::walk_through_walls::HOOK.enable();
        } else {
            crate::hook::walk_through_walls::HOOK.disable();
        }
    }
}
pub(crate) unsafe fn friendship_mul_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "送礼百倍友谊",
        crate::hook::friendship_mul::HOOK.get_swtich_mut(),
    ) {
        if crate::hook::friendship_mul::HOOK.get_swtich() {
            crate::hook::friendship_mul::HOOK.enable();
        } else {
            crate::hook::friendship_mul::HOOK.disable();
        }
    }
}

pub(crate) unsafe fn skill_exp_mul_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "百倍技能经验",
        crate::hook::skill_exp_mul::HOOK.get_swtich_mut(),
    ) {
        if crate::hook::skill_exp_mul::HOOK.get_swtich() {
            crate::hook::skill_exp_mul::HOOK.enable();
        } else {
            crate::hook::skill_exp_mul::HOOK.disable();
        }
    }
}
pub(crate) unsafe fn crop_instant_growth_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "作物即时成熟",
        crate::hook::instant_crop_growth::HOOK.get_swtich_mut(),
    ) {
        if crate::hook::instant_crop_growth::HOOK.get_swtich() {
            crate::hook::instant_crop_growth::HOOK.enable();
        } else {
            crate::hook::instant_crop_growth::HOOK.disable();
        }
    }
}

pub(crate) unsafe fn farm_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox("开启", crate::hook::farm::HOOK.get_swtich_mut()) {
        if crate::hook::farm::HOOK.get_swtich() {
            crate::hook::farm::HOOK.enable();
        } else {
            crate::hook::farm::tilth_plots::MARK = 0;
            crate::hook::farm::soil_quality::MARK = 0;
            crate::hook::farm::watering_plots::MARK = 0;
            crate::hook::farm::plant_plots::MARK = 0;

            crate::hook::farm::tilth_plots::TOGGLE = false;
            crate::hook::farm::soil_quality::TOGGLE = false;
            crate::hook::farm::watering_plots::TOGGLE = false;
            crate::hook::farm::plant_plots::TOGGLE = false;
            crate::hook::farm::HOOK.disable();
        }
    }
}

pub(crate) unsafe fn tilth_plots_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "耕作所有土地",
        &mut *std::ptr::addr_of_mut!(crate::hook::farm::tilth_plots::TOGGLE),
    ) {
        if crate::hook::farm::tilth_plots::TOGGLE == true {
            crate::hook::farm::tilth_plots::MARK = 1;
        } else {
            crate::hook::farm::tilth_plots::MARK = 0;
        }
    }
}

pub(crate) unsafe fn soil_quality_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "土地状态最优",
        &mut *std::ptr::addr_of_mut!(crate::hook::farm::soil_quality::TOGGLE),
    ) {
        if crate::hook::farm::soil_quality::TOGGLE {
            crate::hook::farm::soil_quality::MARK = 1;
        } else {
            crate::hook::farm::soil_quality::MARK = 0;
        }
    }
}

pub(crate) unsafe fn watering_plots_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "自动灌溉",
        &mut *std::ptr::addr_of_mut!(crate::hook::farm::watering_plots::TOGGLE),
    ) {
        if crate::hook::farm::watering_plots::TOGGLE {
            crate::hook::farm::watering_plots::MARK = 1;
        } else {
            crate::hook::farm::watering_plots::MARK = 0;
        }
    }
}

pub(crate) unsafe fn plant_plots_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "自动种植",
        &mut *std::ptr::addr_of_mut!(crate::hook::farm::plant_plots::TOGGLE),
    ) {
        if crate::hook::farm::plant_plots::TOGGLE {
            crate::hook::farm::plant_plots::MARK = 1;
        } else {
            crate::hook::farm::plant_plots::MARK = 0;
        }
    }
}

pub(crate) unsafe fn crop_type_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("种子类型", super::CROP_TYPE_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(super::CROP_TYPE_LIST) {
            if super::CROP_TYPE_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::CROP_TYPE_SELECTED == *current)
                .build()
            {
                super::CROP_TYPE_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置类型") {
        crate::hook::farm::plant_plots::CROP_PROP.set_crop_type(super::CROP_TYPE_SELECTED);
    }
}

pub(crate) unsafe fn crop_level_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("种子等级", super::CROP_LEVEL_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of!(super::CROP_LEVEL_LIST) {
            if super::CROP_LEVEL_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::CROP_LEVEL_SELECTED == *current)
                .build()
            {
                super::CROP_LEVEL_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置等级") {
        crate::hook::farm::plant_plots::CROP_PROP.set_crop_level(super::CROP_LEVEL_SELECTED);
    }
}
pub(crate) unsafe fn crop_growth_stage_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("成长阶段", super::CROP_GROWTH_STAGE_SELECTED.to_string())
    {
        for current in &*::core::ptr::addr_of_mut!(super::CROP_GROWTH_STAGE_LIST) {
            if super::CROP_GROWTH_STAGE_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::CROP_GROWTH_STAGE_SELECTED == *current)
                .build()
            {
                super::CROP_GROWTH_STAGE_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置阶段") {
        crate::hook::farm::plant_plots::CROP_PROP
            .set_crop_growth_stage(super::CROP_GROWTH_STAGE_SELECTED);
    }
}

pub(crate) unsafe fn clear_crop(ui: &hudhook::imgui::Ui) {
    if ui.button("清除农田作物") {
        super::CROP_TYPE_SELECTED = crate::hook::CropType::无;
        crate::hook::farm::plant_plots::CROP_PROP.set_crop_type(crate::hook::CropType::无);
    }
}

pub(crate) unsafe fn time_manager_swtich(ui: &hudhook::imgui::Ui) {
    if ui.checkbox("开启", crate::hook::time::HOOK.get_swtich_mut()) {
        if crate::hook::time::HOOK.get_swtich() {
            crate::hook::time::HOOK.enable();
        } else {
            crate::hook::time::HOOK.disable();
        }
    }
}

pub(crate) unsafe fn time_second_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("秒", super::TIME_SECOND_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(super::TIME_SECOND_LIST) {
            if super::TIME_SECOND_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::TIME_SECOND_SELECTED == *current)
                .build()
            {
                super::TIME_SECOND_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置秒数") {
        (*crate::hook::time::POINTER).set_second(super::TIME_SECOND_SELECTED);
    }
}

pub(crate) unsafe fn time_hour_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("小时", super::TIME_HOUR_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(super::TIME_HOUR_LIST) {
            if super::TIME_HOUR_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::TIME_HOUR_SELECTED == *current)
                .build()
            {
                super::TIME_HOUR_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置小时数") {
        (*crate::hook::time::POINTER).set_hour(super::TIME_HOUR_SELECTED);
    }
}

pub(crate) unsafe fn time_day_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("天", super::TIME_DAY_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(super::TIME_DAY_LIST) {
            if super::TIME_DAY_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::TIME_DAY_SELECTED == *current)
                .build()
            {
                super::TIME_DAY_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置天数") {
        (*crate::hook::time::POINTER).set_day(super::TIME_DAY_SELECTED);
    }
}

pub(crate) unsafe fn time_season_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("季节", super::TIME_SEASON_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(super::TIME_SEASON_LIST) {
            if super::TIME_SEASON_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::TIME_SEASON_SELECTED == *current)
                .build()
            {
                super::TIME_SEASON_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置季节") {
        (*crate::hook::time::POINTER).set_season(super::TIME_SEASON_SELECTED);
    }
}

pub(crate) unsafe fn time_year_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("年", super::TIME_YEAR_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(super::TIME_YEAR_LIST) {
            if super::TIME_YEAR_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::TIME_YEAR_SELECTED == *current)
                .build()
            {
                super::TIME_YEAR_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置年") {
        (*crate::hook::time::POINTER).set_year(super::TIME_YEAR_SELECTED);
    }
}

pub(crate) unsafe fn time_slow_mul_set(ui: &hudhook::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("流速", super::TIME_SLOW_MUL_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(super::TIME_SLOW_MUL_LIST) {
            if super::TIME_SLOW_MUL_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(super::TIME_SLOW_MUL_SELECTED == *current)
                .build()
            {
                super::TIME_SLOW_MUL_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置流速") {
        (*crate::hook::time::POINTER).set_slow_mul(super::TIME_SLOW_MUL_SELECTED);
    }
}

pub(crate) unsafe fn time_pause(ui: &hudhook::imgui::Ui) {
    if ui.button("暂停时间") {
        (*crate::hook::time::POINTER).set_slow_mul(super::TIME_SLOW_MUL_SELECTED);
        super::TIME_SLOW_MUL_SELECTED = super::TimeSlowMul::暂停时间;
    }
}

pub(crate) unsafe fn time_default(ui: &hudhook::imgui::Ui) {
    if ui.button("恢复时间") {
        (*crate::hook::time::POINTER).set_slow_mul(super::TimeSlowMul::默认);
        super::TIME_SLOW_MUL_SELECTED = super::TimeSlowMul::默认;
    }
}
