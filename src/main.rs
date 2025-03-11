
use life_io::Life;
use macroquad::{
    color::{self}, shapes::draw_rectangle, time::get_time, window::next_frame
};

const GRID_SIZE: f32 = 2.;

fn draw_life(life: &Life) {
    for (y, row) in life.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.state > 0 {
                draw_rectangle(
                    x as f32 * GRID_SIZE,
                    y as f32 * GRID_SIZE,
                    GRID_SIZE,
                    GRID_SIZE,
                    color::GREEN,
                );
            }
        }
    }
}

#[macroquad::main("life-io")]
async fn main() {
    let seed = 95;

    println!("Life viewer. Seed: {seed}");

    macroquad::rand::srand(seed);

    let mut life = Life::new((64, 64));

    let mut last_update = get_time();
    let speed: f64 = 1./10.;

    loop {
        if get_time() - last_update > speed {
            last_update = get_time();
            life = life.update();
        }

        draw_life(&life);

        next_frame().await;
    }
}
