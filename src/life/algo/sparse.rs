use std::{collections::HashMap, hash::{BuildHasherDefault, Hash}};

use fxhash::FxHasher;

use super::{Cell, LifeAlgo, LifePops, LifeRule};

/*
 * Sparse algorithm 
 * - Keep alive cells in hashmap
 * - only check thos
 *
 * Should be faster than Naiive because it's time is O(updated_cells) and it's space is O(cells)
 * performs truely terribly at the moment, need to figure out why
 * 
 * Possible improvements:
 * - All of cached can be done again (neighbor counts, change list)
 * 
 * WINS:
 * - use u16 instead of usize for coords (which would by definition consume more memory than a 64-bit machine can address)
 *   - 8000 -> 2500 us/iter
 * - use FxHasher instead of default Sip hasher
 *   - 2500 -> 930 us/iter
 * 
 * Fail:
 * - Use NoHash, was really slow... collisions??
 * 
 * Based on https://ericlippert.com/2020/07/09/life-part-22/
 */
#[derive(PartialEq, Eq, Debug)]
pub struct LifeSparse {
    size: (u16, u16),
    living: HashMap<(u16, u16), Cell, BuildHasherDefault<FxHasher>>,
    recent_updates: Vec<(u16, u16)>,
}

const EMPTY_CELL: Cell = Cell::new(0, 0);

impl LifeSparse {
    pub fn new(size: (u16, u16)) -> Self {
        Self {
            size,
            living: HashMap::with_capacity_and_hasher(64, BuildHasherDefault::default()),
            recent_updates: Vec::new(),
            // grid: HashMap::with_capacity(size.0/4)
        }
    }

    pub fn neighbors(&self, faction: u8, pos: (u16, u16)) -> (u8, u8) {
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
                    .get(&((pos.0 as i32 + dx) as u16, (pos.1 as i32 + dy) as u16))
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

    fn check_cell_and_neighbors(&self, new_self: &mut Self, pos: (u16, u16), rule: &LifeRule) {
        for dy in -1..2 {
            let py = pos.1 as i32 + dy;
            if py < 0 || py as u16 >= self.size.1 {
                continue;
            }
            for dx in -1..2 {
                let px = pos.0 as i32 + dx;
                if px < 0 || px as u16 >= self.size.0 {
                    continue;
                }
                let new_pos: (u16, u16) = (px as u16, py as u16);
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
    fn size(&self) -> (u16, u16) {
        self.size
    }

    fn get(&self, pos: (u16, u16)) -> Option<&Cell> {
        self.living.get(&pos).or(Some(&EMPTY_CELL))
    }

    fn insert(&mut self, pos: (u16, u16), cell: Cell) -> Option<Cell> {
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
