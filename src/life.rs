use std::hash::DefaultHasher;

mod rule;
pub use rule::LifeRule;

mod algo;
pub use algo::FACTION_ALGOS;
use algo::LifeAlgo;
pub use algo::LifeAlgoSelect;
pub use algo::WORKING_ALGOS;

mod file_format;
pub use file_format::rle::*;
pub use file_format::plaintext::*;

pub mod patterns;

pub const FACTION_MAX: usize = 16;

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

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        Self { value }
    }
}

pub struct LifePops {
    pops: [i16; FACTION_MAX],
}

impl LifePops {
    pub fn new() -> Self {
        Self {
            pops: [0; FACTION_MAX],
        }
    }

    pub fn get(&self, faction: u8) -> i16 {
        self.pops[faction as usize]
    }

    pub fn add(&mut self, faction: u8, amount: i16) {
        self.pops[faction as usize] = self.pops[faction as usize].saturating_add(amount)
    }
}

pub struct Life {
    algo: Box<dyn LifeAlgo>,
    rule: LifeRule,
    pops: LifePops,
    generation: u64,
    name: String,
}

impl Default for Life {
    fn default() -> Self {
        Self {
            algo: algo::new(LifeAlgoSelect::Basic, (8, 8)),
            rule: LifeRule::GOL,
            pops: LifePops::new(),
            generation: 0,
            name: String::new(),
        }
    }
}

impl Life {
    pub fn new(algo: LifeAlgoSelect, size: (usize, usize)) -> Self {
        Self {
            algo: algo::new(algo, size),
            ..Default::default()
        }
    }

    pub fn new_rule(algo: LifeAlgoSelect, size: (usize, usize), rule: LifeRule) -> Self {
        Self {
            rule,
            ..Self::new(algo, size)
        }
    }

    pub fn get_rule(&self) -> &LifeRule {
        &self.rule
    }

    pub fn get_generation(&self) -> u64 {
        self.generation
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_cell(&self, pos: (usize, usize)) -> Option<&Cell> {
        self.algo.get(pos)
    }

    pub fn clone(&self) -> Self {
        // this is stupid AF LOLOL
        let str = life_to_rle(self);
        println!("cloneing: {str}");
        new_life_from_rle(str.as_str())
        // Self {
        //     algo: self.algo.clone(),
        //     rule: self.rule,
        //     pops: self.pops,
        //     generation: self.generation,
        //     name: self.name.clone(),
        // }
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
                            if !use_factions || (y < size.1 / 4 || y > (size.1 * 3) / 4) {
                                1
                            } else {
                                0
                            }
                        } else {
                            0
                        },
                        if use_factions && y < size.1 / 2 { 1 } else { 0 },
                    ),
                );
            }
        }
    }



    pub fn paste(&mut self, other: &Self, pos: (usize, usize)) {
        for (x, y, cell) in other.iter() {
            self.insert((pos.0 + x, pos.1 + y), *cell);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &Cell)> {
        let size = self.algo.size();
        (0..size.1).flat_map(move |y: usize| {
            (0..size.0).map(move |x| (x, y, self.algo.get((x, y)).unwrap()))
        })
    }

    pub fn update(&mut self) {
        self.algo.update(&self.rule, &mut self.pops);
        self.generation = self.generation.saturating_add(1);
    }

    pub fn size(&self) -> (usize, usize) {
        self.algo.size()
    }

    pub fn insert(&mut self, pos: (usize, usize), cell: Cell) {
        if let Some(old_cell) = self.algo.get(pos) {
            if old_cell != &cell {
                // TODO: Is this edge case the reason cached is failing for multiple factions?
                if old_cell.is_alive() {
                    self.pops.add(old_cell.get_faction(), -1);
                }
                if cell.is_alive() {
                    self.pops.add(cell.get_faction(), 1);
                }
                self.algo.insert(pos, cell);
            }
        }
    }

    pub fn hash(&self, state: &mut DefaultHasher) {
        self.algo.hash(state);
    }

    pub fn get_pop(&self, faction: u8) -> i16 {
        self.pops.get(faction)
    }

    pub fn set_name(&mut self, as_str: &str) {
        self.name = as_str.into();
    }
}

// TODO: TryFrom instead...
impl From<&str> for Life {
    fn from(value: &str) -> Self {
        from_plaintext(value, None)
    }
}

// Should this be Display or Debug?
impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(life_to_plaintext(self).as_str())
    }
}

pub const GLIDER_RLE: &str = "\
#C This is a glider.
x = 3, y = 3
bo$2bo$3o!";

pub const GOSPER_RLE: &str = "\
#N Gosper glider gun
x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
obo$10bo5bo7bo$11bo3bo$12b2o!";

