use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hasher},
    time::Instant,
};

use life_io::life::{life_to_plaintext, rand::rand_life, Life};

const HISTORY_SIZE: usize = 512;
const MAX_ITERS: usize = 2000;

pub struct LifeResult {
    age: usize,
    period: usize,
    #[allow(unused)]
    life: Life,
}

fn run_to_stabilization(seed: u64) -> Option<LifeResult> {
    let mut life = Life::new(life_io::life::LifeAlgoSelect::Cached, (33, 33));
    rand_life(&mut life, (8, 8), (17, 17), seed, Some(life_io::life::rand::RandSymmetry::C4_1));
    let mut life_history: VecDeque<u64> = VecDeque::new();
    let mut i: usize = 0;

    // println!("{life}");
    // assert!(false);

    // pre-update to reduce hashing
    for _ in 0..50 {
        life.update();
        i += 1;
    }

    loop {
        life.update();

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

/*
 * Basic soup search
 * Enhancements: 
 * - Find patterns not whole grids
 * - Find spaceships somehow
 * - Experiment with sizes and such
 */
fn main() {
    println!("Life oscilator search...");

    let now = Instant::now();

    let mut found_oscilators: Vec<usize> = Vec::new();

    for seed in 0..100000 {
        if let Some(res) = run_to_stabilization(seed) {
            // if !found_oscilators.contains(&res.period) {
            if res.period > 6 {
                found_oscilators.push(res.period);
                println!(
                    "Found oscilator {} seed: {seed} iter: {} str: {}",
                    res.period,
                    res.age,
                    life_to_plaintext(&res.life)
                );
            }
        }
    }

    let elapsed = now.elapsed();

    println!(
        "Finished in : {:.1?} found {}",
        elapsed,
        found_oscilators.len()
    );
}
