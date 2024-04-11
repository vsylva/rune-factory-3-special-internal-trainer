use std::ptr::addr_of_mut;

use crate::{
    hook::{
        AUTO_PRESS_HOOK, COIN_ADDR, COIN_LAST, COIN_MAX, COMBAT_EXP_MUL_HOOK, CROP_PROP, FARM_HOOK,
        FISHING_HOOK, FRIENDSHIP_MUL_HOOK, INF_MISSION_HOOK, INSTANT_CROP_GROWTH_HOOK,
        NO_DEBUFF_HOOK, PLANT_PLOTS_MARK, PLANT_PLOTS_TOGGLE, SKILL_EXP_MUL_HOOK,
        SOIL_QUALITY_MARK, SOIL_QUALITY_TOGGLE, TAME_HOOK, TILTH_PLOTS_MARK, TILTH_PLOTS_TOGGLE,
        TIME_HOOK, TIME_POINTER, WALK_THROUGH_WALLS_HOOK, WATERING_PLOTS_MARK,
        WATERING_PLOTS_TOGGLE, WOOD_ADDR, WOOD_LAST, WOOD_MAX,
    },
    ui::{
        CropType, TimeSlowMul, CROP_GROWTH_STAGE_LIST, CROP_GROWTH_STAGE_SELECTED, CROP_LEVEL_LIST,
        CROP_LEVEL_SELECTED, CROP_TYPE_LIST, CROP_TYPE_SELECTED, TIME_DAY_LIST, TIME_DAY_SELECTED,
        TIME_HOUR_LIST, TIME_HOUR_SELECTED, TIME_SEASON_LIST, TIME_SEASON_SELECTED,
        TIME_SECOND_LIST, TIME_SECOND_SELECTED, TIME_SLOW_MUL_LIST, TIME_SLOW_MUL_SELECTED,
        TIME_YEAR_LIST, TIME_YEAR_SELECTED,
    },
};

pub(crate) unsafe fn loading_bar(s: &str) {
    static mut PROGRESS: f32 = 0.0f32;
    static mut ANIMATE: bool = true;
    static mut PROGRESS_DIR: f32 = 1.0f32;
    if ANIMATE {
        PROGRESS += PROGRESS_DIR * 0.4f32 * hudhook_mini::imgui::sys::igGetIO().read().DeltaTime;
        if PROGRESS >= 1.1f32 {
            PROGRESS = 1.1f32;
            PROGRESS_DIR *= -1.0f32;
        }
        if PROGRESS <= -0.1f32 {
            PROGRESS = -0.1f32;
            PROGRESS_DIR *= -1.0f32;
        }

        hudhook_mini::imgui::sys::igProgressBar(
            PROGRESS,
            hudhook_mini::imgui::sys::ImVec2 {
                x: 0.0f32,
                y: 0.0f32,
            },
            s.as_ptr().cast(),
        );
    }
}

pub(crate) unsafe fn set_gold(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("最高金币", &mut *addr_of_mut!(COIN_MAX)) {
        if COIN_MAX {
            COIN_LAST = *COIN_ADDR;
            *COIN_ADDR = 99999999;
        } else {
            *COIN_ADDR = COIN_LAST;
        }
    }
}

pub(crate) unsafe fn set_wood(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("最高木材", &mut *addr_of_mut!(WOOD_MAX)) {
        if WOOD_MAX {
            WOOD_LAST = *WOOD_ADDR;
            *WOOD_ADDR = 16383;
        } else {
            *WOOD_ADDR = WOOD_LAST;
        }
    }
}

pub(crate) unsafe fn fishing_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("钓鱼自动提竿", &mut FISHING_HOOK.is_enabled) {
        FISHING_HOOK.switch();

        AUTO_PRESS_HOOK.is_enabled = FISHING_HOOK.is_enabled;
        AUTO_PRESS_HOOK.switch();
    }
}
pub(crate) unsafe fn walk_through_walls_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("穿墙", &mut WALK_THROUGH_WALLS_HOOK.is_enabled) {
        WALK_THROUGH_WALLS_HOOK.switch();
    }
}
pub(crate) unsafe fn friendship_mul_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("百倍送礼友谊", &mut FRIENDSHIP_MUL_HOOK.is_enabled) {
        FRIENDSHIP_MUL_HOOK.switch()
    }
}

