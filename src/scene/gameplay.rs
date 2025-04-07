use super::{Scene, popup::Popup};
use crate::{
    context::Context,
    default_patterns::{PATTERN_MAX_COUNT, PATTERN_TIMES, PLAYER_PATTERNS},
    skin::WINDOW_COLOR,
};

use macroquad::{
    color::{self, Color},
    input::{self, is_mouse_button_down},
    math::{Rect, Vec2},
    shapes::draw_rectangle,
    text::draw_text,
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

#[derive(Default)]
pub struct PatternCount {
    timer: usize,
    count: usize,
}

impl PatternCount {
    pub fn update(&mut self, index: usize) {
        if self.timer == 0 {
            self.count += 1;
            self.timer = PATTERN_TIMES[index];
        }
        self.timer -= 1;
    }
}

type PatternCounts = [[PatternCount; PATTERN_MAX_COUNT]; FACTION_MAX];

pub struct Gameplay {
    // ui: UiState,
    _popup: Option<Popup>,
    viewer: LifeViewer,
    resources: Resources,
    pattern_times: PatternCounts,
    // pattern_view: PatternLibViewer,
    selected_pattern: Option<usize>,
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

fn can_afford(_cost: Resource, resources: &mut PatternCount, _faction: Faction) -> bool {
    resources.count > 0
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
    resources: &mut PatternCount,
    faction: Faction,
) -> Result<Resource, PlaceError> {
    if is_occupied(dst, pos, src.size(), faction) {
        return Err(PlaceError::Occupied);
    }

    let cost = cost(src);

    if !can_afford(cost, resources, faction) {
        return Err(PlaceError::NotEnoughResources);
    }

    // resources[faction as usize] -= cost;
    resources.count -= 1;
    dst.paste(src, pos, Some(faction));

    Ok(cost)
}

impl Gameplay {
    pub async fn new(_ctx: &mut Context, life: Box<Life>) -> Self {
        Self {
            // ui: UiState::new(unlocked),
            _popup: None,
            viewer: LifeViewer::new_fit_to_screen(life),
            pattern_times: Default::default(),
            resources: [0; FACTION_MAX],
            // pattern_view: PatternLibViewer::new(),
            selected_pattern: None,
            ai_update_ticks: 0,
        }
    }

    fn place_selected_pattern(&mut self, pos: Pos, ctx: &Context) {
        if let Some(index) = &self.selected_pattern {
            let sel_pat_life = &ctx.default_patterns[*index].get_life();
            self.viewer.edit_life(|life| {
                match place_pattern(
                    life,
                    sel_pat_life,
                    pos.saturating_sub(sel_pat_life.size() / 2),
                    &mut self.pattern_times[0][*index],
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
            self.place_selected_pattern(pos, ctx);
        }

        match ctx.view_context.key_pressed.take()? {
            'v' => {
                self.place_selected_pattern(pos, ctx);
                // if let Some
                // if let Some(string) = clipboard_get() {
                //     println!("Pasting {string:?}");
                //     life.paste(&Life::new_life_from_rle(string.as_str()), pos)
                // } else {
                // println!("No clipboard!");
                // }
            }
            // 'r' => {
            //     if let Some(pattern) = &mut self.pattern_view.selected_pattern {
            //         pattern.replace_life(Box::new(pattern.get_life().rotate()));
            //     }
            // }
            chr => ctx.view_context.key_pressed = Some(chr),
        }
        Some(())
    }

    fn draw_selected_pattern(&mut self, ctx: &mut Context) -> Option<()> {
        if let Some(index) = self.selected_pattern {
            self.viewer.draw_selected(
                self.viewer
                    .screen_to_life_pos(ctx.view_context.mouse_pos?)?,
                &mut ctx.default_patterns[index],
            );
        }
        Some(())
    }

    const TOOLBAR_OUTER_SIZE: f32 = 50.;
    const TOOLBAR_INNER_SIZE: f32 = 40.;
    const TOOLBAR_PAD: f32 = 5.;

    fn draw_pat_toolbar(&mut self, ctx: &mut Context) {
        let mut toolbar_pos: Vec2 = Vec2::new(
            ctx.view_context.screen_size.x / 2.
                - Self::TOOLBAR_OUTER_SIZE * PLAYER_PATTERNS.len() as f32 / 2.,
            ctx.view_context.screen_size.y - Self::TOOLBAR_OUTER_SIZE,
        );
        draw_rectangle(
            toolbar_pos.x,
            toolbar_pos.y,
            Self::TOOLBAR_OUTER_SIZE * PLAYER_PATTERNS.len() as f32,
            Self::TOOLBAR_OUTER_SIZE,
            WINDOW_COLOR,
        );
        for (i, pat) in self.pattern_times[0].iter().enumerate() {
            let percent: f32 = pat.timer as f32 / PATTERN_TIMES[i] as f32;
            let mut color = if self.selected_pattern != Some(i) {
                color::WHITE
            } else {
                color::GREEN
            };
            color.a = 0.4;
            let size = ctx.default_patterns[i].get_life().size();
            ctx.default_patterns[i].resize_to_fit(
                size,
                Vec2::new(Self::TOOLBAR_INNER_SIZE, Self::TOOLBAR_INNER_SIZE),
            );
            ctx.default_patterns[i].screen_offset =
                toolbar_pos + Vec2::new(Self::TOOLBAR_PAD, Self::TOOLBAR_PAD);
            ctx.default_patterns[i].draw();
            draw_rectangle(
                toolbar_pos.x + Self::TOOLBAR_PAD,
                toolbar_pos.y + Self::TOOLBAR_PAD,
                Self::TOOLBAR_INNER_SIZE,
                Self::TOOLBAR_INNER_SIZE * percent,
                color,
            );
            let sel_rect = Rect::new(
                toolbar_pos.x,
                toolbar_pos.y,
                Self::TOOLBAR_OUTER_SIZE,
                Self::TOOLBAR_OUTER_SIZE,
            );
            if let Some(mouse_pos) = ctx.view_context.mouse_pos {
                if sel_rect.contains(mouse_pos) {
                    draw_rectangle(
                        toolbar_pos.x,
                        toolbar_pos.y,
                        Self::TOOLBAR_OUTER_SIZE,
                        Self::TOOLBAR_OUTER_SIZE,
                        Color::new(1., 1., 1., 0.3),
                    );
                    ctx.view_context.mouse_pos = None;
                    if is_mouse_button_down(input::MouseButton::Left) {
                        self.selected_pattern = Some(i);
                    }
                }
            }
            draw_text(
                pat.count.to_string().as_str(),
                toolbar_pos.x,
                toolbar_pos.y,
                10.,
                color::WHITE,
            );
            toolbar_pos.x += Self::TOOLBAR_OUTER_SIZE;
        }
    }
}

const SCORE_WIDTH: f32 = 200.;
const SCORE_HEIGHT: f32 = 50.;
const SCORE_PADDING: f32 = 10.;

fn draw_score(life: &Life, ctx: &Context) {
    let mut total_pop: i16 = 0;
    let mut pops: Vec<(i16, u8)> = (0..life::FACTION_MAX)
        .filter_map(|faction| {
            let pop = life.get_pop(faction as u8);
            total_pop += pop;
            if pop > 0 {
                Some((pop, faction as u8))
            } else {
                None
            }
        })
        .collect();
    pops.sort();

    let mut score_pos = Vec2::new(ctx.view_context.screen_size.x / 2. - SCORE_WIDTH / 2., 0.);

    draw_rectangle(
        score_pos.x,
        score_pos.y,
        SCORE_WIDTH,
        SCORE_HEIGHT,
        WINDOW_COLOR,
    );

    for (pop, faction) in pops.iter().rev() {
        let width = *pop as f32 / total_pop as f32 * (SCORE_WIDTH - SCORE_PADDING * 2.);
        // width = width.round();
        draw_rectangle(
            score_pos.x + SCORE_PADDING,
            score_pos.y + SCORE_PADDING,
            width,
            SCORE_HEIGHT - SCORE_PADDING * 2.,
            viewer::faction_color(&Cell::new(1, *faction)),
        );
        // let faction_text = format!("Team {faction}: {pop}");
        // let measure = macroquad::text::measure_text(faction_text.as_str(), Some(&ctx.font), 40, 1.);
        // macroquad::text::draw_text_ex(
        //     faction_text.as_str(),
        //     pos_x - measure.width - 10.,
        //     pos_y,
        //     macroquad::text::TextParams {
        //         font: Some(&ctx.font),
        //         font_size: 40,
        //         color: viewer::faction_color(&Cell::new(1, *faction)),
        //         ..Default::default()
        //     },
        // );

        score_pos.x += width;
    }
}

pub const PLAYER_CELL_PER_RESOURCE: i16 = 64;
pub const AI_CELL_PER_RESOURCE: i16 = 12;
// pub const AI_UPDATE_TICKS: u32 = 4;
pub const AI_UPDATE_TICKS: u32 = 4;

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

                    for (j, pat_time) in self.pattern_times[i].iter_mut().enumerate() {
                        pat_time.update(j);
                    }

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
                let rand_pattern_i = macroquad::rand::rand() as usize % PATTERN_MAX_COUNT;
                let rand_pattern = &ctx.default_patterns[rand_pattern_i];
                // if rand_pattern.get_lif.get_rule() != self.viewer.get_life().get_rule() {
                //     return;
                // }

                let size = self.viewer.get_life().size();
                let rand_x = macroquad::rand::rand() % (size.x as u32);
                let rand_y = macroquad::rand::rand() % (size.y as u32) / 4;

                self.viewer.edit_life(|life| {
                    let _ = place_pattern(
                        life,
                        rand_pattern.get_life(),
                        pos(rand_x as u16, rand_y as u16),
                        &mut self.pattern_times[1][rand_pattern_i],
                        1,
                    );
                });
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.viewer.draw();
        // self.pattern_view
        // .draw(ctx, self.viewer.get_life().get_rule());
        self.draw_pat_toolbar(ctx);

        self.draw_selected_pattern(ctx);
        self.handle_input(ctx);

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

        // macroquad::text::draw_text_ex(
        //     format!("Resources: {}", self.resources[0]).as_str(),
        //     10.,
        //     macroquad::window::screen_height() - 20.,
        //     macroquad::text::TextParams {
        //         font: Some(&ctx.font),
        //         font_size: 40,
        //         color: color::GREEN,
        //         ..Default::default()
        //     },
        // );

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
