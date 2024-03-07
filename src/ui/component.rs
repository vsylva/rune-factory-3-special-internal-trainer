pub(crate) unsafe fn set_gold(ui: &hudhook::imgui::Ui) {
    ui.text("金币: ");

    ui.same_line();
    ui.text(format!("{}", *crate::hook::COIN_ADDR));

    ui.same_line();
    if ui.button("+100k") {
        *crate::hook::COIN_ADDR += 100_000;
    }

    if *crate::hook::COIN_ADDR > 100_000 {
        ui.same_line();
        if ui.button("-100k") {
            *crate::hook::COIN_ADDR -= 100_000;
        }
    }
}

pub(crate) unsafe fn fishing_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "钓鱼自动提竿",
        std::mem::transmute(std::ptr::addr_of_mut!(crate::hook::fishing::TOGGLE)),
    ) {
        if crate::hook::fishing::TOGGLE == true {
            crate::hook::fishing::HOOK.enable();
            crate::hook::auto_press::HOOK.enable();
        } else {
            crate::hook::fishing::HOOK.disable();
            crate::hook::auto_press::HOOK.disable();
        }
    }
}
pub(crate) unsafe fn walk_through_walls_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "穿墙",
        std::mem::transmute(std::ptr::addr_of_mut!(
            crate::hook::walk_through_walls::TOGGLE
        )),
    ) {
        if crate::hook::walk_through_walls::TOGGLE == true {
            crate::hook::walk_through_walls::HOOK.enable();
        } else {
            crate::hook::walk_through_walls::HOOK.disable();
        }
    }
}
pub(crate) unsafe fn friendship_mul_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "送礼百倍友谊",
        std::mem::transmute(std::ptr::addr_of_mut!(crate::hook::friendship_mul::TOGGLE)),
    ) {
        if crate::hook::friendship_mul::TOGGLE == true {
            crate::hook::friendship_mul::HOOK.enable();
        } else {
            crate::hook::friendship_mul::HOOK.disable();
        }
    }
}

pub(crate) unsafe fn skill_exp_mul_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "百倍技能经验",
        std::mem::transmute(std::ptr::addr_of_mut!(crate::hook::skill_exp_mul::TOGGLE)),
    ) {
        if crate::hook::skill_exp_mul::TOGGLE == true {
            crate::hook::skill_exp_mul::HOOK.enable();
        } else {
            crate::hook::skill_exp_mul::HOOK.disable();
        }
    }
}
pub(crate) unsafe fn crop_instant_growth_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "作物即时成熟",
        std::mem::transmute(std::ptr::addr_of_mut!(
            crate::hook::instant_crop_growth::TOGGLE
        )),
    ) {
        if crate::hook::instant_crop_growth::TOGGLE == true {
            crate::hook::instant_crop_growth::HOOK.enable();
        } else {
            crate::hook::instant_crop_growth::HOOK.disable();
        }
    }
}
pub(crate) unsafe fn farm_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "开启",
        std::mem::transmute(std::ptr::addr_of_mut!(crate::hook::farm::TOGGLE)),
    ) {
        if crate::hook::farm::TOGGLE == true {
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

pub(crate) unsafe fn tilth_plots_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "耕作所有土地",
        std::mem::transmute(std::ptr::addr_of_mut!(
            crate::hook::farm::tilth_plots::TOGGLE
        )),
    ) {
        if crate::hook::farm::tilth_plots::TOGGLE == true {
            crate::hook::farm::tilth_plots::MARK = 1;
        } else {
            crate::hook::farm::tilth_plots::MARK = 0;
        }
    }
}

pub(crate) unsafe fn soil_quality_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "土地状态最优",
        std::mem::transmute(std::ptr::addr_of_mut!(
            crate::hook::farm::soil_quality::TOGGLE
        )),
    ) {
        if crate::hook::farm::soil_quality::TOGGLE {
            crate::hook::farm::soil_quality::MARK = 1;
        } else {
            crate::hook::farm::soil_quality::MARK = 0;
        }
    }
}

pub(crate) unsafe fn watering_plots_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "自动灌溉",
        std::mem::transmute(std::ptr::addr_of_mut!(
            crate::hook::farm::watering_plots::TOGGLE
        )),
    ) {
        if crate::hook::farm::watering_plots::TOGGLE {
            crate::hook::farm::watering_plots::MARK = 1;
        } else {
            crate::hook::farm::watering_plots::MARK = 0;
        }
    }
}

pub(crate) unsafe fn plant_plots_toggle(ui: &hudhook::imgui::Ui) {
    if ui.checkbox(
        "自动种植",
        std::mem::transmute(std::ptr::addr_of_mut!(
            crate::hook::farm::plant_plots::TOGGLE
        )),
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
