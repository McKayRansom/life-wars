use std::{hash::Hash, mem::swap};

use super::{Cell, LifeAlgo, state_update};

#[derive(PartialEq, Eq, Debug)]
pub struct LifeCached {
    grid: Vec<Vec<(Cell, i8)>>,
    old_grid: Vec<Vec<(Cell, i8)>>,
    recent_updates: Vec<(usize, usize)>,
    old_updates: Vec<(usize, usize)>,
    // recent_deaths: Vec<(usize, usize)>,
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

        // assert!(new_cell.is_alive(), "Adding new cell: {new_cell:?} at pos: {pos:?}");
        // println!("Adding new cell: {new_cell:?} at pos: {pos:?}");

        self.recent_updates.push(pos);

        // let res = Some(replace(cell, (new_cell, 0)).0);
        cell.0 = new_cell; // leave neighbor count alone!

        Self::update_neighbors(&mut self.grid, new_cell.get_faction(), 1, pos);

        // println!("self: {self}");
        // res
        None
    }
    
    fn update(&mut self) {
        self.update();
    }
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
                            // cell.set_faction(faction);
                            // amount *= -1;
                        }

                        *neigh += amount;
                        // if *neigh < 0 {
                        // we're hosed, recalc is the only option
                        // cell.set_faction(faction);
                        // *neigh *= -1;
                        // let (neigh, faction) = Self::neighbors(grid, faction, pos);

                        // let (dst_cell, dst_neigh) = grid
                        //     .get_mut((pos.1 as i32 + dy) as usize)
                        //     .unwrap()
                        //     .get_mut((pos.0 as i32 + dx) as usize)
                        //     .unwrap();

                        // dst_cell.set_faction(faction);
                        // *dst_neigh = neigh as i8;
                        // }
                        // } else if *neigh > 0 {
                        // *neigh -= amount;
                        // } else {
                        // *neigh
                        // }

                        // assert!(
                        //     *neigh >= 0,
                        //     "Neighbor underflow at {pos:?} off: ({dx}, {dy})"
                        // );
                    }
                }
            }
        }
    }

    pub fn neighbors(
        grid: &mut Vec<Vec<(Cell, i8)>>,
        faction: u8,
        pos: (usize, usize),
    ) -> (u8, u8) {
        let mut faction: u8 = faction;
        let mut sum: u8 = 0;
        for dy in -1..2 {
            if let Some(row) = grid.get((pos.1 as i32 + dy) as usize) {
                for dx in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if let Some((cell, _neigh)) = row.get((pos.0 as i32 + dx) as usize) {
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

                let new_cell = state_update(old.0.get_state(), (old.1 as u8, old.0.get_faction()));

                let new = &mut new_grid[new_pos.1][new_pos.0];

                if new_cell != new.0 {
                    // println!("Cell at: {new_pos:?} was {:?} now {new_cell:?}", new);
                    updates.push(new_pos);
                    let alive_changed = new_cell.is_alive() != new.0.is_alive();
                    new.0 = new_cell;
                    if alive_changed {
                        Self::update_neighbors(
                            new_grid,
                            new_cell.get_faction(),
                            if new_cell.is_alive() { 1 } else { -1 },
                            new_pos,
                        );
                    }
                }
            }
        }
    }
    pub fn update(&mut self) {
        // TODO: Change to flat vector?
        for (dst, src) in self.old_grid.iter_mut().zip(self.grid.iter()) {
            dst.copy_from_slice(src);
        }

        swap(&mut self.recent_updates, &mut self.old_updates);

        self.recent_updates.clear();

        for pos in &self.old_updates {
            Self::check_cell_and_neighbors(
                self.size(),
                &mut self.old_grid,
                &mut self.grid,
                &mut self.recent_updates,
                *pos,
            );
        }
    }
}

