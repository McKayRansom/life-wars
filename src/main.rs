use life_io::Life;
use macroquad::{
    color::{self},
    input::get_char_pressed,
    shapes::draw_rectangle,
    time::get_time,
    window::{next_frame, screen_height, screen_width},
};

pub struct ViewContext {
    grid_size: f32,
    request_quit: bool,
    paused: bool,
    speed: f64,
}

fn draw_life(life: &Life, ctx: &ViewContext) {
    for (y, row) in life.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.state > 0 {
                let mut color = color::GREEN;
                if cell.state == 2 {
                    color.a = 0.75;
                } else if cell.state == 3 {
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
}

fn handle_input(_life: &Life, ctx: &mut ViewContext) {
    if let Some(chr) = get_char_pressed() {
        match chr {
            'q' => ctx.request_quit = true,
            ' ' => ctx.paused = !ctx.paused,
            _ => {}
        }
    }
}

#[macroquad::main("life-io")]
async fn main() {
    let seed = 3482;

    println!("Life viewer. Seed: {seed}");

    macroquad::rand::srand(seed);

    let mut life = Life::new((32, 32));

    let mut last_update = get_time();

    let mut ctx: ViewContext = ViewContext {
        grid_size: 0.,
        request_quit: false,
        paused: false,
        speed: 1. / 10.,
    };

    loop {
        if !ctx.paused && get_time() - last_update > ctx.speed {
            last_update = get_time();
            life = life.update();
        }

        ctx.grid_size =
            (screen_width() / life.grid[0].len() as f32).min(screen_height() / life.grid.len() as f32);
        handle_input(&life, &mut ctx);

        draw_life(&life, &ctx);

        next_frame().await;

        if ctx.request_quit {
            break;
        }
    }
}
