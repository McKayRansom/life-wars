
#[derive(Clone, Debug)]
pub enum GameOptions {
    Random(u64),
    New,
    Continue,
}


pub enum EScene {
    Gameplay(Box<Life>),
    MainMenu,
    // LevelSelect,
}

use life_io::life::Life;

use crate::context::Context;

// pub mod credits;
pub mod gameplay;
pub mod main_menu;
// pub mod level_select;
// pub mod settings;

pub mod menu;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}