pub(crate) unsafe fn skill_exp_mul_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("百倍技能经验", &mut SKILL_EXP_MUL_HOOK.is_enabled) {
        SKILL_EXP_MUL_HOOK.switch()
    }
}

// pub(crate) unsafe fn damage_mul_swtich(ui: &hudhook_mini::imgui::Ui) {
//     if ui.checkbox("百倍伤害", &mut DAMAGE_MUL_HOOK.is_enabled) {
//         DAMAGE_MUL_HOOK.switch()
//     }
// }

pub(crate) unsafe fn combat_exp_mul_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("百倍战斗经验", &mut COMBAT_EXP_MUL_HOOK.is_enabled) {
        COMBAT_EXP_MUL_HOOK.switch()
    }
}

pub(crate) unsafe fn tame_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("100%驯服魔物", &mut TAME_HOOK.is_enabled) {
        TAME_HOOK.switch()
    }
}

pub(crate) unsafe fn inf_mission(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("无限委托", &mut INF_MISSION_HOOK.is_enabled) {
        INF_MISSION_HOOK.switch()
    }
}

pub(crate) unsafe fn no_debuff(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("无负面状态", &mut NO_DEBUFF_HOOK.is_enabled) {
        NO_DEBUFF_HOOK.switch()
    }
}

pub(crate) unsafe fn crop_instant_growth_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("作物即时成熟", &mut INSTANT_CROP_GROWTH_HOOK.is_enabled) {
        INSTANT_CROP_GROWTH_HOOK.switch()
    }
}

pub(crate) unsafe fn farm_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("开启农田面板", &mut FARM_HOOK.is_enabled) {
        FARM_HOOK.switch();

        if !FARM_HOOK.is_enabled {
            TILTH_PLOTS_MARK = 0;
            SOIL_QUALITY_MARK = 0;
            WATERING_PLOTS_MARK = 0;
            PLANT_PLOTS_MARK = 0;

            TILTH_PLOTS_TOGGLE = false;
            SOIL_QUALITY_TOGGLE = false;
            WATERING_PLOTS_TOGGLE = false;
            PLANT_PLOTS_TOGGLE = false;
        }
    }
}

pub(crate) unsafe fn tilth_plots_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox(
        "耕作所有土地",
        &mut *std::ptr::addr_of_mut!(TILTH_PLOTS_TOGGLE),
    ) {
        if TILTH_PLOTS_TOGGLE {
            TILTH_PLOTS_MARK = 1;
        } else {
            TILTH_PLOTS_MARK = 0;
        }
    }
}

pub(crate) unsafe fn soil_quality_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox(
        "土地状态最优",
        &mut *std::ptr::addr_of_mut!(SOIL_QUALITY_TOGGLE),
    ) {
        if SOIL_QUALITY_TOGGLE {
            SOIL_QUALITY_MARK = 1;
        } else {
            SOIL_QUALITY_MARK = 0;
        }
    }
}

pub(crate) unsafe fn watering_plots_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox(
        "自动灌溉",
        &mut *std::ptr::addr_of_mut!(WATERING_PLOTS_TOGGLE),
    ) {
        if WATERING_PLOTS_TOGGLE {
            WATERING_PLOTS_MARK = 1;
        } else {
            WATERING_PLOTS_MARK = 0;
        }
    }
}

pub(crate) unsafe fn plant_plots_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("自动种植", &mut *std::ptr::addr_of_mut!(PLANT_PLOTS_TOGGLE)) {
        if PLANT_PLOTS_TOGGLE {
            PLANT_PLOTS_MARK = 1;
        } else {
            PLANT_PLOTS_MARK = 0;
        }
    }
}

