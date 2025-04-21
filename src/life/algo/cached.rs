use std::{hash::Hash, mem::swap};

use super::{Cell, LifeAlgo, LifePops, LifeRule, Pos};

/*
 * Simple optimizations from the Naive algorithm:
 * - Cache neighbor counts and update when cells change
 * - Track a list of recent updates and only look through this on updates
 * - Keep the old grid around instead of alloc/freeing every time (this is significant now that we do much less work on each update)
 *
 * This drastically reduces the time complexity from O(cells) to O(cells_changed)
 * at the cost of double the memory from O(cells) to O(2 * cells)
 * 
 * Possible improvements:
 * - instead of a list of cells changed, keep a (probably deduplicated) (HashSet) list of Cells AND neighbors that could change, then the loop is faster
 * - 1d Vec instead of 2d Vec
 * - Instead of having a 2nd copy of the array, store in a high-bit of the cell (or in the change list) a Next state and add changes to NextChanges. 
 *     Iterate through NextChanges and change the array in-place!
 * - instead of a double for-loop, build a static array of the 8 neighbors
 * 
 * Based partly on Abrash/Stafford algorithims from https://ericlippert.com/2020/06/29/life-part-19/
 * In those algorithms but not in mine:
 * - Store  3 (state + neighbors) in a u16. I don't want to do that because I have 4 states instead of 2
 * - Once you've done that, add a bunch of lookups for updating triplets instead of doing bit-math
 * - This saw a 10x speedup in 1990 and a 2x speedup in 2020 so I'm not sure it's worth it
 * - They also did the above improvement of storing and applying changes instead of a 2nd copy
 * 
 * WINS:
 * - Checking if new is already updated automatically de-dups the changes list
 * 
 * Fails:
 * - Faction checking affects performance about 20% and is still broken
 */
#[derive(PartialEq, Eq, Debug)]
pub struct LifeCached {
    grid: Vec<Vec<(Cell, i8)>>,
    old_grid: Vec<Vec<(Cell, i8)>>,
    recent_updates: Vec<Pos>,
    old_updates: Vec<Pos>,
}

impl LifeCached {
    pub fn new(dim: Pos) -> Self {
        Self {
            grid: vec![vec![(Cell::new(0, 0), 0); dim.x as usize]; dim.y as usize],
            recent_updates: Vec::new(),
            old_grid: vec![vec![(Cell::new(0, 0), 0); dim.x as usize]; dim.y as usize],
            old_updates: Vec::new(),
        }
    }

    fn update_neighbors(
        grid: &mut [Vec<(Cell, i8)>],
        faction: u8,
        amount: i8,
        pos: Pos,
    ) {
        for dy in -1..2 {
            for dx in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(row) = grid.get_mut((pos.y as i32 + dy) as usize) {
                    if let Some((cell, neigh)) = row.get_mut((pos.x as i32 + dx) as usize) {

                        // NOTE: This impacts performance ~20% :(
                        if cell.get_faction() != faction {
                            *neigh -= amount;
                        } else {
                            *neigh += amount;
                        }

                        if *neigh < 0 {
                            // TODO: Just recalc?
                            *neigh *= -1;
                            cell.set_faction(cell.get_faction() ^ 1);
                        }
                    }
                }
            }
        }
    }

    #[allow(unused)]
    fn neighbors_cached(&self, pos: (usize, usize)) -> (u8, u8) {
        let thing = &self.grid[pos.1][pos.0];
        (thing.1 as u8, thing.0.get_faction())
    }

    fn check_cell_and_neighbors(
        size: Pos,
        old_grid: &[Vec<(Cell, i8)>],
        new_grid: &mut [Vec<(Cell, i8)>],
        updates: &mut Vec<Pos>,
        pos: Pos,
        rule: &LifeRule,
        pops: &mut LifePops,
    ) {
        // let size = self.size();
        for dy in -1..2 {
            let py = pos.y as i32 + dy;
            if py < 0 || py as i16 >= size.y {
                continue;
            }
            for dx in -1..2 {
                let px = pos.x as i32 + dx;
                if px < 0 || px as i16 >= size.x {
                    continue;
                }
                let new_pos: Pos = (px as i16, py as i16).into();

                let old = old_grid[new_pos.y as usize][new_pos.x as usize];

                let new_cell = rule.update(old.0.get_state(), (old.1 as u8, old.0.get_faction()));

                let new = &mut new_grid[new_pos.y as usize][new_pos.x as usize];

                if new_cell != new.0 {
                    // println!("Cell at: {new_pos:?} was {:?} now {new_cell:?}", new);
                    updates.push(new_pos);
                    let alive_changed = new_cell.is_alive() != new.0.is_alive();
                    new.0 = new_cell;
                    if alive_changed {
                        let amount = if new_cell.is_alive() { 1 } else { -1 };
                        pops.add(new.0.get_faction(), amount as i16);
                        Self::update_neighbors(new_grid, new_cell.get_faction(), amount, new_pos);
                    }
                }
            }
        }
    }
}

