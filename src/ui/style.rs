pub(crate) fn set_font(ctx: &mut hudhook_mini::imgui::Context, font_size: f32) {
    let tf_data = hudhook_mini::imgui::FontSource::TtfData {
        data: include_bytes!("../../res/FZHTJW.TTF"),
        size_pixels: font_size,
        config: Some(hudhook_mini::imgui::FontConfig {
            size_pixels: font_size,
            pixel_snap_h: true,
            glyph_ranges: hudhook_mini::imgui::FontGlyphRanges::from_slice(&[
                0x0020, 0x00FF, 0x2000, 0x206F, 0x3000, 0x30FF, 0x31F0, 0x31FF, 0xFF00, 0xFFEF,
                0xFFFD, 0xFFFD, 0x4E00, 0x9FAF, 0,
            ]),
            ..hudhook_mini::imgui::FontConfig::default()
        }),
    };

    ctx.fonts().add_font(&[tf_data]);
}

pub(crate) fn set_dark_red_style(ctx: &mut hudhook_mini::imgui::Context) {
    let style = ctx.style_mut();

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

    let sytle_colors = &mut ctx.style_mut().colors;

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_WindowBg as usize] = [0.1, 0.105, 0.11, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_NavHighlight as usize] =
        [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_PlotHistogram as usize] =
        [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_Header as usize] = [0.2, 0.205, 0.21, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_HeaderHovered as usize] =
        [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_HeaderActive as usize] =
        [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_Button as usize] = [0.2, 0.205, 0.21, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_ButtonHovered as usize] =
        [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_ButtonActive as usize] =
        [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_CheckMark as usize] =
        [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_FrameBg as usize] = [0.211, 0.210, 0.25, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_FrameBgHovered as usize] =
        [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_FrameBgActive as usize] =
        [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_Tab as usize] = [0.25, 0.2505, 0.251, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_TabHovered as usize] =
        [0.38, 0.3805, 0.381, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_TabActive as usize] =
        [0.28, 0.2805, 0.281, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_TabUnfocused as usize] =
        [0.25, 0.2505, 0.251, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_TabUnfocusedActive as usize] =
        [0.8, 0.805, 0.81, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_ResizeGrip as usize] = [0.2, 0.205, 0.21, 0.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_ResizeGripHovered as usize] =
        [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_ResizeGripActive as usize] =
        [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_TitleBg as usize] = [1.0, 0.0, 0.0, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_TitleBgActive as usize] = [1.0, 0.0, 0.0, 1.0];

    sytle_colors[hudhook_mini::imgui::sys::ImGuiCol_TitleBgCollapsed as usize] =
        [0.25, 0.2505, 0.251, 1.0];
}
