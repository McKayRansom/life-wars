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
 * Fails:
 * - manually unroll neighbors double-loop
 *   - (about even perf)
 *
 * Wins:
 * - NEIGHBOR_OFFSETS array is slightly faster than loop
 *
 */
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct LifeBasic {
    size: (u16, u16),
    grid: Vec<Cell>,
}

impl LifeAlgo for LifeBasic {
    fn size(&self) -> (u16, u16) {
        self.size
    }

    fn get(&self, pos: (u16, u16)) -> Option<&Cell> {
        if pos.1 >= self.size.1 || pos.0 >= self.size.0 {
            None
        } else {
            self.grid.get((pos.1 * self.size.0 + pos.0) as usize)
        }
    }

    fn insert(&mut self, pos: (u16, u16), new_cell: Cell) -> Option<Cell> {
        let cell = self.grid.get_mut((pos.1 * self.size.0 + pos.0) as usize)?;
        // let row = self.grid.get_mut(pos.1 as usize)?;
        // let cell = row.get_mut(pos.0 as usize)?;
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
    pub fn new(dim: (u16, u16)) -> Self {
        Self {
            size: dim,
            grid: vec![Cell::new(0, 0); dim.0 as usize * dim.1 as usize],
        }
    }

    // This seemingly stupid iterator version is somehow faster?
    fn neighbors(&self, faction: u8, pos: (u16, u16)) -> (u8, u8) {
        const NEIGHBOR_OFFSETS: &[(i32, i32)] = &[
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ];

        let mut faction: u8 = faction;
        let mut sum: u8 = 0;
        for (dx, dy) in NEIGHBOR_OFFSETS {
            // for dy in -1..2 {
            // if let Some(row) = self.grid.get((pos.1 as i32 + dy) as usize) {
            // for dx in -1..2 {
            // if dx == 0 && dy == 0 {
            // continue;
            // }
            // if let Some(cell) = self.grid.get((pos.1 * self.size.1), (pos.0 as i32 + dx) as usize) {
            let x = pos.0 as i32 + dx;
            let y = pos.1 as i32 + dy;
            if x < 0 || y < 0 {
                continue;
            }
            if let Some(cell) = self.get((x as u16, y as u16)) {
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
            // }
            // }
        }
        (sum, faction)
    }

    fn update(&self, rule: &LifeRule, pops: &mut LifePops) -> Self {
        *pops = LifePops::new(); // clear 
        Self {
            size: self.size,
            grid: self
                .grid
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    // row.iter()
                    // .enumerate()
                    // .map(|(x, cell)| {
                    let new_cell = rule.update(
                        cell.get_state(),
                        self.neighbors(
                            cell.get_faction(),
                            (
                                (i as u16 % self.size.0) as u16,
                                (i as u16 / self.size.0) as u16,
                            ),
                        ),
                    );

                    if new_cell.is_alive() {
                        pops.add(new_cell.get_faction(), 1);
                    }
                    new_cell
                    // })
                    // .collect()
                })
                .collect(),
        }
    }
}

// impl From<&str> for LifeBasic {
//     fn from(value: &str) -> Self {
//         let grid: Vec<Cell>
//         Self {
//             size,
//             grid: value
//                 .split('\n')
//                 .map(|line| {
//                     line.chars()
//                         .map(|ch| Cell::new(if ch == ' ' { 0 } else { 1 }, 0))
//                         .collect()
//                 })
//                 .collect(),
//         }
//     }
// }

#[cfg(test)]
pub mod life_basic_test {

    use super::*;

    #[test]
    fn life_test_basic_new() {
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

    // #[test]
    // fn life_basic() {
    //     let mut life
    // }
}
