use super::Scene;
use crate::{context::Context, pattern_view::PatternLibViewer};

use macroquad::{
    color::{self, Color},
    input::{self, mouse_position},
    ui::root_ui,
};

use life_io::{
    life::{self, Cell, FACTION_MAX, Life, LifeAlgoSelect, LifeOptions, LifeRule, Pos, pos},
    viewer::{self, LifeViewer},
};

pub struct GameOptions {
    pub size: Pos,
    pub rule: LifeRule,
    pub algo: LifeAlgoSelect,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            size: pos(256, 256),
            rule: LifeRule::GOL,
            algo: LifeAlgoSelect::Basic,
        }
    }
}

impl GameOptions {
    pub fn create(&self) -> Life {
        let seed = 23317;
        let mut life = Life::new_ex(self.size, LifeOptions {
            rule: self.rule,
            algo: self.algo,
        });
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
        Self {
            // ui: UiState::new(unlocked),
            // popup: None,Gameplay
            viewer: LifeViewer::new_fit_to_screen(life),
            resources: [0; FACTION_MAX],
            pattern_view: PatternLibViewer::new(),
            ai_update_ticks: 0,
        }
    }

    fn handle_input(&mut self, _ctx: &mut Context) {
        if root_ui().is_mouse_over(mouse_position().into()) {
            return;
        }

        self.viewer.handle_input();
        // return;
        // }

        let mouse_pos = input::mouse_position();
        if let Some(pos) = self.viewer.screen_to_life_pos(mouse_pos) {
            if input::is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
                if let Some(pattern) = &self.pattern_view.selected_pattern {
                    // TODO: calc better cost...
                    let cost = pattern.life.get_pop(0);
                    if self.resources[0] >= cost {
                        self.resources[0] -= cost;
                        self.viewer.life.paste(
                            &pattern.life,
                            pos - pattern.life.size() / 2,
                            None,
                        );
                        self.viewer.redraw();
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

    fn draw_selected_pattern(&mut self) {
        if let Some(pattern_view) = &mut self.pattern_view.selected_pattern {
            let mouse_pos = input::mouse_position();
            if let Some(mouse_grid_pos) = self.viewer.screen_to_life_pos(mouse_pos) {
                // TODO: one could argue this should be centered instead of starting from the top left...
                let start_pos = self.viewer.life_to_screen_pos(mouse_grid_pos);
                // let pattern_size = pattern_view.size();

                pattern_view.zoom = self.viewer.zoom;
                pattern_view.color = Color::new(1., 1., 1., 0.5);
                pattern_view.screen_offset = (
                    start_pos.0
                        - pattern_view.life_to_screen_scale(pattern_view.life.size().x) / 2.,
                    start_pos.1
                        - pattern_view.life_to_screen_scale(pattern_view.life.size().y) / 2.,
                );

                pattern_view.draw();

                //     macroquad::shapes::draw_rectangle(
                //         start_pos.0,
                //         start_pos.1,
                //         self.viewer.life_to_screen_scale(pattern_size.0),
                //         self.viewer.life_to_screen_scale(pattern_size.1),
                //         color::Color {
                //             r: 1.,
                //             g: 1.,
                //             b: 1.,
                //             a: 0.6,
                //         },
                //     );
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
                color: viewer::faction_color(&Cell::new(1, *faction)),
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

                    // TODO: If player resources are below X and pop is below CELL_PER_RESOURCE, just eliminate them!
                    // if self.map.update() && self.map.metadata.is_level {
                    //     self.popup = Some(Popup::new(format!(
                    //         "Level {} completed!",
                    //         self.map.metadata.level_number
                    //     )));
                    // }
                }

                self.ai_update_ticks = 0;

                // Idea: Easy/Medium/Hard determins what the AI will spawn...
                // let bomber_life = new_life_from_rle(BOMBER_RLE);
                // MEAN: Steal our patterns!
                let rand_pattern_i =
                    macroquad::rand::rand() as usize % ctx.pattern_lib.patterns.len();
                let rand_pattern = &ctx.pattern_lib.patterns[rand_pattern_i];
                if rand_pattern.life.get_rule() != self.viewer.life.get_rule() {
                    return;
                }

                if self.resources[1] > rand_pattern.life.get_pop(0) {
                    self.resources[1] -= rand_pattern.life.get_pop(0);
                    let size = self.viewer.life.size();
                    let rand_x = macroquad::rand::rand() % (size.x as u32);
                    let rand_y = macroquad::rand::rand() % (size.y as u32) / 4;
                    self.viewer.life.paste(
                        &rand_pattern.life,
                        pos(rand_x as u16, rand_y as u16),
                        Some(1),
                    );
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
