use life::Life;
use macroquad::{
    color::{self},
    shapes::draw_rectangle,
    window::next_frame,
};

pub mod life;

const GRID_SIZE: f32 = 2.;

#[macroquad::main("Tron-IO")]
async fn main() {
    println!("Hello, world!");

    let mut life = Life::new((512, 512));

    loop {
        life = life.update();

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

        next_frame().await;
    }
}