pub const STAR_WARS_RLE: &str = "\
x = 43, y = 48, rule = B2/S345/4
2.ABC$2.A2.A$.6A.A$2.A3.2A.B$A.A4.2A.C$B2A5.2A$C.A5.A$2.A5.A4.CBA$.
10A3.A25.CB$2.A2.A2.A.B2.3A23.2A.A$.ABC3.ABC4.A25.3A$13.ABC23.BA.B$
39.A.C$6.A4.A6.A$5.15A$6.A2.A2.A2.A2.A9$15.C$14.A.B$13.4A$12.A.A$12.B
.A$12.C3A$14.A$14.A.C$4.ABC6.3AB$2.A2.A8.A.A$.6A5.A.A$2.A3.2A4.B3A$2.
A4.2A3.C.A24.CBA$.2A5.3A3.A25.A$A.A5.A.B2.3AC22.3A$B.A5.A.C3.A.B21.B
2A.A$C9A4.A.A23.CB$2.A2.A2.A3.4A$4.ABC5.B.A$11.2AC$11.AB$6.A4.A6.A$5.
15A$6.A2.A2.A2.A2.A!";

// TODO: Descriptions??
/*
!Author: Richard K. Guy
!The smallest, most common, and first discovered spaceship.
!www.conwaylife.com/wiki/index.php?title=Glider
*/
pub const GLIDER_TXT: &str = "\
!Name: Glider
.O.
..O
OOO";

#[cfg(test)]
mod life_tests {
    use super::*;

    #[test]
    fn test_txt_glider() {
        let life: Life = GLIDER_TXT.into();
        assert_eq!(format!("{life}"), GLIDER_TXT);
    }

    #[test]
    fn test_rle_glider() {
        let life = new_life_from_rle(GLIDER_RLE);

        assert_eq!(life.algo.size(), (3, 3));
        // don't compare glider rules
        assert_eq!(life_to_rle(&life)[28..], GLIDER_RLE[34..]);
    }

    #[test]
    fn test_rle_gosper() {
        let life = new_life_from_rle(GOSPER_RLE);
        assert_eq!(life.rule, LifeRule::GOL);
        assert_eq!(life.size(), (36, 9));
        assert_eq!(life.algo.get((24, 0)).unwrap(), &Cell::new(1, 0));
        assert_eq!(life_to_rle(&life), GOSPER_RLE);
    }

    #[test]
    fn test_rle_star_wars() {
        let life = new_life_from_rle(STAR_WARS_RLE);
        assert_eq!(life.rule, LifeRule::STAR_WARS);
        let new_rle = life_to_rle(&life);

        for line in new_rle.lines() {
            println!("{line}")
        }

        for line in STAR_WARS_RLE.lines() {
            println!("{line}")
        }
        
        assert_eq!(new_rle, STAR_WARS_RLE);
    }
}

/*
 2.ABC$2.A2.A$.6A.A$2.A3.2A.B$A.A4.2A.C$B2A5.2A$C.A5.A$2.A5.A4.CBA$.                                                                                                                                                                                                                                                                                                              ▐
 10A3.A25.CB$2.A2.A2.A.B2.3A23.2A.A$.ABC3.ABC4.A25.3A$13.ABC23.BA.B$                                                                                                                                                                                                                                                                                                              ▐
 39.A.C$6.A4.A6.A$5.15A$6.A2.A2.A2.A2.A$15.C$14.A.B$13.4A$12.A.A$12.B                                                                                                                                                                                                                                                                                                             ▐
 39.A.C$6.A4.A6.A$5.15A$6.A2.A2.A2.A2.A9$15.C$14.A.B$13.4A$12.A.A$12.B                                                                                                                                                                                                                                                                                                            ▐
 .A$12.C3A$14.A$14.A.C$4.ABC6.3AB$2.A2.A8.A.A$.6A5.A.A$2.A3.2A4.B3A$                                                                                                                                                                                                                                                                                                              ▐
 2.A4.2A3.C.A24.CBA$.2A5.3A3.A25.A$A.A5.A.B2.3AC22.3A$B.A5.A.C3.A.B                                                                                                                                                                                                                                                                                                               ▐
 21.B2A.A$C9A4.A.A23.CB$2.A2.A2.A3.4A$4.ABC5.B.A$11.2AC$11.AB$6.A4.A                                                                                                                                                                                                                                                                                                              ▐
 6.A$5.15A$6.A2.A2.A2.A2.A$$$$$$$$!                                                                                                                                                                                                                                                                                                                                               ▐
 x = 43, y = 48, rule = B2/S345/4                                                                                                                                                                                                                                                                                                                                                 ▐
 2.ABC$2.A2.A$.6A.A$2.A3.2A.B$A.A4.2A.C$B2A5.2A$C.A5.A$2.A5.A4.CBA$.                                                                                                                                                                                                                                                                                                              ▐
 10A3.A25.CB$2.A2.A2.A.B2.3A23.2A.A$.ABC3.ABC4.A25.3A$13.ABC23.BA.B$                                                                                                                                                                                                                                                                                                              ▐
 .A$12.C3A$14.A$14.A.C$4.ABC6.3AB$2.A2.A8.A.A$.6A5.A.A$2.A3.2A4.B3A$2.                                                                                                                                                                                                                                                                                                            ▐
 A4.2A3.C.A24.CBA$.2A5.3A3.A25.A$A.A5.A.B2.3AC22.3A$B.A5.A.C3.A.B21.B                                                                                                                                                                                                                                                                                                             ▐
 2A.A$C9A4.A.A23.CB$2.A2.A2.A3.4A$4.ABC5.B.A$11.2AC$11.AB$6.A4.A6.A$5.                                                                                                                                                                                                                                                                                                            ▐
 15A$6.A2.A2.A2.A2.A!  
 */