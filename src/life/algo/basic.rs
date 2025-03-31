use std::{hash::Hash, mem::replace};

use super::{Cell, LifeAlgo, LifePops, LifeRule, Pos};

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
    size: Pos,
    grid: Vec<Cell>,
}

impl LifeBasic {
    pub fn new(dim: Pos) -> Self {
        Self {
            size: dim,
            grid: vec![Cell::new(0, 0); (dim.x + 2) as usize * (dim.y + 2) as usize],
        }
    }

    fn neighbors(&self, faction: u8, pos: Pos) -> (u8, u8) {
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
            let x = pos.x as i32 + dx;
            let y = pos.y as i32 + dy;

            let cell = self.grid[(y as usize) * (self.size.x as usize + 2) + (x as usize)];

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
}

impl LifeAlgo for LifeBasic {
    fn size(&self) -> Pos {
        self.size
    }

    fn get(&self, pos: Pos) -> Option<&Cell> {
        if pos.y >= self.size.y || pos.x >= self.size.x {
            None
        } else {
            self.grid
                .get((pos.y as usize + 1) * (self.size.x as usize + 2) + (pos.x as usize + 1))
        }
    }

    fn insert(&mut self, pos: Pos, new_cell: Cell) -> Option<Cell> {
        let cell = self
            .grid
            .get_mut((pos.y as usize + 1) * (self.size.x as usize + 2) + (pos.x as usize + 1))?;
        Some(replace(cell, new_cell))
    }

    fn update(&mut self, rule: &LifeRule, pops: &mut LifePops) {
        *pops = LifePops::new(); // clear 
        let mut new_self = Self {
            size: self.size,
            grid: self.grid.clone(), // This clone doesn't even show up on the flamegraph (just 1 alocation probably)
        };

        for y in 1..self.size.y + 1 {
            for x in 1..self.size.x + 1 {
                let pos: Pos = (x, y).into();
                let cell = self.grid[pos.y as usize * (self.size.x as usize + 2) + pos.x as usize];
                let new_cell =
                    rule.update(cell.get_state(), self.neighbors(cell.get_faction(), pos));

                // Note inlining this check to amount is somehow slightly slower?
                if new_cell.is_alive() {
                    pops.add(new_cell.get_faction(), 1);
                }
                new_self.grid[pos.y as usize * (self.size.x as usize + 2) + pos.x as usize] =
                    new_cell;
            }
        }
        *self = new_self;
    }

    fn hash(&self, state: &mut std::hash::DefaultHasher) {
        self.grid.hash(state);
    }
}

#[cfg(test)]
pub mod life_basic_test {

    use super::*;

    #[test]
    fn life_test_basic_new() {
        let mut life = LifeBasic::new((3, 3).into());

        life.insert((1, 0).into(), 1.into());
        life.insert((1, 1).into(), 1.into());
        life.insert((1, 2).into(), 1.into());

        assert_eq!(life.get((0, 0).into()).unwrap().get_state(), 0);
        assert_eq!(life.get((1, 0).into()).unwrap().get_state(), 1);
        assert_eq!(life.get((0, 1).into()).unwrap().get_state(), 0);

        // these are virtual x/y not real x/y
        assert_eq!(life.neighbors(0, (1, 1).into()), (2, 0));
        assert_eq!(life.neighbors(0, (2, 1).into()), (1, 0));
        assert_eq!(life.neighbors(0, (1, 2).into()), (3, 0));
    }
}
