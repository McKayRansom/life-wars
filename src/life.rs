use std::{fmt::Write, hash::DefaultHasher, str::Split};

use basic::LifeBasic;
use cached::LifeCached;

mod basic;
mod cached;
// mod sparse;
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

pub trait LifeAlgo {
    fn size(&self) -> (usize, usize);
    fn get(&self, pos: (usize, usize)) -> Option<&Cell>;
    fn insert(&mut self, pos: (usize, usize), cell: Cell) -> Option<Cell>;
    fn update(&mut self, rule: &LifeRule, pops: &mut LifePops);
    fn hash(&self, state: &mut DefaultHasher);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct LifeRule {
    // TODO: Changing this to a u32 doesn't seem to impact us much
    lut: [u32; 4],
}

impl LifeRule {
    // GOL B3/S23
    pub const GOL: Self = Self::new([0b1_00_00_00, 0b01_01_00_00, 0, 0]);
    // SWR B2/S345/4
    pub const STAR_WARS: Self = Self::new([
        0b1_00_00,
        0b10_10_10_01_01_01_10_10_10,
        0b11_11_11_11_11_11_11_11_11,
        0,
    ]);

    pub const fn new(lut: [u32; 4]) -> Self {
        Self { lut }
    }

    // BXX or X
    fn parse_rule_portion<F>(it: &mut Split<'_, char>, mut func: F)
    where
        F: FnMut(u32) -> (),
    {
        if let Some(portion) = it.next() {
            for chr in portion.chars() {
                if let Some(count) = chr.to_digit(10) {
                    func(count)
                }
            }
        }
    }

    // BXX/SXX or BXX/SXX/X
    // https://conwaylife.com/wiki/Rulestring
    pub fn from_str(str: &str) -> Self {
        let mut new_rule: Self = Self::new([0, 0, 0, 0]);

        let mut portion_it = str.split('/');

        Self::parse_rule_portion(&mut portion_it, |count| new_rule.lut[0] |= 1 << (count * 2));
        Self::parse_rule_portion(&mut portion_it, |count| new_rule.lut[1] |= 1 << (count * 2));
        Self::parse_rule_portion(&mut portion_it, |count| {
            match count {
                3 => {} // do nothing
                4 => {
                    // Transition to state 2 instead of state 0
                    for i in 0..9 {
                        if new_rule.lut[1] & 1 << (i * 2) == 0 {
                            new_rule.lut[1] |= 2 << (i * 2);
                        }
                    }
                    new_rule.lut[2] = 0b11_11_11_11_11_11_11_11_11;
                }
                _ => unimplemented!(),
            }
        });

        new_rule
    }

    pub fn to_str(&self) -> String {
        let mut births: u32 = 0;
        let mut survives: u32 = 0;
        for i in 1..9 {
            if (self.lut[0] & (1 << i * 2)) != 0 {
                births = births * 10 | i;
            }
            if (self.lut[1] & (1 << i * 2)) != 0 {
                survives = (survives * 10) + i;
            }
        }
        if self.lut[2] == 0 {
            format!("B{births}/S{survives}")
        } else {
            format!("B{births}/S{survives}/4")
        }
    }

    pub fn update(&self, state: u8, (neighbors, faction): (u8, u8)) -> Cell {
        Cell::new(Self::state_update_f(self, state, neighbors), faction)
    }

    fn state_update_f(&self, state: u8, neighbors: u8) -> u8 {
        ((self.lut[state as usize] & (3 << (neighbors as u32 * 2))) >> (neighbors as u32 * 2)) as u8
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
            algo: Box::new(LifeBasic::new((8, 8))),
            rule: LifeRule::GOL,
            pops: LifePops::new(),
            generation: 0,
            name: String::new(),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum LifeAlgoSelect {
    #[default]
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
        let str = self.life_to_rle();
        println!("cloneing: {str}");
        Self::new_life_from_rle(str.as_str())
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

    fn rle_parse_header(it: &mut Split<'_, char>) -> Option<Self> {
        let mut name = String::new();
        while let Some(line) = it.next() {
            // parse headers
            if line.starts_with("#N ") {
                name = line[3..].into()
            } else if line.starts_with("#") {
                // ignore tags for now
            } else if line.starts_with("x") {
                // header
                let mut size: (usize, usize) = (16, 16);
                let mut rule: LifeRule = LifeRule::GOL;
                for field in line.split(", ") {
                    let (name, value) = field.split_once(" = ").expect("Failed to parse field");
                    match name {
                        "x" => size.0 = value.parse().expect("Failed to parse field"),
                        "y" => size.1 = value.parse().expect("Failed to parse field"),
                        "rule" => rule = LifeRule::from_str(field),
                        _ => panic!("Unkown header field: {}", name),
                    }
                }
                return Some(Self {
                    rule,
                    name,
                    ..Self::new(LifeAlgoSelect::Basic, size)
                });
            } else {
                panic!("Unkown line: {}", line);
            }
        }
        None
    }

    /*
     *        let rle_glider = "
     *        #C This is a glider.
     *        x = 3, y = 3
     *        bo$2bo$3o!";
     * https://conwaylife.com/wiki/Run_Length_Encoded
     */
    pub fn new_life_from_rle(rle: &str) -> Self {
        let mut line_it = rle.split('\n');
        let mut life: Self =
            Self::rle_parse_header(&mut line_it).expect("Failed to parse header from .rle!");

        let mut pos: (usize, usize) = (0, 0);
        while let Some(line) = line_it.next() {
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
                                life.algo.insert(pos, Cell::new(1, 0));
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
        life
    }

    fn rle_write_state(string: &mut String, count: i32, state: u8, line_count: &mut usize) {
        if count > 1 {
            let count_str = count.to_string();
            *line_count += count_str.len();
            string.push_str(count_str.as_str());
        }
        let pat = match state {
            0 => 'b',
            1 => 'o',
            _ => todo!(),
        };
        string.push(pat);
        *line_count += 1;
        if *line_count > 64 {
            string.push('\n');
            *line_count = 0;
        }
    }

    pub fn life_to_rle(&self) -> String {
        let size = self.size();
        let mut string = String::with_capacity(64);
        if !self.name.is_empty() {
            string.push_str("#N ");
            string.push_str(self.name.as_str());
            string.push('\n');
        }
        string.push_str(
            format!(
                "x = {}, y = {}, rule = {}\n",
                size.0,
                size.1,
                self.rule.to_str().as_str()
            )
            .as_str(),
        );

        let mut prev_count = 0;
        let mut prev_state: Option<u8> = None;
        let mut line_count: usize = 0;
        for (x, y, cell) in self.iter() {
            if x == 0 && y != 0 {
                if prev_state.unwrap() != 0 {
                    Self::rle_write_state(
                        &mut string,
                        prev_count,
                        prev_state.unwrap(),
                        &mut line_count,
                    );
                }

                prev_state = None;
                prev_count = 0;
                string.push('$');
            }
            if let Some(state) = prev_state {
                if state != cell.get_state() {
                    Self::rle_write_state(&mut string, prev_count, state, &mut line_count);
                    prev_count = 0;
                }
            }
            prev_state = Some(cell.get_state());
            prev_count += 1;
        }
        if prev_state.unwrap() != 0 {
            Self::rle_write_state(
                &mut string,
                prev_count,
                prev_state.unwrap(),
                &mut line_count,
            );
        }

        string.push('!');

        string
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

pub const GOSPER_RLE: &str = "\
#N Gosper glider gun
x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
obo$10bo5bo7bo$11bo3bo$12b2o!";

#[cfg(test)]
mod life_tests {
    use super::*;

    #[test]
    fn test_rle_glider() {
        let life = Life::new_life_from_rle(GLIDER_RLE);

        assert_eq!(life.algo.size(), (3, 3));
        assert_eq!(format!("{life}"), "\n * \n  *\n***");
        // don't compare glider rules
        assert_eq!(life.life_to_rle()[28..], GLIDER_RLE[34..]);
    }

    #[test]
    fn test_rle_gosper() {
        let life = Life::new_life_from_rle(GOSPER_RLE);
        assert_eq!(life.rule, LifeRule::GOL);
        assert_eq!(life.size(), (36, 9));
        assert_eq!(life.algo.get((24, 0)).unwrap(), &Cell::new(1, 0));
        assert_eq!(life.life_to_rle(), GOSPER_RLE);
    }

    #[test]
    fn test_rule_from_str() {
        assert_eq!(LifeRule::from_str("B3/S23"), LifeRule::GOL);
        assert_eq!(LifeRule::from_str("B2/S345/4"), LifeRule::STAR_WARS);
    }

    #[test]
    fn test_rule_to_str() {
        assert_eq!(LifeRule::GOL.to_str(), "B3/S23");
        assert_eq!(LifeRule::STAR_WARS.to_str(), "B2/S345/4");
    }

    #[test]
    fn test_rule_life() {
        // GOL B3/S23
        let rule = LifeRule::GOL;

        // Birth
        assert_eq!(rule.update(0, (2, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(0, (3, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(0, (4, 0)), Cell::new(0, 0));

        // Survive
        assert_eq!(rule.update(1, (1, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(1, (2, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(1, (3, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(1, (4, 0)), Cell::new(0, 0));
    }

    #[test]
    fn test_rule_sw() {
        // SW B2/S345/4
        let rule = LifeRule::STAR_WARS;

        // Birth
        assert_eq!(rule.update(0, (1, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(0, (2, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(0, (3, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(0, (4, 0)), Cell::new(0, 0));

        // Survive
        assert_eq!(rule.update(1, (3, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(1, (4, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(1, (5, 0)), Cell::new(1, 0));

        // Refractory
        assert_eq!(rule.update(1, (1, 0)), Cell::new(2, 0));
        assert_eq!(rule.update(1, (2, 0)), Cell::new(2, 0));
        assert_eq!(rule.update(1, (6, 0)), Cell::new(2, 0));

        // Refractory 2
        assert_eq!(rule.update(2, (1, 0)), Cell::new(3, 0));
        assert_eq!(rule.update(2, (3, 0)), Cell::new(3, 0));
        assert_eq!(rule.update(2, (6, 0)), Cell::new(3, 0));

        // Refractory 3
        assert_eq!(rule.update(3, (1, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(3, (3, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(3, (6, 0)), Cell::new(0, 0));
    }
}
