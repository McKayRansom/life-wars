use macroquad::{
    camera::{set_camera, set_default_camera, Camera2D}, color, input::{self, is_key_down, mouse_position, mouse_wheel, KeyCode}, math::{vec2, Rect}, texture::{draw_texture, DrawTextureParams, Image, Texture2D}, time, window::{screen_height, screen_width}
};

use crate::life::{Cell, Life};

/*
 * Simple life viewer
 */
pub struct LifeViewer {
    camera: Camera2D,

    last_map_update: f64,
    pub update_speed: f64,

    pub life: Box<Life>,
    image: Image,
    texture: Option<Texture2D>,
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
        let mut image = Image::gen_image_color(life.size().0, life.size().1, color::BLACK);
        Self::update_image(&life, &mut image);

        // texture is boinked
        Self {
            camera: Camera2D {
                zoom: vec2(0.002, 0.002),
                ..Default::default()
            },
            last_map_update: 0.,
            update_speed: GAME_SPEED_3_FAST,
            life,
            image,
            texture: None, // cannot create textures in unit tests...
        }
    }

    pub fn new_fit_to_screen(life: Box<Life>) -> Self {
        let mut viewer = Self::new(life);
        viewer.fit_to_screen();
        viewer
    }

    pub fn set_pos(&mut self, pos: (f32, f32)) {
        self.camera.target = pos.into();
    }

    pub fn resize_to_fit(&mut self, size: (u16, u16), screen_size: (f32, f32)) {
        // let zoom = (screen_size.0 / size.0 as f32).min(screen_size.1 / size.1 as f32);
        // self.camera.zoom = (zoom, zoom).into();
        self.camera = {
            let rect = Rect { x: 0., y: 0., w: screen_size.0, h: screen_size.1 };
            let target = vec2(rect.x + rect.w / 2., rect.y + rect.h / 2.);

            Camera2D {
                target,
                zoom: vec2(1. / rect.w * 2., 1. / rect.h * 2.),
                offset: vec2(0., 0.),
                rotation: 0.,

                render_target: None,
                viewport: None,
            }
        }
        // self.camera.target.x = (-(screen_size.0 - self.life_to_screen_scale(size.0)) / 2.) / self.zoom;
        // self.camera.target.y = (-(screen_size.1 - self.life_to_screen_scale(size.1)) / 2.) / self.zoom;
    }

    pub fn fit_to_screen(&mut self) {
        self.resize_to_fit(self.life.size(), (screen_width(), screen_height()));
    }

    pub fn change_zoom(&mut self, amount: f32, center: (f32, f32)) {
        let new_zoom = self.camera.zoom.x + amount;

        if new_zoom <= MIN_ZOOM || new_zoom >= MAX_ZOOM {
            return;
        }

        let old_screen_zoom = 1. / self.camera.zoom.x;
        let new_screen_zoom = 1. / new_zoom;
        self.camera.target.x += center.0 * (old_screen_zoom - new_screen_zoom);
        self.camera.target.y += center.1 * (old_screen_zoom - new_screen_zoom);

        self.camera.zoom += amount;
    }

    pub fn screen_to_life_pos(&self, screen_pos: (f32, f32)) -> Option<(u16, u16)> {
        // let pos: (u16, u16) = (
        //     (self.camera.0 + (screen_pos.0 / self.zoom)) as u16,
        //     (self.camera.1 + (screen_pos.1 / self.zoom)) as u16,
        // );

        let life_vec = self.camera.screen_to_world(screen_pos.into());

        Some((life_vec.x as u16, life_vec.y as u16))
    }

    pub fn life_to_screen_pos(&self, (x, y): (u16, u16)) -> (f32, f32) {
        // (
            // (x as f32 - self.camera.0) * self.zoom,
            // (y as f32 - self.camera.1) * self.zoom,
        // )
        self.camera.world_to_screen((x as f32, y as f32).into()).into()
    }

    pub fn life_to_screen_scale(&self, distance: u16) -> f32 {
        distance as f32 * self.camera.zoom.x
    }

    pub fn update_image(life: &Life, image: &mut Image) {
        for (x, y, cell) in life.iter() {
            image.set_pixel(x as u32, y as u32, faction_color(cell));
        }
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
            Self::update_image(&self.life, &mut self.image);
            if let Some(texture) = &self.texture {
                texture.update(&self.image);
            }
            true
        } else {
            false
        }
    }

    /*
     * Draw to screen via rects
     * TODO: draw to image first, see https://github.com/not-fl3/macroquad/blob/master/examples/life.rs
     * Then we could only change pixels that we have to change
     */
    pub fn draw(&mut self) {
        // draw_im
        // let size = self.life.size();
        // set_camera(&dbg!(Camera2D::from_display_rect(Rect::new(0., screen_height(), screen_width(), -screen_height()))));
        set_camera(&self.camera);
        if self.texture.is_none() {
            self.texture = Some(Texture2D::from_image(&self.image));
        }
        draw_texture(
            self.texture.as_ref().unwrap(),
            0., // -self.camera.0 * self.zoom,
            0., // -self.camera.1 * self.zoom,
            color::WHITE,
        );
        set_default_camera();
        // macroquad::shapes::draw_rectangle_lines(
        // macroquad::shapes::draw_rectangle(
        //     -self.camera.0 * self.zoom,
        //     -self.camera.1 * self.zoom,
        //     size.0 as f32 * self.zoom,
        //     size.1 as f32 * self.zoom,
        //     // 2.,
        //     // color::BLACK
        //     // color::WHITE,
        //     color::Color::from_hex(0x202020),
        //     // color::Color::from_hex(0x161616),
        // );
        // for (x, y, cell) in self.life.iter() {
        //     let state = cell.get_state();
        //     if state > 0 {
        //         macroquad::shapes::draw_rectangle(
        //             (x as f32 - self.camera.0) * self.zoom,
        //             (y as f32 - self.camera.1) * self.zoom,
        //             self.zoom,
        //             self.zoom,
        //             faction_color(cell),
        //         );
        //     }
        // }
    }

    pub fn handle_input(&mut self) -> bool {
        if is_key_down(KeyCode::W) {
            self.camera.target.y -= WASD_MOVE_SENSITIVITY; // / self.zoom;
        }
        if is_key_down(KeyCode::A) {
            self.camera.target.x -= WASD_MOVE_SENSITIVITY; // / self.zoom;
        }
        if is_key_down(KeyCode::S) {
            self.camera.target.y += WASD_MOVE_SENSITIVITY; // / self.zoom;
        }
        if is_key_down(KeyCode::D) {
            self.camera.target.x += WASD_MOVE_SENSITIVITY; // / self.zoom;
        }

        let new_mouse_wheel = mouse_wheel();
        if new_mouse_wheel.1 != 0. {
            self.change_zoom(SCROLL_SENSITIVITY * new_mouse_wheel.1, mouse_position());
        }
        // if scrol
        if let Some(chr) = input::get_char_pressed() {
            match chr {
                // 'q' => ctx.request_quit = true,
                ' ' => {
                    if self.update_speed == GAME_SPEED_1_PAUSED {
                        self.update_speed = GAME_SPEED_2_NORMAL;
                    } else {
                        self.update_speed = GAME_SPEED_1_PAUSED;
                    }
                }
                '\t' => {
                    self.update_speed = GAME_SPEED_1_PAUSED;
                    self.life.update();
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

        true
    }
}

pub fn faction_color(cell: &Cell) -> color::Color {
    let state = cell.get_state();
    if state == 0 {
        // return color::BLACK;
        return color::Color::from_hex(0x202020);
    }
    let mut color = match cell.get_faction() {
        0 => color::GREEN,
        1 => color::RED,
        2 => color::YELLOW,
        3 => color::BLUE,
        _ => color::WHITE,
    };
    if state == 2 {
        color.a = 0.75;
    } else if state == 3 {
        color.a = 0.5;
    }
    color
}

/*
pub fn faction_color(faction: u8, state: u8) -> color::Color {
    match (faction, state) {
        (1, 1) => color::GREEN,
        (1, 2) => // color::LIME, //color::Color { r: 0.0, g: 0.5, b: 0.5, a: 1.0 },
     color::Color::new(0.00, 0.72, 0.5, 1.00),
        (1, 3) => color::DARKBLUE,
        (0, 1) => color::YELLOW,
        (0, 2) => color::ORANGE,
        (0, 3) => color::RED,
        _ => color::WHITE,
    }
}
*/

#[cfg(test)]
mod viewer_tests {

    use super::*;

    #[test]
    #[ignore = "Camera2D can't be used for Unit Tests"]
    fn test_viewer_camera() {
        let mut viewer = LifeViewer::new(Box::default());

        viewer.camera.zoom = (16., 16.).into();

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
    #[ignore = "Camera2D can't be used for Unit Tests"]
    fn test_viewer_camera_offset() {
        let mut viewer = LifeViewer::new(Box::default());

        viewer.camera.zoom = (16., 16.).into();
        viewer.camera.target = (8., 8.).into();

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
