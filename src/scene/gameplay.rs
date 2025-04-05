use super::{Scene, popup::Popup};
use crate::{context::Context, pattern_view::PatternLibViewer};

use macroquad::{
    color::{self},
    input::{self},
};

use life_io::{
    life::{
        self, Cell, FACTION_MAX, Faction, Life, LifeAlgoSelect, LifeOptions, LifeRule, Pos, pos,
    },
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

type Resource = i16;
type Resources = [Resource; FACTION_MAX];

pub struct Gameplay {
    // ui: UiState,
    _popup: Option<Popup>,
    viewer: LifeViewer,
    resources: Resources,
    pattern_view: PatternLibViewer,
    ai_update_ticks: u32,
}

fn is_occupied(dst: &mut Life, start: Pos, area: Pos, faction: Faction) -> bool {
    dst.iter_area(&start, area)
        .any(|cell| cell.get_state() > 0 && cell.get_faction() != faction)
}

fn cost(life: &Life) -> i16 {
    // TODO: calc better cost...
    life.get_pop(0)
}

fn can_afford(cost: Resource, resources: &Resources, faction: Faction) -> bool {
    resources[faction as usize] >= cost
}

#[derive(Debug)]
pub enum PlaceError {
    Occupied,
    NotEnoughResources,
}

fn place_pattern(
    dst: &mut Life,
    src: &Life,
    pos: Pos,
    resources: &mut Resources,
    faction: Faction,
) -> Result<Resource, PlaceError> {
    if is_occupied(dst, pos, src.size(), faction) {
        return Err(PlaceError::Occupied);
    }

    let cost = cost(src);

    if !can_afford(cost, resources, faction) {
        return Err(PlaceError::NotEnoughResources);
    }

    resources[faction as usize] -= cost;
    dst.paste(src, pos, Some(faction));

    Ok(cost)
}

impl Gameplay {
    pub async fn new(_ctx: &mut Context, life: Box<Life>) -> Self {
        Self {
            // ui: UiState::new(unlocked),
            _popup: None,
            viewer: LifeViewer::new_fit_to_screen(life),
            resources: [0; FACTION_MAX],
            pattern_view: PatternLibViewer::new(),
            ai_update_ticks: 0,
        }
    }

    fn place_selected_pattern(&mut self, pos: Pos) {
        if let Some(pattern) = &self.pattern_view.selected_pattern {
            let sel_pat_life = pattern.get_life();
            self.viewer.edit_life(|life| {
                match place_pattern(
                    life,
                    sel_pat_life,
                    pos.saturating_sub(sel_pat_life.size() / 2),
                    &mut self.resources,
                    0,
                ) {
                    Ok(cost) => println!("subbing cost {cost}"),
                    Err(err) => println!("error: {err:?}"),
                }
            });
        }
    }

    fn handle_input(&mut self, ctx: &mut Context) -> Option<()> {
        self.viewer.handle_input(&mut ctx.view_context);

        let pos = self
            .viewer
            .screen_to_life_pos(ctx.view_context.mouse_pos?)?;

        if input::is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
            self.place_selected_pattern(pos);
        }

        match ctx.view_context.key_pressed.take()? {
            'v' => {
                self.place_selected_pattern(pos);
                // if let Some
                // if let Some(string) = clipboard_get() {
                //     println!("Pasting {string:?}");
                //     life.paste(&Life::new_life_from_rle(string.as_str()), pos)
                // } else {
                // println!("No clipboard!");
                // }
            },
            'r' => {
                if let Some(pattern) = &mut self.pattern_view.selected_pattern {
                    pattern.replace_life(Box::new(pattern.get_life().rotate()));
                }
            }
            chr => ctx.view_context.key_pressed = Some(chr),
        }
        Some(())
    }

    fn draw_selected_pattern(&mut self, ctx: &mut Context) -> Option<()> {
        self.viewer.draw_selected(
            self.viewer
                .screen_to_life_pos(ctx.view_context.mouse_pos?)?,
            self.pattern_view.selected_pattern.as_mut()?,
        );
        Some(())
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
pub const AI_CELL_PER_RESOURCE: i16 = 12;
// pub const AI_UPDATE_TICKS: u32 = 4;
pub const AI_UPDATE_TICKS: u32 = 64;

pub const MIN_RESOURCES: Resource = 16;

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        if self.viewer.update(&mut ctx.view_context) {
            self.ai_update_ticks += 1;

            if self.ai_update_ticks > AI_UPDATE_TICKS {
                for i in 0..FACTION_MAX {
                    let cell_per_resource = if i == 0 {
                        PLAYER_CELL_PER_RESOURCE
                    } else {
                        AI_CELL_PER_RESOURCE
                    };
                    self.resources[i] = self.resources[i].saturating_add(
                        self.viewer.get_life().get_pop(i as u8) / cell_per_resource,
                    );

                    // TODO: If player resources are below X and pop is below CELL_PER_RESOURCE, just eliminate them!
                    // if self.resources[0] < MIN_RESOURCES && self.viewer.get_life().get_pop(0) < MIN_RESOURCES {
                    //     // lost
                    //     self.popup = Some(Popup::new("Game Lost".into()));

                    // }
                    // if self.popup.is_none() {
                    //     if for i in
                    // }
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
                if rand_pattern.life.get_rule() != self.viewer.get_life().get_rule() {
                    return;
                }

                let size = self.viewer.get_life().size();
                let rand_x = macroquad::rand::rand() % (size.x as u32);
                let rand_y = macroquad::rand::rand() % (size.y as u32) / 4;

                self.viewer.edit_life(|life| {
                    let _ = place_pattern(
                        life,
                        &rand_pattern.life,
                        pos(rand_x as u16, rand_y as u16),
                        &mut self.resources,
                        1,
                    );
                });
            }
        }
        self.handle_input(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.viewer.draw();
        self.pattern_view
            .draw(ctx, self.viewer.get_life().get_rule());
        self.draw_selected_pattern(ctx);

        draw_score(self.viewer.get_life(), ctx);

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
