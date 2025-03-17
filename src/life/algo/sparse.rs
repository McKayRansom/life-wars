use std::{collections::HashMap, hash::Hash};

use super::{Cell, LifeAlgo, LifePops, LifeRule};

/*
 * Sparse algorithm 
 * - Keep alive cells in hashmap
 * - only check thos
 *
 * Should be faster than Naiive because it's time is O(updated_cells) and it's space is O(cells)
 * performs truely terribly at the moment, need to figure out why
 */
#[derive(PartialEq, Eq, Debug)]
pub struct LifeSparse {
    size: (usize, usize),
    living: HashMap<(usize, usize), Cell>,
    recent_updates: Vec<(usize, usize)>,
}

const EMPTY_CELL: Cell = Cell::new(0, 0);

impl LifeSparse {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            size,
            living: HashMap::new(),
            recent_updates: Vec::new(),
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
                let old_cell = self.living.get(&new_pos).unwrap_or(&EMPTY_CELL);
                let new_cell = rule.update(
                    old_cell.get_state(),
                    self.neighbors(old_cell.get_faction(), new_pos),
                );

                if new_self.living.get(&new_pos).unwrap_or(&EMPTY_CELL) != &new_cell {
                    new_self.recent_updates.push(new_pos);

                    if !new_cell.is_dead() {
                        new_self.living.insert(new_pos, new_cell);
                    } else if !old_cell.is_dead() {
                        new_self.living.remove(&new_pos);
                    }
                }
            }
        }
    }
}

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
                self.recent_updates.push(pos);
                None
            }
        } else {
            if cell.is_alive() {
                // Birth
                self.recent_updates.push(pos);
                self.living.insert(pos, cell)
            } else {
                // Already Dead
                None
            }
        }
    }

    fn update(&mut self, rule: &LifeRule, _pops: &mut LifePops) {
        let mut new_self: Self = Self {
            living: self.living.clone(),
            recent_updates: Vec::new(),
            size: self.size,
        };

        for pos in &self.recent_updates {
            self.check_cell_and_neighbors(&mut new_self, *pos, rule);
        }

        *self = new_self
    }

    fn hash(&self, state: &mut std::hash::DefaultHasher) {
        // TODO: Is this deterministic??
        for (pos, cell) in self.living.iter() {
            pos.hash(state);
            cell.hash(state);
        }
    }
}

#[cfg(test)]
pub mod life_sprase_test {

    use super::*;

    #[test]
    fn life_test_basic() {
        let mut life: LifeSparse = LifeSparse::new((3, 3));

        life.insert((1, 0), Cell::new(1, 0));
        life.insert((1, 1), Cell::new(1, 0));
        life.insert((1, 2), Cell::new(1, 0));

        assert_eq!(life.get((0, 0)).unwrap(), &EMPTY_CELL);
        assert_eq!(life.get((1, 0)).unwrap().get_state(), 1);
        assert_eq!(life.get((0, 1)).unwrap(), &EMPTY_CELL);

        assert_eq!(life.neighbors(0, (0, 0)), (2, 0));
        assert_eq!(life.neighbors(0, (1, 0)), (1, 0));
        assert_eq!(life.neighbors(0, (0, 1)), (3, 0));

        let mut pops = LifePops::new();

        life.update(&LifeRule::GOL, &mut pops);
        // assert_eq!(
        //     update.living,
        //     <&str as Into<LifeSparse>>::into("   \n***\n   ").living
        // );
        assert_eq!(life.recent_updates, [(1, 0), (0, 1), (2, 1), (1, 2)]);

        life.update(&LifeRule::GOL, &mut pops);
        // assert_eq!(
        //     update.living,
        //     <&str as Into<LifeSparse>>::into(" * \n * \n * ").living
        // );
    }
}
