use super::{GameOptions, Scene};
use crate::{
    context::Context,
    draw::{self, ViewContext},
};

use macroquad::{color, input, time::get_time};

use life_io::life::{self, Cell, Life, LifeAlgoSelect, LifeRule};

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

const GAME_SPEED_1_PAUSED: f64 = 0.;
const GAME_SPEED_2_NORMAL: f64 = 1. / 8.;
const GAME_SPEED_3_FAST: f64 = 1. / 15.;
const GAME_SPEED_4_VERY_FAST: f64 = 1. / 30.;
const GAME_SPEED_5_EXTREME: f64 = 1. / 60.;
const GAME_SPEED_6_VERY_EXTREME: f64 = 1. / 120.;

fn handle_input(life: &mut Life, view_ctx: &mut ViewContext, ctx: &mut Context) {
    let mouse_pos = input::mouse_position();

    let pos = view_ctx.screen_to_life_pos(mouse_pos);

    if let Some(chr) = input::get_char_pressed() {
        match chr {
            'q' => ctx.request_quit = true,
            ' ' => {
                if ctx.game_speed == GAME_SPEED_1_PAUSED {
                    ctx.game_speed = GAME_SPEED_2_NORMAL;
                } else {
                    ctx.game_speed = GAME_SPEED_1_PAUSED;
                }
            }
            // ctx.paused = !view_ctx.paused,
            '1' => ctx.game_speed = GAME_SPEED_1_PAUSED,
            '2' => ctx.game_speed = GAME_SPEED_2_NORMAL,
            '3' => ctx.game_speed = GAME_SPEED_3_FAST,
            '4' => ctx.game_speed = GAME_SPEED_4_VERY_FAST,
            '5' => ctx.game_speed = GAME_SPEED_5_EXTREME,
            '6' => ctx.game_speed = GAME_SPEED_6_VERY_EXTREME,
            'g' => life.paste(
                &Life::new_life_from_rle(life_io::life::GOSPER_RLE),
                pos.unwrap(),
            ),
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

fn draw_score(life: &Life, ctx: &Context) {
    let mut pos_y = 30.;
    let pos_x = macroquad::window::screen_width();
    let mut pops: Vec<(i16, u8)> = (0..life::FACTION_MAX)
        .filter_map(|faction| {
            let pop = life.get_pop(faction as u8);
            if pop > 0 { Some((pop, faction as u8)) } else { None }
        })
        .collect();
    pops.sort();

    for (pop, faction) in pops.iter().rev() {

        let faction_text = format!("Team {faction}: {pop}");
        let measure = macroquad::text::measure_text(faction_text.as_str(), Some(&ctx.font), 40, 1.);
        macroquad::text::draw_text_ex(
            faction_text.as_str(),
            pos_x - measure.width - 10.,
            pos_y,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size: 40,
                color: draw::faction_color(*faction),
                ..Default::default()
            },
        );

        pos_y += measure.height + 10.;
    }
}

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        if ctx.game_speed != GAME_SPEED_1_PAUSED
            && get_time() - self.last_map_update > ctx.game_speed
        {
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
        self.ctx.resize_to_fit(
            size,
            (
                (macroquad::window::screen_width() - BORDER_SIZE * 2.),
                (macroquad::window::screen_height() - BORDER_SIZE * 2.),
            ),
        );
        self.ctx.set_pos((BORDER_SIZE, BORDER_SIZE));

        handle_input(&mut self.life, &mut self.ctx, ctx);

        crate::draw::draw_life(&self.life, &self.ctx);

        draw_score(&self.life, ctx);

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
