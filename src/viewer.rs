use macroquad::{
    color::{self, Color, WHITE},
    input::{self, KeyCode, is_key_down, mouse_position, mouse_wheel},
    math::{Vec2, vec2},
    texture::{DrawTextureParams, FilterMode, Image, Texture2D, draw_texture_ex},
    time,
    ui::root_ui,
    window::{screen_height, screen_width},
};

use crate::life::{Cell, Life, Pos};

#[derive(Default)]
pub struct ViewContext {
    pub key_pressed: Option<char>,
    pub mouse_pos: Option<Vec2>,
    pub screen_size: Vec2,
}

impl ViewContext {
    pub fn update(&mut self) {
        let mouse_pos: Vec2 = mouse_position().into();
        self.mouse_pos = if root_ui().is_mouse_over(mouse_pos) {
            None
        } else {
            Some(mouse_pos)
        };
        self.screen_size = (screen_width(), screen_height()).into();
        self.key_pressed = input::get_char_pressed();
    }
}

/*
 * Simple life viewer
 */
pub struct LifeViewer {
    life_offset: Vec2,
    pub screen_offset: Vec2,
    pub zoom: f32,

    last_map_update: f64,
    pub update_speed: f64,

    // life is private, any changes to life need to update the image and texture as well
    life: Box<Life>,
    image: Image,
    texture: Option<Texture2D>,
    pub color: Color,
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
        let mut image = Image::gen_image_color(life.size().x, life.size().y, color::BLACK);
        Self::update_image(&life, &mut image);

