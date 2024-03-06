use hudhook::{imgui, ImguiRenderLoop, TextureLoader};

use crate::{util::is_key_down_once, var};

static mut CROP_TYPE_SELECTED: var::CropType = var::CropType::无;
static mut CROP_TYPE_LIST: Vec<var::CropType> = Vec::new();

static mut CROP_LEVEL_SELECTED: var::CropLevel = var::CropLevel::LV1;
static mut CROP_LEVEL_LIST: Vec<var::CropLevel> = Vec::new();

static mut CROP_GROWTH_STAGE_SELECTED: var::CropGrowthStage = var::CropGrowthStage::一阶段;
static mut CROP_GROWTH_STAGE_LIST: Vec<var::CropGrowthStage> = Vec::new();

static ONCE: std::sync::Once = std::sync::Once::new();

pub(crate) unsafe fn on_frame(ui: &hudhook::imgui::Ui) {
    ONCE.call_once(|| {
        for v in <var::CropType as strum::IntoEnumIterator>::iter() {
            CROP_TYPE_LIST.push(v)
        }

        for v in <var::CropLevel as strum::IntoEnumIterator>::iter() {
            CROP_LEVEL_LIST.push(v)
        }

        for v in <var::CropGrowthStage as strum::IntoEnumIterator>::iter() {
            CROP_GROWTH_STAGE_LIST.push(v)
        }
    });

    if ui.collapsing_header("功能", imgui::TreeNodeFlags::empty()) {
        if ui.checkbox(
            "钓鱼自动提竿",
            std::mem::transmute(std::ptr::addr_of_mut!(var::fishing::TOGGLE)),
        ) {
            if var::fishing::TOGGLE == true {
                var::fishing::HOOK.enable();
                var::auto_press::HOOK.enable();
            } else {
                var::fishing::HOOK.disable();
                var::auto_press::HOOK.disable();
            }
        }

        if ui.checkbox(
            "穿墙",
            std::mem::transmute(std::ptr::addr_of_mut!(var::walk_through_walls::TOGGLE)),
        ) {
            if var::walk_through_walls::TOGGLE == true {
                var::walk_through_walls::HOOK.enable();
            } else {
                var::walk_through_walls::HOOK.disable();
            }
        }

        if ui.checkbox(
            "送礼百倍友谊",
            std::mem::transmute(std::ptr::addr_of_mut!(var::friendship_mul::TOGGLE)),
        ) {
            if var::friendship_mul::TOGGLE == true {
                var::friendship_mul::HOOK.enable();
            } else {
                var::friendship_mul::HOOK.disable();
            }
        }

        if ui.checkbox(
            "百倍技能经验",
            std::mem::transmute(std::ptr::addr_of_mut!(var::skill_exp_mul::TOGGLE)),
        ) {
            if var::skill_exp_mul::TOGGLE == true {
                var::skill_exp_mul::HOOK.enable();
            } else {
                var::skill_exp_mul::HOOK.disable();
            }
        }

        if ui.checkbox(
            "作物即时成熟",
            std::mem::transmute(std::ptr::addr_of_mut!(var::instant_crop_growth::TOGGLE)),
        ) {
            if var::instant_crop_growth::TOGGLE == true {
                var::instant_crop_growth::HOOK.enable();
            } else {
                var::instant_crop_growth::HOOK.disable();
            }
        }
    }

    if ui.collapsing_header("农田功能", imgui::TreeNodeFlags::empty()) {
        if ui.checkbox(
            "开启",
            std::mem::transmute(std::ptr::addr_of_mut!(var::farm::TOGGLE)),
        ) {
            if var::farm::TOGGLE == true {
                var::farm::HOOK.enable();
            } else {
                crate::var::farm::tilth_plots::MARK = 0;
                crate::var::farm::soil_quality::MARK = 0;
                crate::var::farm::watering_plots::MARK = 0;
                crate::var::farm::plant_plots::MARK = 0;

                crate::var::farm::tilth_plots::TOGGLE = false;
                crate::var::farm::soil_quality::TOGGLE = false;
                crate::var::farm::watering_plots::TOGGLE = false;
                crate::var::farm::plant_plots::TOGGLE = false;

                var::farm::HOOK.disable();
            }
        }

        if var::farm::TOGGLE {
            if ui.checkbox(
                "耕作所有土地",
                std::mem::transmute(std::ptr::addr_of_mut!(var::farm::tilth_plots::TOGGLE)),
            ) {
                if var::farm::tilth_plots::TOGGLE == true {
                    crate::var::farm::tilth_plots::MARK = 1;
                } else {
                    crate::var::farm::tilth_plots::MARK = 0;
                }
            }

            if ui.checkbox(
                "土地状态最优",
                std::mem::transmute(std::ptr::addr_of_mut!(var::farm::soil_quality::TOGGLE)),
            ) {
                if var::farm::soil_quality::TOGGLE {
                    crate::var::farm::soil_quality::MARK = 1;
                } else {
                    crate::var::farm::soil_quality::MARK = 0;
                }
            }

            if ui.checkbox(
                "自动灌溉",
                std::mem::transmute(std::ptr::addr_of_mut!(var::farm::watering_plots::TOGGLE)),
            ) {
                if var::farm::watering_plots::TOGGLE {
                    crate::var::farm::watering_plots::MARK = 1;
                } else {
                    crate::var::farm::watering_plots::MARK = 0;
                }
            }

            if ui.checkbox(
                "自动种植",
                std::mem::transmute(std::ptr::addr_of_mut!(var::farm::plant_plots::TOGGLE)),
            ) {
                if var::farm::plant_plots::TOGGLE {
                    crate::var::farm::plant_plots::MARK = 1;
                } else {
                    crate::var::farm::plant_plots::MARK = 0;
                }
            }

            if var::farm::plant_plots::TOGGLE {
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
                    var::farm::plant_plots::CROP_PROP.set_crop_type(CROP_TYPE_SELECTED);
                }

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
                    var::farm::plant_plots::CROP_PROP.set_crop_level(CROP_LEVEL_SELECTED);
                }

                if let Some(cb) = ui.begin_combo("成长阶段", CROP_GROWTH_STAGE_SELECTED.to_string())
                {
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
                    var::farm::plant_plots::CROP_PROP
                        .set_crop_growth_stage(CROP_GROWTH_STAGE_SELECTED);
                }

                if ui.button("清除农田作物") {
                    var::farm::plant_plots::CROP_PROP.set_crop_type(var::CropType::无);
                }
            }
        }
    }
}

