use super::Scene;
use crate::{
    context::Context,
    viewer::{self, LifeViewer},
};

use macroquad::input;

use life_io::life::{self, Cell, Life, LifeAlgoSelect, LifeRule};

pub const DEFAULT_MAP_SIZE: (i16, i16) = (127, 127);

pub struct GameOptions {
    pub size: (usize, usize),
    pub rule: LifeRule,
    pub algo: LifeAlgoSelect,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            size: (256, 256),
            rule: LifeRule::GOL,
            algo: LifeAlgoSelect::Basic,
        }
    }
}

impl GameOptions {
    pub fn create(&self) -> Life {
        let seed = 23317;
        let mut life = Life::new_rule(self.algo, self.size, self.rule);
        life.randomize(seed, true);
        life
        // match &self {
        //     GameOptions::New => Ok(Map::new_generate(DEFAULT_MAP_SIZE)),
        //     GameOptions::Level(level) => Ok(new_level(*level)),
        //     GameOptions::Continue => Map::load(),
        // }
    }
}

pub struct Gameplay {
    // ui: UiState,
    // popup: Option<Popup>,
    viewer: LifeViewer,
}

impl Gameplay {
    pub async fn new(_ctx: &mut Context, life: Box<Life>) -> Self {
        // let unlocked = map.metadata.unlocks;
        let gameplay = Gameplay {
            // ui: UiState::new(unlocked),
            // popup: None,
            viewer: LifeViewer::new(life),
        };

        // ctx.tileset.reset_camera(gameplay.map.grid.size_px());

        gameplay
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        let mouse_pos = input::mouse_position();

        let pos = self.viewer.screen_to_life_pos(mouse_pos);

        if let Some(chr) = input::get_char_pressed() {
            match chr {
                'q' => ctx.request_quit = true,
                ' ' => {
                    if self.viewer.update_speed == viewer::GAME_SPEED_1_PAUSED {
                        self.viewer.update_speed = viewer::GAME_SPEED_2_NORMAL;
                    } else {
                        self.viewer.update_speed = viewer::GAME_SPEED_1_PAUSED;
                    }
                }
                // self.viewer.update_speed = !view_self.viewer.update_speed,
                '1' => self.viewer.update_speed = viewer::GAME_SPEED_1_PAUSED,
                '2' => self.viewer.update_speed = viewer::GAME_SPEED_2_NORMAL,
                '3' => self.viewer.update_speed = viewer::GAME_SPEED_3_FAST,
                '4' => self.viewer.update_speed = viewer::GAME_SPEED_4_VERY_FAST,
                '5' => self.viewer.update_speed = viewer::GAME_SPEED_5_EXTREME,
                '6' => self.viewer.update_speed = viewer::GAME_SPEED_6_VERY_EXTREME,
                'g' => self.viewer.life.paste(
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
            self.viewer.life.insert(pos.unwrap(), Cell::new(1, 0)); //view_ctx.selected_faction));
        }
    }
}

fn draw_score(life: &Life, ctx: &Context) {
    let mut pos_y = 30.;
    let pos_x = macroquad::window::screen_width();
    let mut pops: Vec<(i16, u8)> = (0..life::FACTION_MAX)
        .filter_map(|faction| {
            let pop = life.get_pop(faction as u8);
            if pop > 0 {
                Some((pop, faction as u8))
            } else {
                None
            }
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
                color: viewer::faction_color(*faction),
                ..Default::default()
            },
        );

        pos_y += measure.height + 10.;
    }
}

impl Scene for Gameplay {
    fn update(&mut self, _ctx: &mut Context) {
        self.viewer.update();
    }

    fn draw(&mut self, ctx: &mut Context) {
        let size = self.viewer.life.size();
        self.viewer.resize_to_fit(
            size,
            (
                (macroquad::window::screen_width() - BORDER_SIZE * 2.),
                (macroquad::window::screen_height() - BORDER_SIZE * 2.),
            ),
        );
        self.viewer.set_pos((BORDER_SIZE, BORDER_SIZE));

        self.handle_input(ctx);

        self.viewer.draw();

        draw_score(&self.viewer.life, ctx);

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
