use std::{fmt::Write, hash::DefaultHasher};

use basic::LifeBasic;
use cached::LifeCached;

pub mod basic;
pub mod cached;
pub mod sparse;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Cell {
    value: u8,
}

impl Cell {
    const STATE_MASK: u8 = 0xF;
    const FACTION_MASK: u8 = 0xF0;

    pub const fn new(state: u8, faction: u8) -> Self {
        Self {
            value: state | (faction << 4),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.value & Self::STATE_MASK == 0x1
    }

    pub fn get_state(&self) -> u8 {
        self.value & Self::STATE_MASK
    }

    pub fn get_faction(&self) -> u8 {
        (self.value & Self::FACTION_MASK) >> 4
    }

    pub fn set_state(&mut self, state: u8) {
        self.value = (self.value & Self::FACTION_MASK) | state;
    }

    pub fn set_faction(&mut self, state: u8) {
        self.value = (self.value & Self::STATE_MASK) | (state << 4);
    }
}

pub trait LifeAlgo {
    fn size(&self) -> (usize, usize);
    fn get(&self, pos: (usize, usize)) -> Option<&Cell>;
    fn insert(&mut self, pos: (usize, usize), cell: Cell) -> Option<Cell>;
    fn update(&mut self, rule: &LifeRule);
    fn hash(&self, state: &mut DefaultHasher);
}

pub struct LifeRule {
    birth: u16,
    survive: u16,
}

impl LifeRule {
    pub const GOL: Self = Self::new(0b1000, 0b1100);

    pub const fn new(birth: u16, survive: u16) -> Self {
        Self { birth, survive }
    }

    pub fn update(&self, state: u8, (neighbors, faction): (u8, u8)) -> Cell {
        Cell::new(Self::state_update_f(self, state, neighbors), faction)
    }

    // fn new_life_from_string(str: &str) -> Box<dyn Life> {

    // }

    // GOL B3/S23
    // const BIRTH_RULE: [u8; 9] = [0, 0, 0, 1, 0, 0, 0, 0, 0];
    // const SURVIVE_RULE: [u8; 9] = [0, 0, 1, 1, 0, 0, 0, 0, 0];
    // const STATE_RULE: [[u8; 9]; 2] = [BIRTH_RULE, SURVIVE_RULE];

    pub fn state_update_f(&self, state: u8, neighbors: u8) -> u8 {
        // SWR B2/S345/4
        // if state == 0 {
        //     if neighbors == 2 { 1 } else { 0 }
        // } else if state == 1 {
        //     if neighbors >= 3 && neighbors <= 5 {
        //         1
        //     } else {
        //         2
        //     }
        // } else if state == 3 {
        //     0
        // } else {
        //     state + 1
        // }
        // GOL B3/S23
        if state > 0 {
            ((self.survive & 1 << neighbors) >> neighbors) as u8
        } else {
            ((self.birth & 1 << neighbors) >> neighbors) as u8
        }
    }
}

pub struct Life {
    algo: Box<dyn LifeAlgo>,
    rule: LifeRule,
}

pub enum LifeAlgoSelect {
    Basic,
    Cached,
}

impl Life {
    pub fn new(algo: LifeAlgoSelect, size: (usize, usize)) -> Self {
        Self {
            algo: match algo {
                LifeAlgoSelect::Basic => Box::new(LifeBasic::new(size)),
                LifeAlgoSelect::Cached => Box::new(LifeCached::new(size)),
            },
            rule: LifeRule::GOL, // rule: LifeRule::new(0, 0)
        }
    }

    // fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut u8)>;
    pub fn randomize(&mut self, seed: u64, use_factions: bool) {
        macroquad::rand::srand(seed);

        let size = self.size();
        for x in 0..size.0 {
            for y in 0..size.1 {
                self.insert(
                    (x, y),
                    Cell::new(
                        if macroquad::rand::rand() < u32::MAX / 5 {
                            1
                        } else {
                            0
                        },
                        if use_factions && y < size.1 / 2 { 1 } else { 0 },
                    ),
                );
            }
        }
    }

