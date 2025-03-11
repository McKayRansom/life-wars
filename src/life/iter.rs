use serde::{Deserialize, Serialize};

use super::{Life, state_update};

#[derive(PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct LifeIter {
    pub grid: Vec<Vec<u8>>,
}

impl Life for LifeIter {
    fn size(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn get(&self, pos: (usize, usize)) -> Option<&u8> {
        self.grid
            .get(pos.1)
            .map(|thing| thing.get(pos.0))
            .unwrap_or(None)
    }

    fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut u8> {
        self.grid
            .get_mut(pos.1)
            .map(|thing| thing.get_mut(pos.0))
            .unwrap_or(None)
    }
}

impl LifeIter {
    pub fn new(dim: (usize, usize)) -> Self {
        Self {
            grid: vec![vec![0; dim.0]; dim.1],
        }
    }

    // This brute force was bench_256 at 341uS  instead of 289 for iter version...
    // pub fn neighbors(&self, pos: (usize, usize)) -> u8 {
    //     let mut neighbors: u8 = 0;
    //     if pos.0 > 0 {
    //         if pos.1 > 0 {
    //             neighbors += self.get((pos.0 - 1, pos.1 - 1)).unwrap_or(&0);
    //         }
    //         neighbors += self.get((pos.0 - 1, pos.1 + 1)).unwrap_or(&0);
    //         neighbors += self.get((pos.0 - 1, pos.1)).unwrap_or(&0);
    //     }
    //     if pos.1 > 0 {
    //         neighbors += self.get((pos.0, pos.1 - 1)).unwrap_or(&0);
    //         neighbors += self.get((pos.0 + 1, pos.1 - 1)).unwrap_or(&0);
    //     }

    //     neighbors += self.get((pos.0 + 1, pos.1)).unwrap_or(&0);
    //     neighbors += self.get((pos.0 + 1, pos.1 + 1)).unwrap_or(&0);
    //     neighbors += self.get((pos.0, pos.1 + 1)).unwrap_or(&0);

    //     neighbors
    // }
    
    // This seemingly stupid iterator version is somehow faster?
    pub fn neighbors(&self, pos: (usize, usize)) -> u8 {
        (-1..2)
            .map(|y: i32| {
                self.grid
                    .get((pos.1 as i32 + y) as usize)
                    .map(|row| {
                        (-1..2)
                            .map(|x: i32| {
                                if x == 0 && y == 0 {
                                    0
                                } else {
                                    row.get((pos.0 as i32 + x) as usize)
                                        .map(|state| if *state == 1 { 1 } else { 0 })
                                        .unwrap_or(0)
                                }
                            })
                            .sum()
                    })
                    .unwrap_or(0)
            })
            .sum()
    }

    pub fn update(&self) -> Self {
        Self {
            grid: self
                .grid
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, cell)| state_update(*cell, self.neighbors((x, y))))
                        .collect()
                })
                .collect(),
        }
    }
}

impl From<&str> for LifeIter {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|ch| if ch == ' ' { 0 } else { 1 })
                        .collect()
                })
                .collect(),
        }
    }
}
