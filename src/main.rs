use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
};

use life::Life;
use macroquad::{
    color::{self},
    shapes::draw_rectangle,
    window::next_frame,
};

pub mod life;

const GRID_SIZE: f32 = 2.;
const HISTORY_SIZE: usize = 32;

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

pub struct LifeResult {
    age: usize,
    period: usize,
    life: Life,
}

fn run_to_stabilization(seed: u64) -> LifeResult {
    macroquad::rand::srand(seed);

    let mut life = Life::new((32, 32));
    let mut life_history: VecDeque<u64> = VecDeque::new();
    let mut i: usize = 0;

    loop {
        life = life.update();

        let mut hasher = DefaultHasher::new();
        life.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(index) = life_history.iter().position(|i| i == &hash) {
            // println!("Seed {seed} Stabilized at i: {i} period: {index} life: {life}");
            return LifeResult {
                age: i,
                period: index,
                life,
            }
        }

        life_history.push_front(hash);
        if life_history.len() > HISTORY_SIZE {
            life_history.pop_back();
        }

        // next_frame().await;
        i += 1;
    }
}

// #[macroquad::main("Tron-IO")]
fn main() {
    println!("Life oscilator search...");

    let mut found_oscilators: Vec<usize> = Vec::new();

    // search the first 100 seeds
    for seed in 0..1000 {
        let res = run_to_stabilization(seed);
        if !found_oscilators.contains(&res.period) {
            found_oscilators.push(res.period);
            println!("Found oscilator {} seed: {seed} iter: {} life: {}", res.period, res.age, res.life);
        }
    }
}