    /*
     *        let rle_glider = "
     *        #C This is a glider.
     *        x = 3, y = 3
     *        bo$2bo$3o!";
     */
    pub fn new_life_from_rle(rle: &str) -> Self {
        let mut life: Option<Self> = None;
        let mut pos: (usize, usize) = (0, 0);
        for line in rle.split('\n') {
            if life.is_none() {
                // parse headers
                if line.starts_with("#") {
                    // ignore tags for now
                } else if line.starts_with("x") {
                    // header
                    let (x_str, y_str) = line.split_once(',').expect("Failed to parse header");
                    let x: usize = x_str[4..]
                        .parse()
                        .expect(format!("Failed to parse x '{}'", &x_str[4..]).as_str());
                    let y: usize = y_str[5..]
                        .parse()
                        .expect(format!("Failed to parse y '{}'", &y_str[5..]).as_str());

                    life = Some(Self::new(LifeAlgoSelect::Cached, (x, y)));
                } else {
                    panic!("Unkown line: {}", line);
                }
            } else {
                let mut run_count = 0;
                for chr in line.chars() {
                    if let Some(count) = chr.to_digit(10) {
                        run_count = (run_count * 10) + count;
                    } else {
                        if run_count == 0 {
                            run_count = 1;
                        }
                        match chr {
                            'b' => pos.0 += run_count as usize,
                            'o' => {
                                for _ in 0..run_count {
                                    if let Some(life) = &mut life {
                                        life.algo.insert(pos, Cell::new(1, 0));
                                    }
                                    pos.0 += 1;
                                }
                            }
                            '$' => {
                                pos.1 += 1;
                                pos.0 = 0;
                            }
                            '!' => break,

                            _ => panic!("Unkown <tag> '{chr}'"),
                        }
                        run_count = 0;
                    }
                }
            }
        }
        life.expect("Failed to parse header from .rle!")
    }

    pub fn paste(&mut self, other: &Self, pos: (usize, usize)) {
        for (x, y, cell) in other.iter() {
            self.insert((pos.0 + x, pos.1 + y), *cell);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &Cell)> {
        let size = self.algo.size();
        (0..size.1).flat_map(move |y: usize| {
            (0..size.1).map(move |x| (x, y, self.algo.get((x, y)).unwrap()))
        })
    }

    pub fn update(&mut self) {
        self.algo.update(&self.rule);
    }

    pub fn size(&self) -> (usize, usize) {
        self.algo.size()
    }

    pub fn insert(&mut self, pos: (usize, usize), cell: Cell) {
        self.algo.insert(pos, cell);
    }

    pub fn hash(&self, state: &mut DefaultHasher) {
        self.algo.hash(state);
    }
}

// pub fn iter_life_mut<'a>(life: &'a mut dyn Life) -> impl Iterator<Item = (usize, usize, &'a mut u8)> {
//     let size = life.size();
//     (0..size.1).flat_map(move |y: usize| (0..size.1).map(move |x| (x, y, life.get_mut((x, y)).unwrap())))
// }

// Should this be Display or Debug?
impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, _y, cell) in self.iter() {
            if x == 0 {
                f.write_char('\n')?;
            }
            if cell.is_alive() {
                f.write_char('*')?;
            } else {
                f.write_char(' ')?;
            }
        }
        Ok(())
    }
}

pub const GLIDER_RLE: &str = "\
#C This is a glider.
x = 3, y = 3
bo$2bo$3o!";

#[cfg(test)]
mod life_tests {
    use super::*;

    #[test]
    fn test_rle_glider() {
        let life = Life::new_life_from_rle(GLIDER_RLE);

        assert_eq!(life.algo.size(), (3, 3));

        assert_eq!(format!("{life}"), "\n * \n  *\n***");
    }

    /*
    #N Gosper glider gun
    #C This was the first gun discovered.
    #C As its name suggests, it was discovered by Bill Gosper.
    x = 36, y = 9, rule = B3/S23
    24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
    obo$10bo5bo7bo$11bo3bo$12b2o!
     */
}
