use life_io::life::Life;
use macroquad::color;

pub struct ViewContext {
    grid_size: f32,
    grid_pos: (f32, f32),
}

impl ViewContext {
    pub fn new() -> Self {
        Self {
            grid_size: 0.,
            grid_pos: (0., 0.),
        }
    }

    pub fn set_pos(&mut self, pos: (f32, f32)) {
        self.grid_pos = pos;
    }

    pub fn resize_to_fit(&mut self, size: (usize, usize), screen_size: (f32, f32)) {
        self.grid_size = (screen_size.0 / size.0 as f32).min(screen_size.1 / size.1 as f32);
        // self.ctx.grid_pos = (BORDER_SIZE, BORDER_SIZE);
    }

    pub fn screen_to_life_pos(&self, screen_pos: (f32, f32)) -> Option<(usize, usize)> {
        if screen_pos.0 < self.grid_pos.0 || screen_pos.1 < self.grid_pos.1 {
            // TODO: CHECK out of bounds at the end of grid!
            return None;
        }
        let pos: (usize, usize) = (
            ((screen_pos.0 - self.grid_pos.0) / self.grid_size) as usize,
            ((screen_pos.1 - self.grid_pos.1) / self.grid_size) as usize,
        );
        Some(pos)
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

pub fn draw_life(life: &Life, ctx: &ViewContext) {
    for (x, y, cell) in life.iter() {
        let state = cell.get_state();
        if state > 0 {
            let mut color = faction_color(cell.get_faction());
            if state == 2 {
                color.a = 0.75;
            } else if state == 3 {
                color.a = 0.5;
            }
            macroquad::shapes::draw_rectangle(
                ctx.grid_pos.0 + x as f32 * ctx.grid_size,
                ctx.grid_pos.1 + y as f32 * ctx.grid_size,
                ctx.grid_size,
                ctx.grid_size,
                color,
            );
        }
    }
    let size = life.size();
    macroquad::shapes::draw_rectangle_lines(
        ctx.grid_pos.0,
        ctx.grid_pos.1,
        size.0 as f32 * ctx.grid_size,
        size.1 as f32 * ctx.grid_size,
        2.,
        color::WHITE,
    );
}
