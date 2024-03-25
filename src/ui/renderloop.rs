use super::window::window;

pub(crate) struct RenderLoop;

impl hudhook::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _loader: hudhook::TextureLoader<'a>,
    ) {
        unsafe {
            super::init::init(_ctx);
        }
    }

    fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        unsafe { window(ui) }
    }
}
