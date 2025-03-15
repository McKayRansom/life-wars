use super::{GameOptions, Scene};
use crate::{context::{Context, GameSpeed}, draw::ViewContext};

use macroquad::{color, input, time::get_time};

use life_io::life::{Cell, Life, LifeAlgoSelect, LifeRule};

pub const DEFAULT_MAP_SIZE: (i16, i16) = (127, 127);

pub struct Gameplay {
    life: Box<Life>,
    // ui: UiState,
    last_map_update: f64,
    // popup: Option<Popup>,
    ctx: ViewContext,
}

impl GameOptions {
    pub fn create(&self) -> Life {
        let seed = 23317;
        let mut life = Life::new_rule(LifeAlgoSelect::Basic, (256, 256), LifeRule::STAR_WARS);
        life.randomize(seed, true);
        life
        // match &self {
        //     GameOptions::New => Ok(Map::new_generate(DEFAULT_MAP_SIZE)),
        //     GameOptions::Level(level) => Ok(new_level(*level)),
        //     GameOptions::Continue => Map::load(),
        // }
    }
}

impl Gameplay {
    pub async fn new(_ctx: &mut Context, life: Box<Life>) -> Self {
        // let unlocked = map.metadata.unlocks;
        let gameplay = Gameplay {
            life,
            // ui: UiState::new(unlocked),
            last_map_update: get_time(),
            // popup: None,
            ctx: ViewContext::new(),
        };

        // ctx.tileset.reset_camera(gameplay.map.grid.size_px());

        gameplay
    }
}

const GAME_SPEED_NORMAL: f64 = 1. / 8.;
const GAME_SPEED_FAST: f64 = 1. / 16.;


fn handle_input(life: &mut Life, view_ctx: &mut ViewContext, ctx: &mut Context) {
    let mouse_pos = input::mouse_position();

    let pos = view_ctx.screen_to_life_pos(mouse_pos);

    if let Some(chr) = input::get_char_pressed() {
        match chr {
            'q' => ctx.request_quit = true,
            ' ' => {
                if matches!(ctx.game_speed, GameSpeed::Paused) {
                    ctx.game_speed = GameSpeed::Normal
                } else {
                    ctx.game_speed = GameSpeed::Paused
                }
            }
            // ctx.paused = !view_ctx.paused,
            // '1' => ctx.selected_faction = 0,
            // '2' => ctx.selected_faction = 1,
            // '3' => ctx.selected_faction = 2,
            // '4' => ctx.selected_faction = 3,
            'g' => life.paste(&Life::new_life_from_rle(life_io::life::GOSPER_RLE), pos.unwrap()),
            'p' => {
                // if let Some(string) = clipboard_get() {
                //     println!("Pasting {string:?}");
                //     life.paste(&Life::new_life_from_rle(string.as_str()), pos)
                // } else {
                println!("No clipboard!");
                // }
            }
            _ => {}
        }
    }

    if input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
        life.insert(pos.unwrap(), Cell::new(1, 0)); //view_ctx.selected_faction));
    }
}

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        let map_speed = match ctx.game_speed {
            GameSpeed::Paused => return,
            GameSpeed::Normal => GAME_SPEED_NORMAL,
            GameSpeed::FastForward => GAME_SPEED_FAST,
        };

        if get_time() - self.last_map_update > map_speed {
            // if self.map.update() && self.map.metadata.is_level {
            //     self.popup = Some(Popup::new(format!(
            //         "Level {} completed!",
            //         self.map.metadata.level_number
            //     )));
            // }
            self.last_map_update = macroquad::time::get_time();
            self.life.update();
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        let size = self.life.size();
        self.ctx.resize_to_fit(size, ((macroquad::window::screen_width() - BORDER_SIZE), (macroquad::window::screen_height() - BORDER_SIZE)));

        handle_input(&mut self.life, &mut self.ctx, ctx);

        crate::draw::draw_life(&self.life, &self.ctx);
        // draw_life(&life, &ctx);

        macroquad::text::draw_text_ex("Life Viewer", 10., 30., macroquad::text::TextParams {
            font: Some(&ctx.font),
            font_size: 40,
            color: color::GREEN,
            ..Default::default()
        });

        // self.ui.draw(&mut self.map, ctx);

        // if let Some(popup) = &self.popup {
        //     match popup.draw() {
        //         Some(PopupResult::Ok) => {
        //             let level_number = self.map.metadata.level_number + 1;
        //             ctx.switch_scene_to = if level_number < LEVEL_COUNT {
        //                 Some(EScene::Gameplay(Box::new(new_level(level_number))))
        //             } else {
        //                 Some(EScene::MainMenu)
        //             }
        //         }
        //         Some(PopupResult::Cancel) => {
        //             self.popup = None;
        //             self.map.metadata.level_complete = true;
        //         }
        //         None => {}
        //     }
        // }
    }
}


const BORDER_SIZE: f32 = 40.;