pub(crate) unsafe fn crop_type_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("种子类型", CROP_TYPE_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(CROP_TYPE_LIST) {
            if CROP_TYPE_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(CROP_TYPE_SELECTED == *current)
                .build()
            {
                CROP_TYPE_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置类型") {
        CROP_PROP.set_crop_type(CROP_TYPE_SELECTED);
    }
}

pub(crate) unsafe fn crop_level_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("种子等级", CROP_LEVEL_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of!(CROP_LEVEL_LIST) {
            if CROP_LEVEL_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(CROP_LEVEL_SELECTED == *current)
                .build()
            {
                CROP_LEVEL_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置等级") {
        CROP_PROP.set_crop_level(CROP_LEVEL_SELECTED);
    }
}
pub(crate) unsafe fn crop_growth_stage_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("成长阶段", CROP_GROWTH_STAGE_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(CROP_GROWTH_STAGE_LIST) {
            if CROP_GROWTH_STAGE_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(CROP_GROWTH_STAGE_SELECTED == *current)
                .build()
            {
                CROP_GROWTH_STAGE_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置阶段") {
        CROP_PROP.set_crop_growth_stage(CROP_GROWTH_STAGE_SELECTED);
    }
}

pub(crate) unsafe fn clear_crop(ui: &hudhook_mini::imgui::Ui) {
    if ui.button("清除农田作物") {
        CROP_TYPE_SELECTED = CropType::无;
        CROP_PROP.set_crop_type(CropType::无);
    }
}

pub(crate) unsafe fn time_swtich(ui: &hudhook_mini::imgui::Ui) {
    if ui.checkbox("开启时间面板", &mut TIME_HOOK.is_enabled) {
        TIME_HOOK.switch()
    }
}

pub(crate) unsafe fn time_second_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("秒", TIME_SECOND_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(TIME_SECOND_LIST) {
            if TIME_SECOND_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(TIME_SECOND_SELECTED == *current)
                .build()
            {
                TIME_SECOND_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置秒数") {
        (*TIME_POINTER).set_second(TIME_SECOND_SELECTED);
    }
}

pub(crate) unsafe fn time_hour_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("小时", TIME_HOUR_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(TIME_HOUR_LIST) {
            if TIME_HOUR_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(TIME_HOUR_SELECTED == *current)
                .build()
            {
                TIME_HOUR_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置小时数") {
        (*TIME_POINTER).set_hour(TIME_HOUR_SELECTED);
    }
}

pub(crate) unsafe fn time_day_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("天", TIME_DAY_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(TIME_DAY_LIST) {
            if TIME_DAY_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(TIME_DAY_SELECTED == *current)
                .build()
            {
                TIME_DAY_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置天数") {
        (*TIME_POINTER).set_day(TIME_DAY_SELECTED);
    }
}

pub(crate) unsafe fn time_season_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("季节", TIME_SEASON_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(TIME_SEASON_LIST) {
            if TIME_SEASON_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(TIME_SEASON_SELECTED == *current)
                .build()
            {
                TIME_SEASON_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置季节") {
        (*TIME_POINTER).set_season(TIME_SEASON_SELECTED as u8);
    }
}

pub(crate) unsafe fn time_year_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("年", TIME_YEAR_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(TIME_YEAR_LIST) {
            if TIME_YEAR_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(TIME_YEAR_SELECTED == *current)
                .build()
            {
                TIME_YEAR_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置年") {
        (*TIME_POINTER).set_year(TIME_YEAR_SELECTED);
    }
}

pub(crate) unsafe fn time_slow_mul_set(ui: &hudhook_mini::imgui::Ui) {
    if let Some(cb) = ui.begin_combo("流速", TIME_SLOW_MUL_SELECTED.to_string()) {
        for current in &*::core::ptr::addr_of_mut!(TIME_SLOW_MUL_LIST) {
            if TIME_SLOW_MUL_SELECTED == *current {
                ui.set_item_default_focus();
            }

            if ui
                .selectable_config(current.to_string())
                .selected(TIME_SLOW_MUL_SELECTED == *current)
                .build()
            {
                TIME_SLOW_MUL_SELECTED = *current;
            }
        }
        cb.end();
    }

    ui.same_line();
    if ui.button("设置流速") {
        (*TIME_POINTER).set_slow_mul(TIME_SLOW_MUL_SELECTED as u32);
    }
}

pub(crate) unsafe fn time_pause(ui: &hudhook_mini::imgui::Ui) {
    if ui.button("暂停时间") {
        TIME_SLOW_MUL_SELECTED = TimeSlowMul::暂停时间;
        (*TIME_POINTER).set_slow_mul(TIME_SLOW_MUL_SELECTED as u32);
    }
}

pub(crate) unsafe fn time_default(ui: &hudhook_mini::imgui::Ui) {
    if ui.button("恢复时间") {
        TIME_SLOW_MUL_SELECTED = TimeSlowMul::默认;
        (*TIME_POINTER).set_slow_mul(TimeSlowMul::默认 as u32);
    }
}
