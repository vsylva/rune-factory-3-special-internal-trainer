mod body;
mod component;
mod setting;

pub(crate) static mut WINDOW_HANDLE: isize = 0;
pub(crate) static mut IS_SHOW_UI: bool = true;

static mut CROP_TYPE_SELECTED: crate::hook::CropType = crate::hook::CropType::无;
static mut CROP_TYPE_LIST: Vec<crate::hook::CropType> = Vec::new();

static mut CROP_LEVEL_SELECTED: crate::hook::CropLevel = crate::hook::CropLevel::LV1;
static mut CROP_LEVEL_LIST: Vec<crate::hook::CropLevel> = Vec::new();

static mut CROP_GROWTH_STAGE_SELECTED: crate::hook::CropGrowthStage =
    crate::hook::CropGrowthStage::一阶段;
static mut CROP_GROWTH_STAGE_LIST: Vec<crate::hook::CropGrowthStage> = Vec::new();

pub(crate) struct RenderLoop;

impl hudhook::ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _loader: hudhook::TextureLoader<'a>,
    ) {
        unsafe {
            setting::set_dark_red_style(_ctx);
            setting::set_font(_ctx, 20.0);
            _ctx.set_ini_filename(None);

            for v in <crate::hook::CropType as strum::IntoEnumIterator>::iter() {
                CROP_TYPE_LIST.push(v)
            }

            for v in <crate::hook::CropLevel as strum::IntoEnumIterator>::iter() {
                CROP_LEVEL_LIST.push(v)
            }

            for v in <crate::hook::CropGrowthStage as strum::IntoEnumIterator>::iter() {
                CROP_GROWTH_STAGE_LIST.push(v)
            }

            let windows_name = "Rune Factory 3 Special\0"
                .encode_utf16()
                .collect::<Vec<u16>>();

            WINDOW_HANDLE = crate::FindWindowW(::core::ptr::null_mut(), windows_name.as_ptr());
        }
    }

    fn should_block_messages(&self, _io: &hudhook::imgui::Io) -> bool {
        unsafe {
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
        unsafe {
            // VK ~
            if is_key_down_once(0xC0) {
                IS_SHOW_UI = !IS_SHOW_UI;
            }

            if !IS_SHOW_UI {
                return;
            }

            ui.window(format!("符文工房3修改器\t[~]键打开/关闭菜单"))
                .title_bar(true)
                .size([500.0, 400.0], hudhook::imgui::Condition::FirstUseEver)
                .resizable(true)
                .collapsible(true)
                .movable(true)
                .build(|| {
                    body::on_frame(ui);
                });
        }
    }
}

pub(crate) unsafe fn is_key_down_once(virtual_key_code: i32) -> bool {
    static WAS_KEY_DOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    if (crate::GetAsyncKeyState(virtual_key_code) & 0x8000) != 0 {
        if !WAS_KEY_DOWN.load(std::sync::atomic::Ordering::SeqCst) {
            WAS_KEY_DOWN.store(true, std::sync::atomic::Ordering::SeqCst);
            return true;
        }
    } else {
        WAS_KEY_DOWN.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    false
}