        Self {
            life_offset: vec2(0., 0.),
            screen_offset: vec2(0., 0.),
            zoom: 1.,
            last_map_update: 0.,
            update_speed: GAME_SPEED_3_FAST,
            life,
            image,
            texture: None, // cannot create textures in unit tests, Create it on the first draw call
            color: WHITE,
        }
    }

    pub fn get_life(&self) -> &Life {
        &self.life
    }

    pub fn paste_life(&mut self, other: &Life, center_pos: Pos, faction: Option<u8>) {
        self.life.paste(other, center_pos.saturating_sub(other.size() / 2), faction);
        self.redraw();
    }

    pub fn edit_life<T: FnMut(&mut Life) -> ()>(&mut self, mut edit_func: T) {
        edit_func(&mut self.life);
        self.redraw();
    }

    /// new_life can change size in this case!
    pub fn replace_life(&mut self, new_life: Box<Life>) {
        let image = Image::gen_image_color(new_life.size().x, new_life.size().y, color::BLACK);
        self.texture = None;
        self.life = new_life;
        self.image = image;
        self.redraw();
    }

    pub fn get_texture(&mut self) -> &Texture2D {
        if self.texture.is_none() {
            let texture = Texture2D::from_image(&self.image);
            texture.set_filter(FilterMode::Nearest);
            self.texture = Some(texture);
        }

        self.texture.as_ref().unwrap()
    }

    pub fn new_fit_to_screen(life: Box<Life>) -> Self {
        let mut viewer = Self::new(life);
        viewer.fit_to_screen();
        viewer
    }

    pub fn set_life_offset(&mut self, offset: Vec2) {
        self.life_offset = offset
    }

    pub fn resize_to_fit(&mut self, size: Pos, screen_size: Vec2) {
        self.zoom = (screen_size / size.as_vec2()).min_element();

        self.life_offset = (-(screen_size - self.life_to_screen_scale(size)) / 2.) / self.zoom;
    }

    pub fn fit_to_screen(&mut self) {
        self.resize_to_fit(self.life.size(), vec2(screen_width(), screen_height()));
    }

    pub fn change_zoom(&mut self, amount: f32, center: Vec2) {
        let new_zoom = self.zoom + amount;

        if new_zoom <= MIN_ZOOM || new_zoom >= MAX_ZOOM {
            return;
        }

        let old_screen_zoom = 1. / self.zoom;
        let new_screen_zoom = 1. / new_zoom;

        self.life_offset += center * (old_screen_zoom - new_screen_zoom);

        self.zoom += amount;
    }

    pub fn screen_to_life_pos(&self, screen_pos: Vec2) -> Option<Pos> {
        Pos::try_from_vec2(
            self.life_offset + (screen_pos / self.zoom),
            self.life.size(),
        )
    }

    pub fn life_to_screen_pos(&self, pos: Pos) -> Vec2 {
        self.screen_offset + (pos.as_vec2() - self.life_offset) * self.zoom
    }

    pub fn life_to_screen_scale(&self, distance: Pos) -> Vec2 {
        vec2(distance.x as f32 * self.zoom, distance.y as f32 * self.zoom)
    }

    pub fn update_image(life: &Life, image: &mut Image) {
        for (x, y, cell) in life.iter() {
            image.set_pixel(x as u32, y as u32, faction_color(cell));
        }
    }

    fn redraw(&mut self) {
        Self::update_image(&self.life, &mut self.image);
        if let Some(texture) = &self.texture {
            texture.update(&self.image);
        }
    }

    pub fn step(&mut self) {
        self.last_map_update = macroquad::time::get_time();
        self.life.update();
        self.redraw();
    }

    pub fn update(&mut self, _view_context: &mut ViewContext) -> bool {
        if self.update_speed != GAME_SPEED_1_PAUSED
            && time::get_time() - self.last_map_update > self.update_speed
        {
            self.step();
            true
        } else {
            false
        }
    }

    /*
     * Draw to screen via an cached image
     * See https://github.com/not-fl3/macroquad/blob/master/examples/life.rs
     */
    pub fn draw(&mut self) {
        let texture_pos = self.life_to_screen_pos(Pos::new(0, 0));
        let size = self.life_to_screen_scale(self.life.size());
        let color = self.color;
        let texture = self.get_texture();
        draw_texture_ex(
            texture,
            texture_pos.x,
            texture_pos.y,
            color,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }

    pub fn draw_selected(&self, center_pos: Pos, other: &mut LifeViewer) {
        other.zoom = self.zoom;
        other.life_offset = Vec2::ZERO;

        // TODO: change color based on if we can place!
        other.color = Color::new(1., 1., 1., 0.5);
        other.screen_offset = self.life_to_screen_pos(center_pos)
            - other.life_to_screen_scale(other.get_life().size() / 2);

        other.draw();
    }

    pub fn handle_input(&mut self, view_context: &mut ViewContext) {
        if is_key_down(KeyCode::W) {
            self.life_offset.y -= WASD_MOVE_SENSITIVITY / self.zoom;
        }
        if is_key_down(KeyCode::A) {
            self.life_offset.x -= WASD_MOVE_SENSITIVITY / self.zoom;
        }
        if is_key_down(KeyCode::S) {
            self.life_offset.y += WASD_MOVE_SENSITIVITY / self.zoom;
        }
        if is_key_down(KeyCode::D) {
            self.life_offset.x += WASD_MOVE_SENSITIVITY / self.zoom;
        }

        // only do these if we have the mouse
        if let Some(mouse_pos) = &view_context.mouse_pos {
            let new_mouse_wheel = mouse_wheel();
            if new_mouse_wheel.1 != 0. {
                self.change_zoom(SCROLL_SENSITIVITY * new_mouse_wheel.1, *mouse_pos);
            }
        }
        // only do these if no one else has used them yet
        if let Some(chr) = view_context.key_pressed.take() {
            match chr {
                ' ' => {
                    if self.update_speed == GAME_SPEED_1_PAUSED {
                        self.update_speed = GAME_SPEED_2_NORMAL;
                    } else {
                        self.update_speed = GAME_SPEED_1_PAUSED;
                    }
                }
                '\t' => {
                    self.update_speed = GAME_SPEED_1_PAUSED;
                    self.step();
                }
                // self.viewer.update_speed = !view_self.viewer.update_speed,
                // '1' => self.update_speed = GAME_SPEED_1_PAUSED,
                // '2' => self.update_speed = GAME_SPEED_2_NORMAL,
                // '3' => self.update_speed = GAME_SPEED_3_FAST,
                // '4' => self.update_speed = GAME_SPEED_4_VERY_FAST,
                // '5' => self.update_speed = GAME_SPEED_5_EXTREME,
                // '6' => self.update_speed = GAME_SPEED_6_VERY_EXTREME,
                '=' => self.change_zoom(PLUS_MINUS_SENSITVITY, view_context.screen_size / 2.),
                '-' => self.change_zoom(-PLUS_MINUS_SENSITVITY, view_context.screen_size / 2.),
                _ => view_context.key_pressed = Some(chr),
            }
        }
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

    // Now that we buffer the life in an Image before drawing, we have to do the color math manually
    // I'm not totally sure this is right, but it looks much better than before
    // for sure we can't set the Alpha to anything, changing the grey adjustment looks okay
    if state == 2 {
        // color.a = 0.75;
        color.r = color.r * 0.75; // + 0.078125 * 0.25;
        color.g = color.g * 0.75; // + 0.078125 * 0.25;
        color.b = color.b * 0.75; // + 0.078125 * 0.25;
    } else if state == 3 {
        // color.a = 0.5;
        color.r = color.r * 0.5; // + 0.078125 * 0.5;
        color.g = color.g * 0.5; // + 0.078125 * 0.5;
        color.b = color.b * 0.5; // + 0.078125 * 0.5;
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

    use crate::life::pos;

    use super::*;

    #[test]
    // #[ignore = "Camera2D can't be used for Unit Tests"]
    fn test_viewer_camera() {
        let mut viewer = LifeViewer::new(Box::default());

        viewer.zoom = 16.;

        assert_eq!(
            viewer.life_to_screen_scale((1, 2).into()),
            (16., 32.).into()
        );
        // assert_eq!(viewer.life_to_screen_scale(2), 32.);

        assert_eq!(viewer.screen_to_life_pos((0., 0.).into()), Some(pos(0, 0)));
        assert_eq!(
            viewer.screen_to_life_pos((16., 16.).into()),
            Some(pos(1, 1))
        );
        assert_eq!(
            viewer.screen_to_life_pos((16. * 8., 16. * 8.).into()),
            Some(pos(8, 8))
        );

        assert_eq!(viewer.life_to_screen_pos(pos(0, 0)), (0., 0.).into());
        assert_eq!(viewer.life_to_screen_pos(pos(1, 1)), (16., 16.).into());
    }

    #[test]
    // #[ignore = "Camera2D can't be used for Unit Tests"]
    fn test_viewer_camera_offset() {
        let mut viewer = LifeViewer::new(Box::default());

        viewer.zoom = 16.;
        viewer.life_offset = (8., 8.).into();

        assert_eq!(
            viewer.life_to_screen_scale((1, 2).into()),
            (16., 32.).into()
        );
        // assert_eq!(viewer.life_to_screen_scale(2), 32.);

        assert_eq!(viewer.screen_to_life_pos((0., 0.).into()), Some(pos(8, 8)));
        assert_eq!(viewer.screen_to_life_pos((16., 16.).into()), None);
        // assert_eq!(
        //     viewer.screen_to_life_pos((16. * 7., 16. * 7.).into()),
        //     Some(pos(7, 7))
        // );

        assert_eq!(viewer.life_to_screen_pos(pos(0, 0)), (-128., -128.).into());
        assert_eq!(viewer.life_to_screen_pos(pos(1, 1)), (-112., -112.).into());
    }
}