impl From<&str> for LifeCached {
    fn from(value: &str) -> Self {
        let mut recent_births = Vec::new();
        let mut grid = Vec::new();

        let mut pos: (usize, usize) = (0, 0);
        for line in value.split('\n') {
            let mut cell_line: Vec<(Cell, i8)> = Vec::new();
            for chr in line.chars() {
                if chr != ' ' {
                    cell_line.push((Cell::new(1, 0), 0));
                    recent_births.push(pos);
                } else {
                    cell_line.push((Cell::new(0, 0), 0));
                }
                pos.0 += 1;
            }
            grid.push(cell_line);
            pos.1 += 1;
            pos.0 = 0;
        }

        let mut new_self = Self {
            old_grid: grid.clone(),
            grid,
            old_updates: Vec::new(),
            recent_updates: recent_births,
        };

        for pos in &new_self.recent_updates {
            Self::update_neighbors(&mut new_self.grid, 0, 1, *pos);
        }

        new_self
    }
}

impl Hash for LifeCached {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
        // Do not hash these as they don't matter
        // self.recent_births.hash(state);
        // self.recent_deaths.hash(state);
    }
}

// impl std::fmt::Display for LifeCached {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for (x, y, _cell) in iter_life(&dyn Life) {
//             if x == 0 {
//                 f.write_char('\n')?;
//             }
//             f.write_char(char::from_digit(self.grid[y][x].1 as u32, 10).unwrap())?;
//         }
//         Ok(())
//     }
// }

#[cfg(test)]
pub mod life_cached_test {

    use crate::life::basic::LifeBasic;

    use super::*;

    #[test]
    fn test_cached() {
        let mut life: LifeCached = " * \n * \n * ".into();

        assert_eq!(life.get((0, 0)).unwrap().get_state(), 0);
        assert_eq!(life.get((1, 0)).unwrap().get_state(), 1);
        assert_eq!(life.get((0, 1)).unwrap().get_state(), 0);

        assert_eq!(life.neighbors_cached((0, 0)), (2, 0));
        assert_eq!(life.neighbors_cached((1, 0)), (1, 0));
        assert_eq!(life.neighbors_cached((0, 1)), (3, 0));

        assert_eq!(LifeCached::neighbors(&mut life.grid, 0, (0, 0)), (2, 0));
        assert_eq!(LifeCached::neighbors(&mut life.grid, 0, (1, 0)), (1, 0));
        assert_eq!(LifeCached::neighbors(&mut life.grid, 0, (0, 1)), (3, 0));

        assert_eq!(life.recent_updates, [(1, 0), (1, 1), (1, 2)]);

        life.update();

        // assert_eq!(life.recent_updates, [(0, 1), (2, 1), (1, 0), (1, 2)]);
        assert_eq!(
            life.grid,
            <&str as Into<LifeCached>>::into("   \n***\n   ").grid
        );

        life.update();
        assert_eq!(
            life.grid,
            <&str as Into<LifeCached>>::into(" * \n * \n * ").grid
        );

        assert_eq!(life.neighbors_cached((0, 0)), (2, 0));
        assert_eq!(life.neighbors_cached((1, 0)), (1, 0));
        assert_eq!(life.neighbors_cached((0, 1)), (3, 0));

        life.update();
        assert_eq!(
            life.grid,
            <&str as Into<LifeCached>>::into("   \n***\n   ").grid
        );
    }

    #[test]
    fn test_basic_comparee() {
        let mut life_basic = LifeBasic::new((8, 8));
        let mut life_cached = LifeCached::new((8, 8));

        life_basic.randomize(1234, false);
        life_cached.randomize(1234, false);

        for _ in 0..100 {
            // for (x, y, basic_cell) in iter_life(&life_basic) {
            //     let cached_cell = life_cached.get((x, y)).unwrap();
            //     assert_eq!(basic_cell, cached_cell);
            // }

            life_basic = life_basic.update();
            life_cached.update();
        }
    }
}
