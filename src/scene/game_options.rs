use life_io::{
    life::{Life, LifeOptions, LifeRule, pos, Pos},
    viewer::LifeViewer,
};
use macroquad::{
    math,
    ui::{self, hash, widgets},
    window::{self, screen_height, screen_width},
};

pub struct GameOptions {
    preview_life: LifeViewer,

    selected_rule: usize,
    selected_size: usize,
    selected_difficulty: usize,
}

impl GameOptions {
    pub fn new() -> Self {
        Self {
            preview_life: LifeViewer::new(Box::new(Life::new(pos(256, 256)))),

            selected_rule: 1,
            selected_size: 1,
            selected_difficulty: 0,
        }
    }

    pub fn create_life(&self) -> Box<Life> {
        let mut life = Box::new(Life::new_ex(
            GAME_SIZES[self.selected_size],
            LifeOptions {
                algo: life_io::life::LifeAlgoSelect::Basic,
                rule: *GAME_RULES[self.selected_rule],
            }, 
        ));
        // Should randomizing be part of the options?
        life.randomize(1234, true);
        life
    }
}

impl Default for GameOptions {
    fn default() -> Self {
        Self::new()
    }
}

const GAME_RULES: &[&LifeRule] = &[&LifeRule::GOL, &LifeRule::STAR_WARS];
const GAME_RULES_NAMES: &[&str] = &["Game of Life", "Star Wars"];

const GAME_SIZES: &[Pos] = &[pos(64, 64), pos(128, 128), pos(256, 256), pos(512, 512)];
const GAME_SIZES_NAMES: &[&str] = &["small", "medium", "large", "huge"];

const GAME_DIFFICULTY_NAMES: &[&str] = &["easy", "normal", "hard"];

impl super::Scene for GameOptions {
    fn update(&mut self, _ctx: &mut crate::context::Context) {
        self.preview_life.update();
    }

    fn draw(&mut self, ctx: &mut crate::context::Context) {
        self.preview_life.resize_to_fit(
            self.preview_life.life.size(),
            (screen_width(), screen_height()).into(),
        );
        self.preview_life.draw();

        let menu_width = window::screen_width() / 2.;
        let menu_height = window::screen_height() / 2.;

        widgets::Window::new(
            hash!(),
            math::vec2(
                window::screen_width() / 2.0 - (menu_width / 2.),
                window::screen_height() / 2.0 - (menu_height / 2.),
            ),
            math::vec2(menu_width, menu_height),
        )
        .titlebar(false)
        .movable(false)
        .ui(&mut ui::root_ui(), |ui| {
            ui.label(None, "Game Options");

            ui.combo_box(
                hash!(),
                "Rule",
                GAME_RULES_NAMES,
                Some(&mut self.selected_rule),
            );
            ui.combo_box(
                hash!(),
                "Size",
                GAME_SIZES_NAMES,
                Some(&mut self.selected_size),
            );
            ui.combo_box(
                hash!(),
                "Difficulty",
                GAME_DIFFICULTY_NAMES,
                Some(&mut self.selected_difficulty),
            );

            if ui.button(None, "Start") {
                ctx.switch_scene_to = Some(super::EScene::Gameplay(self.create_life()));
            }
        });

        if GAME_RULES[self.selected_rule] != self.preview_life.life.get_rule()
            || GAME_SIZES[self.selected_size] != self.preview_life.life.size()
        {
            self.preview_life.life = self.create_life();
        }
    }
}
