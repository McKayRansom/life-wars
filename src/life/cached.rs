use std::{fmt::Write, hash::Hash};

use super::{iter_life, state_update, Cell, Life};

#[derive(PartialEq, Eq, Debug)]
pub struct LifeCached {
    grid: Vec<Vec<(Cell, i8)>>,
    recent_births: Vec<(usize, usize)>,
    recent_deaths: Vec<(usize, usize)>,
}

impl Life for LifeCached {
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


        self.recent_births.push(pos);

        // let res = Some(replace(cell, (new_cell, 0)).0);
        cell.0 = new_cell; // leave neighbor count alone!

        Self::update_neighbors(&mut self.grid, 0, 1, pos);

        // println!("self: {self}");
        // res
        None
    }
}

impl LifeCached {
    pub fn new(dim: (usize, usize)) -> Self {
        Self {
            grid: vec![vec![(Cell::new(0, 0), 0); dim.0]; dim.1],
            recent_births: Vec::new(),
            recent_deaths: Vec::new(),
        }
    }

    // This seemingly stupid iterator version is somehow faster?
    pub fn update_neighbors(
        grid: &mut Vec<Vec<(Cell, i8)>>,
        _faction: u8,
        amount: i8,
        pos: (usize, usize),
    ) {
        for dy in -1..2 {
            if let Some(row) = grid.get_mut((pos.1 as i32 + dy) as usize) {
                for dx in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if let Some((_cell, neigh)) = row.get_mut((pos.0 as i32 + dx) as usize) {
                        // if cell.get_faction() == faction {
                        *neigh += amount;

                        assert!(*neigh >= 0, "Neighbor underflow at {pos:?} off: ({dx}, {dy})");
                        // } else {
                        // TODO: FACTIONS BROKEN??
                        // faction = cell.get_faction();
                        // If this is less than 0, we need to recalc?
                        // *neigh -= amount;
                        // }
                    }
                }
            }
        }
    }

    #[allow(unused)]
    fn neighbors(&self, pos: (usize, usize)) -> (u8, u8) {
        let thing = &self.grid[pos.1][pos.0];
        (thing.1 as u8, thing.0.get_faction())
    }

    fn check_cell_and_neighbors(&self, new_self: &mut Self, pos: (usize, usize)) {
        let size = self.size();
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

                let old = self.grid[new_pos.1][new_pos.0];

                let new_cell = state_update(old.0.get_state(), (old.1 as u8, old.0.get_faction()));

                let new = &mut new_self.grid[new_pos.1][new_pos.0];

                if new_cell != new.0 {
                    // if old.0 != new.0 {
                    // println!("Cell at: {new_pos:?} was {:?} now {new_cell:?}", new);
                    // println!("New birth!");
                    if new_cell.is_alive() {
                        new_self.recent_births.push(new_pos);
                    } else {
                        new_self.recent_deaths.push(new_pos);
                    }
                    // } else {
                    //     println!("Stale birth!");
                    // }
                    new.0 = new_cell;
                    Self::update_neighbors(
                        &mut new_self.grid,
                        new_cell.get_faction(),
                        if new_cell.is_alive() { 1 } else { -1 },
                        new_pos,
                    );

                    // println!("Self now: {new_self}");
                }
            }
        }
    }
    pub fn update(&self) -> Self {
        let mut new_self: Self = Self {
            grid: self.grid.clone(),
            recent_births: Vec::new(),
            recent_deaths: Vec::new(),
        };

        // println!("UUJ");

        for pos in &self.recent_births {
            self.check_cell_and_neighbors(&mut new_self, *pos);
        }

        for pos in &self.recent_deaths {
            self.check_cell_and_neighbors(&mut new_self, *pos);
        }

        new_self
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
            grid,
            recent_births,
            recent_deaths: Vec::new(),
        };

        for pos in &new_self.recent_births {
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

impl std::fmt::Display for LifeCached {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, y, _cell) in iter_life(self as &dyn Life) {
            if x == 0 {
                f.write_char('\n')?;
            }
            f.write_char(char::from_digit(self.grid[y][x].1 as u32, 10).unwrap())?;
            // if cell.is_alive() {
            //     f.write_char('*')?;
            // } else {
            //     f.write_char(' ')?;
            // }

        }
        Ok(())
    }
}

#[cfg(test)]
pub mod life_cached_test {

    use crate::life::basic::LifeBasic;

    use super::*;

    #[test]
    fn life_test_basic() {
        let life: LifeCached = " * \n * \n * ".into();

        assert_eq!(life.get((0, 0)).unwrap().get_state(), 0);
        assert_eq!(life.get((1, 0)).unwrap().get_state(), 1);
        assert_eq!(life.get((0, 1)).unwrap().get_state(), 0);

        assert_eq!(life.neighbors((0, 0)), (2, 0));
        assert_eq!(life.neighbors((1, 0)), (1, 0));
        assert_eq!(life.neighbors((0, 1)), (3, 0));

        assert_eq!(life.recent_births, [(1, 0), (1, 1), (1, 2)]);

        let update = life.update();

        assert_eq!(update.recent_births, [(0, 1), (2, 1)]);
        assert_eq!(update.recent_deaths, [(1, 0), (1, 2)]);
        assert_eq!(
            update.grid,
            <&str as Into<LifeCached>>::into("   \n***\n   ").grid
        );

        let update = update.update();
        assert_eq!(
            update.grid,
            <&str as Into<LifeCached>>::into(" * \n * \n * ").grid
        );

        assert_eq!(update.neighbors((0, 0)), (2, 0));
        assert_eq!(update.neighbors((1, 0)), (1, 0));
        assert_eq!(update.neighbors((0, 1)), (3, 0));

        let update = update.update();
        assert_eq!(
            update.grid,
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
            for (x, y, basic_cell) in iter_life(&life_basic) {

                let cached_cell = life_cached.get((x, y)).unwrap();
                assert_eq!(basic_cell, cached_cell);
            }

            life_basic = life_basic.update();
            life_cached = life_cached.update();
        }
    }
}
