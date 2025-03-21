use super::Scene;
use crate::{
    context::Context,
    pattern_view::PatternLibViewer,
    viewer::{self, LifeViewer},
};

use macroquad::{
    color,
    input::{self, mouse_position},
    ui::root_ui,
    window::{screen_height, screen_width},
};

use life_io::life::{self, FACTION_MAX, Life, LifeAlgoSelect, LifeRule};

pub struct GameOptions {
    pub size: (u16, u16),
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
    resources: [i16; FACTION_MAX],
    pattern_view: PatternLibViewer,
    ai_update_ticks: u32,
}

impl Gameplay {
    pub async fn new(_ctx: &mut Context, life: Box<Life>) -> Self {
        // let unlocked = map.metadata.unlocks;
        let mut gameplay = Gameplay {
            // ui: UiState::new(unlocked),
            // popup: None,
            viewer: LifeViewer::new(life),
            resources: [0; FACTION_MAX],
            pattern_view: PatternLibViewer::new(),
            ai_update_ticks: 0,
        };

        gameplay.viewer.resize_to_fit(
            gameplay.viewer.life.size(),
            (screen_width(), screen_height()),
        );

        gameplay
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        if root_ui().is_mouse_over(mouse_position().into()) {
            return;
        }

        self.viewer.handle_input(ctx);
        // return;
        // }

        let mouse_pos = input::mouse_position();
        if let Some(pos) = self.viewer.screen_to_life_pos(mouse_pos) {
            if input::is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
                if let Some(pattern) = &self.pattern_view.selected_pattern {
                    // TODO: calc better cost...
                    let cost = pattern.get_pop(0);
                    if self.resources[0] >= cost {
                        self.resources[0] -= cost;
                        self.viewer.life.paste(pattern, pos, None);
                        println!("Subing {cost} from");
                    } else {
                        // TODO: UI somewhere??
                        println!("NOT ENOUGH RESOURCES");
                    }
                }
            }
        }

        // if let Some(chr) = input::get_char_pressed() {
        //     match chr {
        //         'q' => ctx.request_quit = true,
        //         'g' => self.viewer.life.paste(
        //             &Life::new_life_from_rle(life_io::life::GOSPER_RLE),
        //             pos.unwrap(),
        //         ),
        //         'p' => {
        //             // if let Some(string) = clipboard_get() {
        //             //     println!("Pasting {string:?}");
        //             //     life.paste(&Life::new_life_from_rle(string.as_str()), pos)
        //             // } else {
        //             println!("No clipboard!");
        //             // }
        //         }
        //         _ => {}
        //     }
        // }
    }

    fn draw_selected_pattern(&self) {
        if let Some(pattern) = &self.pattern_view.selected_pattern {
            let mouse_pos = input::mouse_position();
            if let Some(mouse_grid_pos) = self.viewer.screen_to_life_pos(mouse_pos) {
                // TODO: one could argue this should be centered instead of starting from the top left...
                let start_pos = self.viewer.life_to_screen_pos(mouse_grid_pos);
                let pattern_size = pattern.size();
                macroquad::shapes::draw_rectangle(
                    start_pos.0,
                    start_pos.1,
                    self.viewer.life_to_screen_scale(pattern_size.0),
                    self.viewer.life_to_screen_scale(pattern_size.1),
                    color::Color {
                        r: 1.,
                        g: 1.,
                        b: 1.,
                        a: 0.6,
                    },
                );
            }
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

pub const PLAYER_CELL_PER_RESOURCE: i16 = 64;
pub const AI_CELL_PER_RESOURCE: i16 = 2;
// pub const AI_UPDATE_TICKS: u32 = 4;
pub const AI_UPDATE_TICKS: u32 = 16;

// TODO: This bomber is great but it's facing left
// const BOMBER_RLE: &str = "\
// #N bomber
// x = 16, y = 10, rule = B2/S345/4
// 3$7.A$6.B.B$4.3AC2A$4.3ACA.CB.C$5.A.BA.CBA2$!";

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        if self.viewer.update() {
            self.ai_update_ticks += 1;

            if self.ai_update_ticks > AI_UPDATE_TICKS {
                for i in 0..FACTION_MAX {
                    let cell_per_resource = if i == 0 {
                        PLAYER_CELL_PER_RESOURCE
                    } else {
                        AI_CELL_PER_RESOURCE
                    };
                    self.resources[i] = self.resources[i]
                        .saturating_add(self.viewer.life.get_pop(i as u8) / cell_per_resource)
                }

                self.ai_update_ticks = 0;

                // Idea: Easy/Medium/Hard determins what the AI will spawn...
                // let bomber_life = new_life_from_rle(BOMBER_RLE);
                // MEAN: Steal our patterns!
                let rand_pattern_i = macroquad::rand::rand() as usize % ctx.pattern_lib.patterns.len();
                let rand_patter= &ctx.pattern_lib.patterns[rand_pattern_i];
                if rand_patter.get_rule() != self.viewer.life.get_rule() {
                    return;
                }

                let bomber_life = rand_patter;
                
                // let rand_

                if self.resources[1] > bomber_life.get_pop(0) {
                    self.resources[1] -= bomber_life.get_pop(0);
                    let size = self.viewer.life.size();
                    let rand_x = macroquad::rand::rand() % (size.0 as u32);
                    let rand_y = macroquad::rand::rand() % (size.1 as u32) / 4;
                    self.viewer
                        .life
                        .paste(bomber_life, (rand_x as u16, rand_y as u16), Some(1));
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        // let size = self.viewer.life.size();
        // self.viewer.resize_to_fit(
        //     size,
        //     (
        //         (macroquad::window::screen_width() - BORDER_SIZE * 2.),
        //         (macroquad::window::screen_height() - BORDER_SIZE * 2.),
        //     ),
        // );
        // self.viewer.set_pos((BORDER_SIZE, BORDER_SIZE));

        self.handle_input(ctx);

        self.viewer.draw();

        self.pattern_view.draw(ctx, self.viewer.life.get_rule());

        self.draw_selected_pattern();

        draw_score(&self.viewer.life, ctx);

        macroquad::text::draw_text_ex(
            format!("Battle in progress... speed: {}", self.viewer.update_speed).as_str(),
            10.,
            20.,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size: 40,
                color: color::GREEN,
                ..Default::default()
            },
        );

        macroquad::text::draw_text_ex(
            format!("Resources: {}", self.resources[0]).as_str(),
            10.,
            macroquad::window::screen_height() - 20.,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size: 40,
                color: color::GREEN,
                ..Default::default()
            },
        );

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

// const BORDER_SIZE: f32 = 40.;
