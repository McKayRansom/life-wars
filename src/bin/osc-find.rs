use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
};

use life_io::life::{iter::LifeIter, Life};

const HISTORY_SIZE: usize = 512;
const MAX_ITERS: usize = 2000;

pub struct LifeResult {
    age: usize,
    period: usize,
    #[allow(unused)]
    life: LifeIter,
}

fn run_to_stabilization(seed: u64) -> Option<LifeResult> {
    macroquad::rand::srand(seed);

    let mut life = LifeIter::new((16, 16));
    life.randomize();
    let mut life_history: VecDeque<u64> = VecDeque::new();
    let mut i: usize = 0;

    loop {
        life = life.update();

        let mut hasher = DefaultHasher::new();
        life.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(index) = life_history.iter().position(|i| i == &hash) {
            // println!("Seed {seed} Stabilized at i: {i} period: {index} ");
            return Some(LifeResult {
                age: i,
                period: index + 1,
                life,
            });
        }

        life_history.push_front(hash);
        if life_history.len() > HISTORY_SIZE {
            life_history.pop_back();
        }

        // next_frame().await;
        i += 1;

        if i > MAX_ITERS {
            // println!("Seed {seed} failed to stablize after {i} iters");
            return None;
        }
    }
}

// #[macroquad::main("Tron-IO")]
fn main() {
    println!("Life oscilator search...");

    let mut found_oscilators: Vec<usize> = Vec::new();

    // search the first 100 seeds
    for seed in 0..100000 {
        if let Some(res) = run_to_stabilization(seed) {
            if !found_oscilators.contains(&res.period) {
                found_oscilators.push(res.period);
                println!(
                    "Found oscilator {} seed: {seed} iter: {}",
                    res.period, res.age
                );
            }
        }
    }
}