pub(crate) struct MyRenderLoop;

impl ImguiRenderLoop for MyRenderLoop {
    fn initialize<'a>(&'a mut self, _ctx: &mut imgui::Context, _loader: TextureLoader<'a>) {
        ImguiSettings(_ctx)
            .set_dark_red_style()
            .set_font(20.0)
            .set_ini_filename("");
    }

    fn should_block_messages(&self, _io: &imgui::Io) -> bool {
        unsafe {
            if crate::IsIconic(var::WINDOW_HANDLE) != 0 {
                return false;
            }

            (*imgui::sys::igGetIO()).MouseDrawCursor = var::IS_SHOW_UI;

            if var::IS_SHOW_UI {
                return true;
            }
        }
        false
    }

    fn render(&mut self, ui: &mut imgui::Ui) {
        unsafe {
            if is_key_down_once(var::VK_CODE) {
                var::IS_SHOW_UI = !var::IS_SHOW_UI;
            }

            if !var::IS_SHOW_UI {
                return;
            }

            ui.window(format!("符文工房3修改器\t[~]键打开/关闭菜单"))
                .title_bar(true)
                .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
                .resizable(true)
                .collapsible(true)
                .movable(true)
                .build(|| {
                    on_frame(ui);
                });
        }
    }
}

pub(crate) struct ImguiSettings<'a>(pub(crate) &'a mut hudhook::imgui::Context);