impl LifeAlgo for LifeCached {
    fn size(&self) -> Pos {
        (self.grid[0].len() as i16, self.grid.len() as i16).into()
    }

    fn get(&self, pos: Pos) -> Option<&Cell> {
        self.grid
            .get(pos.y as usize)
            .map(|thing| thing.get(pos.x as usize).map(|(cell, _neigh)| cell))
            .unwrap_or(None)
    }

    fn insert(&mut self, pos: Pos, new_cell: Cell) -> Option<Cell> {
        let row = self.grid.get_mut(pos.y as usize)?;
        let cell = row.get_mut(pos.x as usize)?;

        if new_cell == cell.0 {
            return None;
        }

        self.recent_updates.push(pos);

        let alived_changed = new_cell.is_alive() != cell.0.is_alive();

        cell.0 = new_cell; // leave neighbor count alone!

        if alived_changed {
            Self::update_neighbors(
                &mut self.grid,
                new_cell.get_faction(),
                if new_cell.is_alive() { 1 } else { -1 },
                pos,
            );
        }

        None
    }

    fn update(&mut self, rule: &LifeRule, pops: &mut LifePops) {
        let this = &mut *self;

        for (dst, src) in this.old_grid.iter_mut().zip(this.grid.iter()) {
            dst.copy_from_slice(src);
        }

        swap(&mut this.recent_updates, &mut this.old_updates);

        this.recent_updates.clear();

        for pos in &this.old_updates {
            LifeCached::check_cell_and_neighbors(
                this.size(),
                &this.old_grid,
                &mut this.grid,
                &mut this.recent_updates,
                *pos,
                rule,
                pops,
            );
        }
    }

    fn hash(&self, state: &mut std::hash::DefaultHasher) {
        self.grid.hash(state);
    }
}

#[cfg(test)]
pub mod life_cached_test {
    use super::*;

    #[test]
    fn test_cached() {
        let mut life: LifeCached = LifeCached::new((3, 3).into());

        life.insert((1, 1).into(), 1.into());

        assert_eq!(life.get((0, 0).into()).unwrap().get_state(), 0);
        assert_eq!(life.get((1, 1).into()).unwrap().get_state(), 1);

        assert_eq!(life.neighbors_cached((0, 0)), (1, 0));
        assert_eq!(life.neighbors_cached((1, 0)), (1, 0));
        assert_eq!(life.neighbors_cached((0, 1)), (1, 0));
    }

    #[test]
    fn test_cached_faction() {
        let mut life: LifeCached = LifeCached::new((3, 3).into());
        life.insert((1, 0).into(), Cell::new(1, 1));
        life.insert((1, 1).into(), Cell::new(1, 1));
        life.insert((1, 2).into(), Cell::new(1, 1));

        assert_eq!(life.get((0, 0).into()).unwrap().get_state(), 0);
        assert_eq!(life.get((1, 0).into()).unwrap().get_state(), 1);
        assert_eq!(life.get((0, 1).into()).unwrap().get_state(), 0);

        let mut life_pops: LifePops = LifePops::new();
        life.update(&LifeRule::GOL, &mut life_pops);

        // assert_eq!(life.get((1, 1)).unwrap(), &Cell::new(1, 1));
        // assert_eq!(life.get((0, 1)).unwrap(), &Cell::new(1, 1));
        // assert_eq!(life.get((2, 1)).unwrap(), &Cell::new(1, 1));
    }
}
