use std::hash::DefaultHasher;
use std::hash::Hasher;

mod rule;
pub use rule::LifeRule;

mod algo;
pub use algo::FACTION_ALGOS;
use algo::LifeAlgo;
pub use algo::LifeAlgoSelect;
pub use algo::WORKING_ALGOS;

pub mod rand;
mod pos;
pub use pos::Pos;

mod file_format;

pub mod pattern_lib;

pub const FACTION_MAX: usize = 16;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Default)]
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

    /// NOTE: is_alive() != !is_dead() due to refactory states
    pub fn is_alive(&self) -> bool {
        self.value & Self::STATE_MASK == 0x1
    }

    /// NOTE: is_dead() != !is_alive() due to refactory states
    pub fn is_dead(&self) -> bool {
        self.value & Self::STATE_MASK == 0
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


#[derive(Default)]
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


#[derive(Default)]
pub struct LifeOptions {
    pub algo: LifeAlgoSelect,
    pub rule: LifeRule,
}

pub struct Life {
    algo: Box<dyn LifeAlgo>,
    rule: LifeRule,
    pops: LifePops,
    generation: u64,
}

impl Default for Life {
    fn default() -> Self {
        Self {
            algo: algo::new(LifeAlgoSelect::Basic, (8, 8)),
            rule: LifeRule::GOL,
            pops: LifePops::new(),
            generation: 0,
        }
    }
}

impl Life {
    pub fn new(size: (u16, u16)) -> Self {
        Self {
            algo: algo::new(LifeAlgoSelect::Basic, size),
            ..Default::default()
        }
    }

    pub fn new_rule(size: (u16, u16), rule: LifeRule) -> Self {
        Self {
            algo: algo::new(LifeAlgoSelect::Basic, size),
            rule,
            ..Default::default()
        }
    }

    pub fn new_ex(size: (u16, u16), options: LifeOptions) -> Self {
        Self {
            rule: options.rule,
            algo: algo::new(options.algo, size),
            ..Default::default()
        }
    }

    pub fn get_rule(&self) -> &LifeRule {
        &self.rule
    }

    pub fn get_generation(&self) -> u64 {
        self.generation
    }

    pub fn get_cell(&self, pos: (u16, u16)) -> Option<&Cell> {
        self.algo.get(pos)
    }


    // fn iter_mut(&mut self) -> impl Iterator<Item = (u16, u16, &mut u8)>;
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

    pub fn paste(&mut self, other: &Self, pos: (u16, u16), faction: Option<u8>) {
        let faction = faction.unwrap_or(0);
        for (x, y, cell) in other.iter() {
            let cell = Cell::new(cell.get_state(), faction);
            self.insert((pos.0 + x, pos.1 + y), cell);
        }
    }

    pub fn copy(&self, pos: (u16, u16), area: (u16, u16)) -> Self {
        let mut life = Life::new_ex(
            area,
            LifeOptions { algo: LifeAlgoSelect::Basic, rule: self.rule }
        );

        let start_pos: Pos = pos.into();

        for pos in start_pos.iter(area.into()) {
            if let Some(cell) = self.get_cell(pos.into()) {
                life.insert((pos - start_pos).into(), *cell);
            }
        }

        life
    }

    pub fn iter(&self) -> impl Iterator<Item = (u16, u16, &Cell)> {
        let size = self.algo.size();
        (0..size.1).flat_map(move |y: u16| {
            (0..size.0).map(move |x| (x, y, self.algo.get((x, y)).unwrap()))
        })
    }

    pub fn update(&mut self) {
        self.algo.update(&self.rule, &mut self.pops);
        self.generation = self.generation.saturating_add(1);
    }

    pub fn size(&self) -> (u16, u16) {
        self.algo.size()
    }

    pub fn insert(&mut self, pos: (u16, u16), cell: Cell) {
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

    pub fn hash(&self) -> u64 {
        // TODO: Use other hasher?
        let mut hasher = DefaultHasher::new();
        self.algo.hash(&mut hasher);
        hasher.finish()
    }

    pub fn get_pop(&self, faction: u8) -> i16 {
        self.pops.get(faction)
    }
}

impl Clone for Life {
    fn clone(&self) -> Self {
        // this is stupid AF LOLOL
        let str = self.to_string();
        println!("cloneing: {str}");
        str.as_str().into()
        // Self {
        //     algo: self.algo.clone(),
        //     rule: self.rule,
        //     pops: self.pops,
        //     generation: self.generation,
        //     name: self.name.clone(),
        // }
    }
}

// TODO: TryFrom instead...
impl From<&str> for Life {
    fn from(value: &str) -> Self {
        Life::from_plaintext(value, LifeOptions::default())
    }
}

// Should this be Display or Debug?
impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_plaintext().as_str())
    }
}
