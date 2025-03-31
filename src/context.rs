use life_io::life::pattern_lib;
use life_io::viewer::ViewContext;
use macroquad::text::Font;

// use crate::assets_path::determine_asset_path;
// use crate::audio;
// use crate::consts::*;
// use crate::font;
// use crate::save::Save;
use crate::scene::EScene;
use crate::skin;
// use crate::settings::Settings;
// use crate::tileset::Tileset;
// use crate::ui::skin;
// use macroquad::math::Rect;
// use macroquad::miniquad::FilterMode;
// use macroquad::texture::render_target;
// use macroquad::camera::Camera2D;

/// game-wide data and resources
pub struct Context {
    pub request_quit: bool,
    pub view_context: ViewContext,
    // pub gamepads: Gamepads,
    // pub textures: texture::TextureAtlas,
    // pub tileset: Tileset,
    // pub fonts: font::FontAtlas,
    pub font: Font,
    // pub audio: audio::AudioAtlas,
    // pub render_target: RenderTarget,
    // pub render_target_cam: Camera2D,
    pub switch_scene_to: Option<EScene>,
    // pub settings: Settings,
    // pub save: Save,
    pub pattern_lib: pattern_lib::PatternLib,
}

impl Context {
    pub async fn new() -> Self {
        Self {
            // gamepads: Gamepads::new(),
            view_context: ViewContext::default(),
            request_quit: false,
            // tileset: Tileset::new().await,
            font: skin::init().await,
            // audio: audio::AudioAtlas::new().await,
            // fonts: font::FontAtlas::new().await,
            // render_target,
            // render_target_cam,
            switch_scene_to: None,
            // settings: Settings::load(),
            // save: Save::load(),
            pattern_lib: pattern_lib::PatternLib::new(),
        }
    }

    // global keybinds
    fn input_handler(&mut self) {
        if let Some(chr) = self.view_context.key_pressed.take() {
            match chr {
                'q' => self.request_quit = true,
                _ => {
                    self.view_context.key_pressed = Some(chr);
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.view_context.update();
        self.input_handler();
    }
}
