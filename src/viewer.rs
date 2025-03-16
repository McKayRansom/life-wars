use life_io::life::Life;
use macroquad::{color, input, time};

use crate::context::Context;

pub struct LifeViewer {
    grid_size: f32,
    grid_pos: (f32, f32),

    last_map_update: f64,
    pub update_speed: f64,

    pub life: Box<Life>,
}

pub const GAME_SPEED_1_PAUSED: f64 = 0.;
pub const GAME_SPEED_2_NORMAL: f64 = 1. / 8.;
pub const GAME_SPEED_3_FAST: f64 = 1. / 15.;
pub const GAME_SPEED_4_VERY_FAST: f64 = 1. / 30.;
pub const GAME_SPEED_5_EXTREME: f64 = 1. / 60.;
pub const GAME_SPEED_6_VERY_EXTREME: f64 = 1. / 120.;

impl LifeViewer {
    pub fn new(life: Box<Life>) -> Self {
        Self {
            grid_size: 0.,
            grid_pos: (0., 0.),
            last_map_update: time::get_time(),
            update_speed: GAME_SPEED_2_NORMAL,
            life,
        }
    }

    pub fn set_pos(&mut self, pos: (f32, f32)) {
        self.grid_pos = pos;
    }

    pub fn fit_to_screen(&mut self) {}

    pub fn resize_to_fit(&mut self, size: (usize, usize), screen_size: (f32, f32)) {
        self.grid_size = (screen_size.0 / size.0 as f32).min(screen_size.1 / size.1 as f32);
        // self.ctx.grid_pos = (BORDER_SIZE, BORDER_SIZE);
    }

    pub fn screen_to_life_pos(&self, screen_pos: (f32, f32)) -> Option<(usize, usize)> {
        if screen_pos.0 < self.grid_pos.0 || screen_pos.1 < self.grid_pos.1 {
            return None;
        }
        let pos: (usize, usize) = (
            ((screen_pos.0 - self.grid_pos.0) / self.grid_size) as usize,
            ((screen_pos.1 - self.grid_pos.1) / self.grid_size) as usize,
        );
        let size = self.life.size();

        if size.0 <= pos.0 || size.1 <= pos.1 {
            return None;
        }
        Some(pos)
    }

    pub fn life_to_screen_pos(&self, (x, y): (usize, usize)) -> (f32, f32) {
        (
            self.grid_pos.0 + x as f32 * self.grid_size,
            self.grid_pos.1 + y as f32 * self.grid_size,
        )
    }

    pub fn update(&mut self) {
        if self.update_speed != GAME_SPEED_1_PAUSED
            && time::get_time() - self.last_map_update > self.update_speed
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

    pub fn draw(&self) {
        for (x, y, cell) in self.life.iter() {
            let state = cell.get_state();
            if state > 0 {
                let mut color = faction_color(cell.get_faction());
                if state == 2 {
                    color.a = 0.75;
                } else if state == 3 {
                    color.a = 0.5;
                }
                macroquad::shapes::draw_rectangle(
                    self.grid_pos.0 + x as f32 * self.grid_size,
                    self.grid_pos.1 + y as f32 * self.grid_size,
                    self.grid_size,
                    self.grid_size,
                    color,
                );
            }
        }
        let size = self.life.size();
        macroquad::shapes::draw_rectangle_lines(
            self.grid_pos.0,
            self.grid_pos.1,
            size.0 as f32 * self.grid_size,
            size.1 as f32 * self.grid_size,
            2.,
            color::WHITE,
        );
    }

    pub fn handle_input(&mut self, ctx: &mut Context) -> bool {
        if let Some(chr) = input::get_char_pressed() {
            match chr {
                'q' => ctx.request_quit = true,
                ' ' => {
                    if self.update_speed == GAME_SPEED_1_PAUSED {
                        self.update_speed = GAME_SPEED_2_NORMAL;
                    } else {
                        self.update_speed = GAME_SPEED_1_PAUSED;
                    }
                }
                // self.viewer.update_speed = !view_self.viewer.update_speed,
                '1' => self.update_speed = GAME_SPEED_1_PAUSED,
                '2' => self.update_speed = GAME_SPEED_2_NORMAL,
                '3' => self.update_speed = GAME_SPEED_3_FAST,
                '4' => self.update_speed = GAME_SPEED_4_VERY_FAST,
                '5' => self.update_speed = GAME_SPEED_5_EXTREME,
                '6' => self.update_speed = GAME_SPEED_6_VERY_EXTREME,
                _ => {
                    return false;
                }
            }
        } else {
            return false;
        }

        return true;
    }
}

pub fn faction_color(faction: u8) -> color::Color {
    match faction {
        0 => color::GREEN,
        1 => color::RED,
        2 => color::YELLOW,
        3 => color::BLUE,
        _ => color::WHITE,
    }
}