impl<'a> ImguiSettings<'a> {
    pub(crate) fn set_dark_red_style(&mut self) -> &mut Self {
        let style = self.0.style_mut();

        style.use_dark_colors();

        style.window_rounding = 0.0;

        style.child_rounding = 0.0;

        style.popup_rounding = 0.0;

        style.frame_rounding = 0.0;

        style.scrollbar_rounding = 0.0;

        style.grab_rounding = 0.0;

        style.tab_rounding = 0.0;

        style.window_border_size = 0.0;

        style.cell_padding = [0.0, 0.0];

        style.window_padding = [0.0, 0.0];

        style.window_title_align = [0.5, 0.5];

        // style.alpha = 0.66;

        let sytle_colors = &mut self.0.style_mut().colors;

        sytle_colors[hudhook::imgui::sys::ImGuiCol_WindowBg as usize] = [0.1, 0.105, 0.11, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_NavHighlight as usize] = [0.3, 0.305, 0.31, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_PlotHistogram as usize] =
            [0.3, 0.305, 0.31, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_Header as usize] = [0.2, 0.205, 0.21, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_HeaderHovered as usize] =
            [0.3, 0.305, 0.31, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_HeaderActive as usize] =
            [0.55, 0.5505, 0.551, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_Button as usize] = [0.2, 0.205, 0.21, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_ButtonHovered as usize] =
            [0.3, 0.305, 0.31, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_ButtonActive as usize] =
            [0.55, 0.5505, 0.551, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_CheckMark as usize] = [0.55, 0.5505, 0.551, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_FrameBg as usize] = [0.211, 0.210, 0.25, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_FrameBgHovered as usize] =
            [0.3, 0.305, 0.31, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_FrameBgActive as usize] =
            [0.55, 0.5505, 0.551, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_Tab as usize] = [0.25, 0.2505, 0.251, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_TabHovered as usize] =
            [0.38, 0.3805, 0.381, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_TabActive as usize] = [0.28, 0.2805, 0.281, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_TabUnfocused as usize] =
            [0.25, 0.2505, 0.251, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_TabUnfocusedActive as usize] =
            [0.8, 0.805, 0.81, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_ResizeGrip as usize] = [0.2, 0.205, 0.21, 0.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_ResizeGripHovered as usize] =
            [0.3, 0.305, 0.31, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_ResizeGripActive as usize] =
            [0.55, 0.5505, 0.551, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_TitleBg as usize] = [1.0, 0.0, 0.0, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_TitleBgActive as usize] = [1.0, 0.0, 0.0, 1.0];

        sytle_colors[hudhook::imgui::sys::ImGuiCol_TitleBgCollapsed as usize] =
            [0.25, 0.2505, 0.251, 1.0];

        self
    }

    pub(crate) fn set_font(&mut self, font_size: f32) -> &mut Self {
        let tf_data = hudhook::imgui::FontSource::TtfData {
            data: include_bytes!("../res/FZHTJW.TTF"),
            size_pixels: font_size,
            config: Some(hudhook::imgui::FontConfig {
                size_pixels: font_size,
                pixel_snap_h: true,
                glyph_ranges: hudhook::imgui::FontGlyphRanges::from_slice(&[
                    0x0020, 0x00FF, 0x2000, 0x206F, 0x3000, 0x30FF, 0x31F0, 0x31FF, 0xFF00, 0xFFEF,
                    0xFFFD, 0xFFFD, 0x4E00, 0x9FAF, 0,
                ]),
                ..hudhook::imgui::FontConfig::default()
            }),
        };

        self.0.fonts().add_font(&[tf_data]);

        self
    }

    pub(crate) fn set_ini_filename(&mut self, name: &str) -> &mut Self {
        if name.is_empty() {
            self.0.set_ini_filename(None)
        } else {
            self.0
                .set_ini_filename(Some(std::path::PathBuf::from(name)));
        }

        self
    }
}
