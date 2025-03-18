use life_io::life::{Life, LifeRule};
use macroquad::{
    math,
    ui::{self, hash, widgets},
    window::{self, screen_height, screen_width},
};

use crate::viewer::LifeViewer;

pub struct GameOptions {
    preview_life: LifeViewer,

    selected_rule: usize,
    selected_size: usize,
}

impl GameOptions {
    pub fn new() -> Self {
        Self {
            preview_life: LifeViewer::new(Box::new(Life::new(
                life_io::life::LifeAlgoSelect::Cached,
                (256, 256),
            ))),

            selected_rule: 0,
            selected_size: 0,
        }
    }

    pub fn create_life(&self) -> Box<Life> {
        let mut life = Box::new(Life::new_rule(
            life_io::life::LifeAlgoSelect::Basic,
            GAME_SIZES[self.selected_size],
            *GAME_RULES[self.selected_rule],
        ));
        // Should randomizing be part of the options?
        life.randomize(1234, true);
        life
    }
}

const GAME_RULES: &[&LifeRule] = &[&LifeRule::GOL, &LifeRule::STAR_WARS];
const GAME_RULES_NAMES: &[&str] = &["Game of Life", "Star Wars"];

const GAME_SIZES: &[(u16, u16)] = &[(64, 64), (128, 128), (256, 256), (512, 512)];
const GAME_SIZES_NAMES: &[&str] = &["small", "medium", "large", "huge"];

impl super::Scene for GameOptions {
    fn update(&mut self, _ctx: &mut crate::context::Context) {
        self.preview_life.update();
    }

    fn draw(&mut self, ctx: &mut crate::context::Context) {
        self.preview_life.resize_to_fit(
            self.preview_life.life.size(),
            (screen_width(), screen_height()),
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

            self.selected_rule = ui.combo_box(hash!(), "Rule", GAME_RULES_NAMES, None);
            self.selected_size = ui.combo_box(hash!(), "Size", GAME_SIZES_NAMES, None);

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
