use std::hash::DefaultHasher;


use super::{Cell, LifePops, LifeRule, Pos};


mod basic;
mod cached;
mod sparse;
mod quick;
// mod quad;

#[derive(Clone, Copy, Default, Debug)]
pub enum LifeAlgoSelect {
    #[default]
    Basic,
    Cached,
    Sprase,
    // Quick,
    // Hash,
}

/// Algorithms working correctly with any ruleset
pub const WORKING_ALGOS: &[LifeAlgoSelect] = &[
    LifeAlgoSelect::Basic,
    LifeAlgoSelect::Cached,
    LifeAlgoSelect::Sprase,
];

/// Algorithms working correctly with multiple factions
pub const FACTION_ALGOS: &[LifeAlgoSelect] = &[
    LifeAlgoSelect::Basic,
    LifeAlgoSelect::Cached, // Cached ONLY WORKS for 2 factions...
];


pub trait LifeAlgo {
    /// Get the size of the life grid
    fn size(&self) -> Pos;
    /// Get a cell at a position
    fn get(&self, pos: Pos) -> Option<&Cell>;
    /// Insert a cell at a position, return old cell if it was present
    fn insert(&mut self, pos: Pos, cell: Cell) -> Option<Cell>;
    /// Advance life 1 tick with given rule
    fn update(&mut self, rule: &LifeRule, pops: &mut LifePops);
    /// Hash is NOT consistent accross algorithms, use pop for those cases
    fn hash(&self, state: &mut DefaultHasher);
}

pub fn new(algo: LifeAlgoSelect, size: Pos) -> Box<dyn LifeAlgo> {
    match algo {
        LifeAlgoSelect::Basic => Box::new(basic::LifeBasic::new(size)),
        LifeAlgoSelect::Cached => Box::new(cached::LifeCached::new(size)),
        LifeAlgoSelect::Sprase => Box::new(sparse::LifeSparse::new(size)),
    }
}

