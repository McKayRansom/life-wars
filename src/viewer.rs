use life_io::life::Life;
use macroquad::{
    color,
    input::{self, KeyCode, is_key_down, mouse_position, mouse_wheel},
    time,
    window::{screen_height, screen_width},
};

use crate::context::Context;

pub struct LifeViewer {
    // pixels per life cell
    zoom: f32,
    // camera position in Life space
    camera: (f32, f32),

    last_map_update: f64,
    pub update_speed: f64,

    pub life: Box<Life>,
}

const MIN_ZOOM: f32 = 1.; // don't zoom in to more than 1 cell per pixel
const MAX_ZOOM: f32 = 16.;

const WASD_MOVE_SENSITIVITY: f32 = 20.;
const SCROLL_SENSITIVITY: f32 = 0.1;
const PLUS_MINUS_SENSITVITY: f32 = 0.8; // 20% zoom seems pretty standard (I.E. that is also what VSCode does)

pub const GAME_SPEED_1_PAUSED: f64 = 0.;
pub const GAME_SPEED_2_NORMAL: f64 = 1. / 8.;
pub const GAME_SPEED_3_FAST: f64 = 1. / 15.;
pub const GAME_SPEED_4_VERY_FAST: f64 = 1. / 30.;
pub const GAME_SPEED_5_EXTREME: f64 = 1. / 60.;
pub const GAME_SPEED_6_VERY_EXTREME: f64 = 1. / 120.;

impl LifeViewer {
    pub fn new(life: Box<Life>) -> Self {
        Self {
            zoom: 1.,
            camera: (0., 0.),
            last_map_update: 0.,
            update_speed: GAME_SPEED_2_NORMAL,
            life,
        }
    }

    pub fn set_pos(&mut self, pos: (f32, f32)) {
        self.camera = pos;
    }

    pub fn resize_to_fit(&mut self, size: (u16, u16), screen_size: (f32, f32)) {
        self.zoom = (screen_size.0 / size.0 as f32).min(screen_size.1 / size.1 as f32);
        self.camera.0 = (-(screen_size.0 - self.life_to_screen_scale(size.0)) / 2.) / self.zoom;
        self.camera.1 = (-(screen_size.1 - self.life_to_screen_scale(size.1)) / 2.) / self.zoom;
    }

    pub fn change_zoom(&mut self, amount: f32, center: (f32, f32)) {
        let new_zoom = self.zoom + amount;

        if new_zoom <= MIN_ZOOM || new_zoom >= MAX_ZOOM {
            return;
        }

        let old_screen_zoom = 1. / self.zoom;
        let new_screen_zoom = 1. / new_zoom;
        // self.camera.0 += screen_width() * (old_screen_zoom - new_screen_zoom) / 2.;
        // self.camera.1 += screen_height() * (old_screen_zoom - new_screen_zoom) / 2.;
        self.camera.0 += center.0 * (old_screen_zoom - new_screen_zoom);
        self.camera.1 += center.1 * (old_screen_zoom - new_screen_zoom);

        self.zoom += amount;
        // println!("Zoom + {} = {}", amount, self.zoom);
        // let self.zoom = self.zoom.round();
    }

    pub fn screen_to_life_pos(&self, screen_pos: (f32, f32)) -> Option<(u16, u16)> {
        // if screen_pos.0 < self.camera.0 || screen_pos.1 < self.camera.1 {
        //     return None;
        // }
        let pos: (u16, u16) = (
            (self.camera.0 + (screen_pos.0 / self.zoom)) as u16,
            (self.camera.1 + (screen_pos.1 / self.zoom)) as u16,
        );
        // let size = self.life.size();

        // if size.0 <= pos.0 || size.1 <= pos.1 {
        //     return None;
        // }
        Some(pos)
    }

    pub fn life_to_screen_pos(&self, (x, y): (u16, u16)) -> (f32, f32) {
        (
            (x as f32 - self.camera.0) * self.zoom,
            (y as f32 - self.camera.1) * self.zoom,
        )
    }

