use std::{hash::Hash, mem::replace};

use super::{Cell, LifeAlgo, LifePops, LifeRule};

/*
 * Naive algorithm:
 * - Check every cell every tick
 * - calculate neighbors
 * - apply rule
 * 
 * Always works but is O(cells)
 * 
 */
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct LifeBasic {
    grid: Vec<Vec<Cell>>,
}

impl LifeAlgo for LifeBasic {
    fn size(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn get(&self, pos: (usize, usize)) -> Option<&Cell> {
        self.grid
            .get(pos.1)
            .map(|thing| thing.get(pos.0))
            .unwrap_or(None)
    }

    fn insert(&mut self, pos: (usize, usize), new_cell: Cell) -> Option<Cell> {
        let row = self.grid.get_mut(pos.1)?;
        let cell = row.get_mut(pos.0)?;
        Some(replace(cell, new_cell))
    }

    fn update(&mut self, rule: &LifeRule, pops: &mut LifePops) {
        *self = Self::update(self, rule, pops);
    }

    fn hash(&self, state: &mut std::hash::DefaultHasher) {
        self.grid.hash(state);
    }
}

impl LifeBasic {
    pub fn new(dim: (usize, usize)) -> Self {
        Self {
            grid: vec![vec![Cell::new(0, 0); dim.0]; dim.1],
        }
    }

    // This seemingly stupid iterator version is somehow faster?
    fn neighbors(&self, faction: u8, pos: (usize, usize)) -> (u8, u8) {
        let mut faction: u8 = faction;
        let mut sum: u8 = 0;
        for dy in -1..2 {
            if let Some(row) = self.grid.get((pos.1 as i32 + dy) as usize) {
                for dx in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if let Some(cell) = row.get((pos.0 as i32 + dx) as usize) {
                        if cell.is_alive() {
                            // sum += 1;
                            if cell.get_faction() == faction {
                                sum += 1;
                            } else if sum > 0 {
                                sum -= 1;
                            } else {
                                faction = cell.get_faction();
                                sum += 1;
                            }
                        }
                    }
                }
            }
        }
        (sum, faction)
    }

    fn update(&self, rule: &LifeRule, pops: &mut LifePops) -> Self {
        *pops = LifePops::new(); // clear 
        Self {
            grid: self
                .grid
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, cell)| {
                            let new_cell = rule.update(
                                cell.get_state(),
                                self.neighbors(cell.get_faction(), (x, y)),
                            );

                            if new_cell.is_alive() {
                                pops.add(new_cell.get_faction(), 1);
                            }
                            new_cell
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl From<&str> for LifeBasic {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|ch| Cell::new(if ch == ' ' { 0 } else { 1 }, 0))
                        .collect()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
pub mod life_basic_test {

    use super::*;

    #[test]
    fn life_test_basic() {
        let mut life = LifeBasic::new((3, 3));

        life.insert((1, 0), 1.into());
        life.insert((1, 1), 1.into());
        life.insert((1, 2), 1.into());

        assert_eq!(life.get((0, 0)).unwrap().get_state(), 0);
        assert_eq!(life.get((1, 0)).unwrap().get_state(), 1);
        assert_eq!(life.get((0, 1)).unwrap().get_state(), 0);

        assert_eq!(life.neighbors(0, (0, 0)), (2, 0));
        assert_eq!(life.neighbors(0, (1, 0)), (1, 0));
        assert_eq!(life.neighbors(0, (0, 1)), (3, 0));

        // assert_eq!(life.update(&LifeRule::GOL, &mut life_pops), "   \n***\n   ".into());
        // assert_eq!(life_pops.get(0), 3);
    }
}
