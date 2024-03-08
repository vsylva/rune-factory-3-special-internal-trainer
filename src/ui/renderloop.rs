use super::draw::windows;
use crate::ui::IS_SHOW_UI;

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

    fn should_block_messages(&self, _io: &hudhook::imgui::Io) -> bool {
        unsafe {
            static ONCE: ::std::sync::Once = ::std::sync::Once::new();

            pub(crate) static mut WINDOW_HANDLE: isize = 0;

            ONCE.call_once(|| {
                let windows_name = "Rune Factory 3 Special\0"
                    .encode_utf16()
                    .collect::<Vec<u16>>();

                WINDOW_HANDLE = crate::FindWindowW(::core::ptr::null_mut(), windows_name.as_ptr());
            });

            if crate::IsIconic(WINDOW_HANDLE) != 0 {
                return false;
            }

            (*hudhook::imgui::sys::igGetIO()).MouseDrawCursor = IS_SHOW_UI;

            if IS_SHOW_UI {
                return true;
            }
        }
        false
    }

    fn render(&mut self, ui: &mut hudhook::imgui::Ui) {
        unsafe { windows(ui) }
    }
}
