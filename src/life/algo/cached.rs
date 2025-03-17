use std::{hash::Hash, mem::swap};

use super::{Cell, LifeAlgo, LifePops, LifeRule};

/*
 * Simple optimizations from the Naive algorithm:
 * - Cache neighbor counts and update when cells change
 * - Track a list of recent updates and only look through this on updates
 * - Keep the old grid around instead of alloc/freeing every time (this is significant now that we do much less work on each update)
 *
 * This drastically reduces the time complexity from O(cells) to O(cells_changed)
 * at the cost of double the memory plus some
 */
#[derive(PartialEq, Eq, Debug)]
pub struct LifeCached {
    grid: Vec<Vec<(Cell, i8)>>,
    old_grid: Vec<Vec<(Cell, i8)>>,
    recent_updates: Vec<(usize, usize)>,
    old_updates: Vec<(usize, usize)>,
    // recent_deaths: Vec<(usize, usize)>,
}

impl LifeCached {
    pub fn new(dim: (usize, usize)) -> Self {
        Self {
            grid: vec![vec![(Cell::new(0, 0), 0); dim.0]; dim.1],
            recent_updates: Vec::new(),
            old_grid: vec![vec![(Cell::new(0, 0), 0); dim.0]; dim.1],
            old_updates: Vec::new(),
        }
    }

    fn update_neighbors(
        grid: &mut Vec<Vec<(Cell, i8)>>,
        faction: u8,
        amount: i8,
        pos: (usize, usize),
    ) {
        for dy in -1..2 {
            for dx in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(row) = grid.get_mut((pos.1 as i32 + dy) as usize) {
                    if let Some((cell, neigh)) = row.get_mut((pos.0 as i32 + dx) as usize) {
                        if cell.get_faction() != faction {
                            *neigh -= amount;
                        } else {
                            *neigh += amount;
                        }

                        if *neigh < 0 {
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
        size: (usize, usize),
        old_grid: &Vec<Vec<(Cell, i8)>>,
        new_grid: &mut Vec<Vec<(Cell, i8)>>,
        updates: &mut Vec<(usize, usize)>,
        pos: (usize, usize),
        rule: &LifeRule,
        pops: &mut LifePops,
    ) {
        // let size = self.size();
        for dy in -1..2 {
            let py = pos.1 as i32 + dy;
            if py < 0 || py as usize >= size.1 {
                continue;
            }
            for dx in -1..2 {
                let px = pos.0 as i32 + dx;
                if px < 0 || px as usize >= size.0 {
                    continue;
                }
                let new_pos: (usize, usize) = (px as usize, py as usize);

                let old = old_grid[new_pos.1][new_pos.0];

                let new_cell = rule.update(old.0.get_state(), (old.1 as u8, old.0.get_faction()));

                let new = &mut new_grid[new_pos.1][new_pos.0];

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
    fn size(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn get(&self, pos: (usize, usize)) -> Option<&Cell> {
        self.grid
            .get(pos.1)
            .map(|thing| thing.get(pos.0).map(|(cell, _neigh)| cell))
            .unwrap_or(None)
    }

    fn insert(&mut self, pos: (usize, usize), new_cell: Cell) -> Option<Cell> {
        let row = self.grid.get_mut(pos.1)?;
        let cell = row.get_mut(pos.0)?;

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
        // TODO: Change to flat vector?
        for (dst, src) in this.old_grid.iter_mut().zip(this.grid.iter()) {
            dst.copy_from_slice(src);
        }

        swap(&mut this.recent_updates, &mut this.old_updates);

        this.recent_updates.clear();

        for pos in &this.old_updates {
            LifeCached::check_cell_and_neighbors(
                this.size(),
                &mut this.old_grid,
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
        let mut life: LifeCached = LifeCached::new((3, 3));

        life.insert((1, 1), 1.into());

        assert_eq!(life.get((0, 0)).unwrap().get_state(), 0);
        assert_eq!(life.get((1, 1)).unwrap().get_state(), 1);

        assert_eq!(life.neighbors_cached((0, 0)), (1, 0));
        assert_eq!(life.neighbors_cached((1, 0)), (1, 0));
        assert_eq!(life.neighbors_cached((0, 1)), (1, 0));
    }

    #[test]
    fn test_cached_faction() {
        let mut life: LifeCached = LifeCached::new((3, 3));
        life.insert((1, 0), Cell::new(1, 1));
        life.insert((1, 1), Cell::new(1, 1));
        life.insert((1, 2), Cell::new(1, 1));

        assert_eq!(life.get((0, 0)).unwrap().get_state(), 0);
        assert_eq!(life.get((1, 0)).unwrap().get_state(), 1);
        assert_eq!(life.get((0, 1)).unwrap().get_state(), 0);

        let mut life_pops: LifePops = LifePops::new();
        life.update(&LifeRule::GOL, &mut life_pops);

        // assert_eq!(life.get((1, 1)).unwrap(), &Cell::new(1, 1));
        // assert_eq!(life.get((0, 1)).unwrap(), &Cell::new(1, 1));
        // assert_eq!(life.get((2, 1)).unwrap(), &Cell::new(1, 1));
    }
}
