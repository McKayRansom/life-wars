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
 * - get_unchecked is WAAAAY slower?? 
 *
 * Wins:
 * - NEIGHBOR_OFFSETS array is slightly faster than loop
 * - Make grid size + 2 in all dirs to avoid bounds checking (1800 ms -> 1110 ms acorn-time)
 * - 1d vec instead of 2d vec (??? was this faster?)
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
            self.grid
                .get(((pos.1 + 1) * (self.size.0 + 2) + (pos.0 + 1)) as usize)
        }
    }

    fn insert(&mut self, pos: (u16, u16), new_cell: Cell) -> Option<Cell> {
        let cell = self
            .grid
            .get_mut(((pos.1 + 1) * (self.size.0 + 2) + (pos.0 + 1)) as usize)?;
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
            grid: vec![Cell::new(0, 0); (dim.0 + 2) as usize * (dim.1 + 2) as usize],
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

            let x = pos.0 as i32 + dx;
            let y = pos.1 as i32 + dy;

            let cell =
                self.grid[(y as usize) * (self.size.0 as usize + 2) + (x as usize)];

            if cell.is_alive() {
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
        (sum, faction)
    }

    fn update(&self, rule: &LifeRule, pops: &mut LifePops) -> Self {
        *pops = LifePops::new(); // clear 
        let mut new_self = Self {
            size: self.size,
            grid: self.grid.clone(),
        };

        for y in 1..self.size.1 + 1 {
            for x in 1..self.size.0 + 1 {
                let pos = (x, y);
                let cell = self.grid
                    [((pos.1) as usize * (self.size.0 as usize + 2) + (pos.0 as usize)) as usize]; 
                let new_cell =
                    rule.update(cell.get_state(), self.neighbors(cell.get_faction(), pos));

                if new_cell.is_alive() {
                    pops.add(new_cell.get_faction(), 1);
                }
                new_self.grid
                    [(pos.1 as usize * (self.size.0 as usize + 2) + (pos.0 as usize)) as usize] =
                    new_cell;
            }
        }
        new_self
    }
}

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

        // these are virtual x/y not real x/y
        assert_eq!(life.neighbors(0, (1, 1)), (2, 0));
        assert_eq!(life.neighbors(0, (2, 1)), (1, 0));
        assert_eq!(life.neighbors(0, (1, 2)), (3, 0));
    }
}
