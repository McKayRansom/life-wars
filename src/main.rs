use life_io::life::{iter_life, sparse::LifeSparse, Cell, Life};

use macroquad::{
    color::{self},
    input::{get_char_pressed, is_mouse_button_down, mouse_position},
    shapes::draw_rectangle,
    time::get_time,
    window::{next_frame, screen_height, screen_width},
};

pub struct ViewContext {
    grid_size: f32,
    request_quit: bool,
    paused: bool,
    speed: f64,
    selected_faction: u8,
}

fn draw_life(life: &dyn Life, ctx: &ViewContext) {
    for (x, y, cell) in iter_life(life) {
        let state = cell.get_state();
        if state > 0 {
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
            draw_rectangle(
                x as f32 * ctx.grid_size,
                y as f32 * ctx.grid_size,
                ctx.grid_size,
                ctx.grid_size,
                color,
            );
        }
    }
}

fn handle_input(life: &mut dyn Life, ctx: &mut ViewContext) {
    if let Some(chr) = get_char_pressed() {
        match chr {
            'q' => ctx.request_quit = true,
            ' ' => ctx.paused = !ctx.paused,
            '1' => ctx.selected_faction = 0,
            '2' => ctx.selected_faction = 1,
            '3' => ctx.selected_faction = 2,
            '4' => ctx.selected_faction = 3,
            _ => {}
        }
    }

    if is_mouse_button_down(macroquad::input::MouseButton::Left) {
        let mouse_pos = mouse_position();
        let pos: (usize, usize) = (
            (mouse_pos.0 / ctx.grid_size) as usize,
            (mouse_pos.1 / ctx.grid_size) as usize,
        );
        life.insert(pos, Cell::new(1, ctx.selected_faction));
    }
}

#[macroquad::main("life-io")]
async fn main() {
    let seed = 1234;

    println!("Life viewer. Seed: {seed}");


    let mut life = LifeSparse::new((256, 256));
    life.randomize(seed);

    let mut last_update = get_time();

    let mut ctx: ViewContext = ViewContext {
        grid_size: 0.,
        request_quit: false,
        paused: false,
        speed: 1. / 10.,
        selected_faction: 0,
    };

    loop {
        if !ctx.paused && get_time() - last_update > ctx.speed {
            last_update = get_time();
            life = life.update();
        }

        let size = life.size();
        ctx.grid_size = (screen_width() / size.0 as f32)
            .min(screen_height() / size.1 as f32);
        handle_input(&mut life, &mut ctx);

        draw_life(&life, &ctx);

        next_frame().await;

        if ctx.request_quit {
            break;
        }
    }
}
