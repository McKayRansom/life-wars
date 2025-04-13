
pub enum EScene {
    GameOptions,
    Gameplay(Box<Life>),
    MainMenu,
    Editor,
    // LevelSelect,
}

use life_io::life::Life;

use crate::context::Context;

// pub mod credits;
pub mod gameplay;
pub mod main_menu;
pub mod game_options;
pub mod editor;
// pub mod level_select;
// pub mod settings;


pub trait Scene {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}