    pub fn life_to_screen_scale(&self, distance: u16) -> f32 {
        distance as f32 * self.zoom
    }

    pub fn update(&mut self) -> bool {
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
            true
        } else {
            false
        }
    }

    pub fn draw(&self) {
        let size = self.life.size();
        // macroquad::shapes::draw_rectangle_lines(
        macroquad::shapes::draw_rectangle(
            -self.camera.0 * self.zoom,
            -self.camera.1 * self.zoom,
            size.0 as f32 * self.zoom,
            size.1 as f32 * self.zoom,
            // 2.,
            // color::BLACK
            // color::WHITE,
            color::Color::from_hex(0x202020),
            // color::Color::from_hex(0x161616),
        );
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
                    (x as f32 - self.camera.0) * self.zoom,
                    (y as f32 - self.camera.1) * self.zoom,
                    self.zoom,
                    self.zoom,
                    color,
                );
            }
        }
    }

    pub fn handle_input(&mut self, ctx: &mut Context) -> bool {
        if is_key_down(KeyCode::W) {
            self.camera.1 -= WASD_MOVE_SENSITIVITY / self.zoom;
        }
        if is_key_down(KeyCode::A) {
            self.camera.0 -= WASD_MOVE_SENSITIVITY / self.zoom;
        }
        if is_key_down(KeyCode::S) {
            self.camera.1 += WASD_MOVE_SENSITIVITY / self.zoom;
        }
        if is_key_down(KeyCode::D) {
            self.camera.0 += WASD_MOVE_SENSITIVITY / self.zoom;
        }

        let new_mouse_wheel = mouse_wheel();
        if new_mouse_wheel.1 != 0. {
            self.change_zoom(SCROLL_SENSITIVITY * new_mouse_wheel.1, mouse_position());
        }
        // if scrol
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
                '=' => self.change_zoom(
                    PLUS_MINUS_SENSITVITY,
                    (screen_width() / 2., screen_height() / 2.),
                ),
                '-' => self.change_zoom(
                    -PLUS_MINUS_SENSITVITY,
                    (screen_width() / 2., screen_height() / 2.),
                ),
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

#[cfg(test)]
mod viewer_tests {

    use super::*;

    #[test]
    fn test_viewer_camera() {
        let mut viewer = LifeViewer::new(Box::new(Life::default()));

        viewer.zoom = 16.;

        assert_eq!(viewer.life_to_screen_scale(1), 16.);
        assert_eq!(viewer.life_to_screen_scale(2), 32.);

        assert_eq!(viewer.screen_to_life_pos((0., 0.)), Some((0, 0)));
        assert_eq!(viewer.screen_to_life_pos((16., 16.)), Some((1, 1)));
        assert_eq!(
            viewer.screen_to_life_pos((16. * 8., 16. * 8.)),
            Some((8, 8))
        );

        assert_eq!(viewer.life_to_screen_pos((0, 0)), (0., 0.));
        assert_eq!(viewer.life_to_screen_pos((1, 1)), (16., 16.));
    }

    #[test]
    fn test_viewer_camera_offset() {
        let mut viewer = LifeViewer::new(Box::new(Life::default()));

        viewer.zoom = 16.;
        viewer.camera = (8., 8.);

        assert_eq!(viewer.life_to_screen_scale(1), 16.);
        assert_eq!(viewer.life_to_screen_scale(2), 32.);

        assert_eq!(viewer.screen_to_life_pos((0., 0.)), Some((8, 8)));
        assert_eq!(viewer.screen_to_life_pos((16., 16.)), Some((9, 9)));
        assert_eq!(
            viewer.screen_to_life_pos((16. * 8., 16. * 8.)),
            Some((16, 16))
        );

        assert_eq!(viewer.life_to_screen_pos((0, 0)), (-128., -128.));
        assert_eq!(viewer.life_to_screen_pos((1, 1)), (-112., -112.));
    }
}
