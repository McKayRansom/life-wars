use std::collections::HashMap;

use super::{Cell, LifeAlgo, LifePops, LifeRule};

#[derive(PartialEq, Eq, Debug)]
pub struct LifeSparse {
    size: (usize, usize),
    living: HashMap<(usize, usize), Cell>,
    recent_births: Vec<(usize, usize)>,
    recent_deaths: Vec<(usize, usize)>,
}

const EMPTY_CELL: Cell = Cell::new(0, 0);

impl LifeAlgo for LifeSparse {
    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn get(&self, pos: (usize, usize)) -> Option<&Cell> {
        self.living.get(&pos).or(Some(&EMPTY_CELL))
    }

    fn insert(&mut self, pos: (usize, usize), cell: Cell) -> Option<Cell> {
        if self.living.contains_key(&pos) {
            if cell.is_alive() {
                // Already alive
                // self.living.insert(pos, cell)
                None
            } else {
                // Kill
                self.living.remove(&pos);
                self.recent_deaths.push(pos);
                None
            }
        } else {
            if cell.is_alive() {
                // Birth
                self.recent_births.push(pos);
                self.living.insert(pos, cell)
            } else {
                // Already Dead
                None
            }
        }
   }
   
    fn update(&mut self, _rule: &LifeRule, _pops: &mut LifePops) {
        todo!()
    }
    
    fn hash(&self, _state: &mut std::hash::DefaultHasher) {
        // self.living.hash();
    }
}

impl LifeSparse {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            size,
            living: HashMap::new(),
            recent_births: Vec::new(),
            recent_deaths: Vec::new(),
            // grid: HashMap::with_capacity(size.0/4)
        }
    }

    pub fn neighbors(&self, faction: u8, pos: (usize, usize)) -> (u8, u8) {
        let mut faction: u8 = faction;
        let mut sum: u8 = 0;
        for dy in -1..2 {
            // TODO: Try doing calc here...
            for dx in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(cell) = self
                    .living
                    .get(&((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize))
                {
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
        (sum, faction)
    }

    fn check_cell_and_neighbors(&self, new_self: &mut Self, pos: (usize, usize), rule: &LifeRule) {
        for dy in -1..2 {
            let py = pos.1 as i32 + dy;
            if py < 0 || py as usize >= self.size.1 {
                continue;
            }
            for dx in -1..2 {
                let px = pos.0 as i32 + dx;
                if px < 0 || px as usize >= self.size.0 {
                    continue;
                }
                let new_pos: (usize, usize) = (px as usize, py as usize);
                // if new_pos == pos {
                let was_alive = self.living.contains_key(&new_pos);
                let cell = rule.update(if was_alive { 1 } else { 0 }, self.neighbors(0, new_pos));

                if was_alive {
                    if cell.is_alive() {
                        // still alive
                    } else {
                        // just died
                        if new_self.living.remove(&new_pos).is_some() {
                            new_self.recent_deaths.push(new_pos);
                        }
                        // println!("Death: {new_pos:?}");
                    }
                } else {
                    if cell.is_alive() {
                        // just born
                        if new_self.living.insert(new_pos, cell).is_none() {
                            new_self.recent_births.push(new_pos);
                        }
                        // println!("Birth: {new_pos:?}");
                    } else {
                        // still dead
                    }
                }
            }
        }
    }

    pub fn update(&self, rule: &LifeRule) -> Self {
        let mut new_self: Self = Self {
            living: self.living.clone(),
            recent_births: Vec::new(),
            recent_deaths: Vec::new(),
            size: self.size,
        };

        for pos in &self.recent_births {
            self.check_cell_and_neighbors(&mut new_self, *pos, rule);
        }

        for pos in &self.recent_deaths {
            self.check_cell_and_neighbors(&mut new_self, *pos, rule);
        }

        new_self
    }
}

impl From<&str> for LifeSparse {
    fn from(value: &str) -> Self {
        let mut size: (usize, usize) = (0, 0);
        let mut living: HashMap<(usize, usize), Cell> = HashMap::new();
        let mut recent_births: Vec<(usize, usize)> = Vec::new();
        let mut pos: (usize, usize) = (0, 0);
        for line in value.split('\n') {
            for chr in line.chars() {
                if chr != ' ' {
                    living.insert(pos, Cell::new(1, 0));
                    recent_births.push(pos);
                }
                pos.0 += 1;
                if pos > size {
                    size = pos;
                }
            }
            pos.1 += 1;
            pos.0 = 0;
        }

        size.1 += 1;

        Self {
            size,
            living,
            recent_births,
            recent_deaths: Vec::new(),
        }
    }
}

#[cfg(test)]
pub mod life_sprase_test {

    use super::*;

    #[test]
    fn life_test_basic() {
        let life: LifeSparse = " * 
 * 
 * "
        .into();

        assert_eq!(life.get((0, 0)).unwrap(), &EMPTY_CELL);
        assert_eq!(life.get((1, 0)).unwrap().get_state(), 1);
        assert_eq!(life.get((0, 1)).unwrap(), &EMPTY_CELL);

        assert_eq!(life.neighbors(0, (0, 0)), (2, 0));
        assert_eq!(life.neighbors(0, (1, 0)), (1, 0));
        assert_eq!(life.neighbors(0, (0, 1)), (3, 0));

        let update = life.update(&LifeRule::GOL);
        assert_eq!(
            update.living,
            <&str as Into<LifeSparse>>::into("   \n***\n   ").living
        );
        assert_eq!(update.recent_births, [(0, 1), (2, 1)]);
        assert_eq!(update.recent_deaths, [(1, 0), (1, 2)]);

        let update = update.update(&LifeRule::GOL);
        assert_eq!(
            update.living,
            <&str as Into<LifeSparse>>::into(" * \n * \n * ").living
        );
    }
}
