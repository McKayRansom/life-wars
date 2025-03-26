/*
 * built-in patterns
 */

use crate::{pattern::Pattern, storage::{SaveError, Storage}};

use super::life_to_rle;

pub const GLIDER_RLE: &str = "\
#N Glider
x = 3, y = 3
bo$2bo$3o!";

pub const GOSPER_RLE: &str = "\
#N Gosper glider gun
x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
obo$10bo5bo7bo$11bo3bo$12b2o!";

// pub const DOMINO_RLE: &str = "\
// #N Domino
// x = 3, y = 2, rule = B3/S23
// ";

pub const BUILT_IN_PATTERNS: &[&str] = &[GLIDER_RLE, GOSPER_RLE];

pub struct PatternLib {
    pub patterns: Vec<Pattern>,
    storage: Storage,
}

impl PatternLib {
    pub fn new() -> Self {
        let storage = Storage::new("save.ron");
        Self {
            patterns: if let Ok(patterns_strings) = Self::load(&storage) {
                println!("Pattern lib loaded {} patterns", patterns_strings.len());
                patterns_strings
                    .iter()
                    .map(|rle| super::new_pattern_from_rle(rle.as_str()))
                    .collect()
            } else {
                println!("Pattern lib NOT FOUND!");
                BUILT_IN_PATTERNS
                    .iter()
                    .map(|rle| super::new_pattern_from_rle(rle))
                    .collect()
            },
            storage,
        }
    }

    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(pattern);

        if self.save().is_err() {
            println!("ERROR SAVING!");
        }
    }

    pub fn load(storage: &Storage) -> Result<Vec<String>, SaveError> {
        let string = storage.load()?;
        Ok(ron::from_str(string.as_str())?)
    }

    pub fn save(&self) -> Result<(), SaveError> {
        let pattern_strings: Vec<String> = self
            .patterns
            .iter()
            .map(life_to_rle)
            .collect();
        self.storage.save_as_ron(&pattern_strings)
    }
}


impl Default for PatternLib {
    fn default() -> Self {
        Self::new()
    }
}

