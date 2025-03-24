/*
 * built-in patterns
 */

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
    pub patterns: Vec<super::Life>,
}

impl PatternLib {
    pub fn new() -> Self {
        Self {
            patterns: if let Ok(patterns_strings) = Self::load() {
                println!("Pattern lib loaded {} patterns", patterns_strings.len());
                patterns_strings
                    .iter()
                    .map(|rle| super::new_life_from_rle(rle.as_str()))
                    .collect()
            } else {
                println!("Pattern lib NOT FOUND!");
                BUILT_IN_PATTERNS
                    .iter()
                    .map(|rle| super::new_life_from_rle(*rle))
                    .collect()
            },
        }
    }

    pub fn add_pattern(&mut self, life: &super::Life) {
        self.patterns.push(life.clone());

        if self.save().is_err() {
            println!("ERROR SAVING!");
        }
    }
}

use ron::de::SpannedError;
// #[cfg(not(target_family = "wasm"))]
// use crate::dir;
// use serde::{Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
use std::path::PathBuf;

// use crate::map::Map;

#[cfg(not(target_family = "wasm"))]
use directories::ProjectDirs;

use super::life_to_rle;

#[cfg(not(target_family = "wasm"))]
/// returns the ProjectDirs struct from the directories crate with the proper identifier for the
/// game
pub fn project_dirs() -> ProjectDirs {
    #[cfg(test)]
    let dirs = ProjectDirs::from("com", "TilesRUs", "lifeIO-test").unwrap();
    #[cfg(not(test))]
    let dirs = ProjectDirs::from("com", "TilesRUs", "lifeIO").unwrap();
    dirs
}

#[cfg(not(target_family = "wasm"))]
const SAVE_FILE: &str = "save.ron";

#[cfg(target_family = "wasm")]
const WASM_SAVE_KEY: &str = "save";

#[derive(Debug)]
pub enum SaveError {
    #[allow(unused)]
    ReadFile(std::io::Error),
    #[allow(unused)]
    Deserialize(SpannedError),
    #[allow(unused)]
    Serialize(ron::Error),
}

impl From<std::io::Error> for SaveError {
    fn from(err: std::io::Error) -> Self {
        SaveError::ReadFile(err)
    }
}

impl From<SpannedError> for SaveError {
    fn from(err: SpannedError) -> Self {
        SaveError::Deserialize(err)
    }
}

impl From<ron::Error> for SaveError {
    fn from(err: ron::Error) -> Self {
        SaveError::Serialize(err)
    }
}

pub type LoadResult = Result<Vec<String>, SaveError>;
type SaveResult = Result<(), SaveError>;

impl PatternLib {
    #[cfg(not(target_family = "wasm"))]
    pub fn save_exists() -> bool {
        std::fs::exists(Self::determine_save_path()).unwrap()
    }

    #[cfg(target_family = "wasm")]
    pub fn save_exists() -> bool {
        quad_storage::STORAGE.lock().unwrap().len() > 0
    }

    pub fn load() -> LoadResult {
        #[cfg(not(target_family = "wasm"))]
        let map = Self::load_desktop();
        #[cfg(target_family = "wasm")]
        let map = Self::load_wasm();

        // match &mut map {
        //     Err(err) => println!("Error loading save: {:?}", *err),
        //     Ok(map) => map
        //         .fixup()
        //         .unwrap_or_else(|err| println!("Error fixing save! {err:?}")),
        // }

        map
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn load_desktop() -> LoadResult {
        let save_path = Self::determine_save_path();

        println!("loading from {:?}", save_path.as_os_str());

        let toml_str = std::fs::read_to_string(save_path)?;
        Ok(ron::from_str(toml_str.as_str())?)
    }

    #[cfg(not(target_family = "wasm"))]
    fn determine_save_path() -> PathBuf {
        let project_dirs = project_dirs();
        let save_dir = project_dirs.data_local_dir();
        std::fs::create_dir_all(save_dir).unwrap();
        let mut save_path = PathBuf::from(save_dir);
        save_path.push(SAVE_FILE);
        save_path
    }

    #[cfg(target_family = "wasm")]
    pub fn load_wasm() -> LoadResult {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        let wasm_save = storage.get(WASM_SAVE_KEY).unwrap();
        Ok(ron::from_str(wasm_save.as_str()).unwrap())
    }

    /// writes the save to local storage
    #[cfg(target_family = "wasm")]
    pub fn save(&self) -> SaveResult {
        // TODO: Solve unwrap
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        storage.set(WASM_SAVE_KEY, &self.to_ron_string().unwrap().as_str());
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    /// writes the save to disk
    pub fn save(&self) -> SaveResult {
        Ok(std::fs::write(
            Self::determine_save_path(),
            self.to_ron_string()?,
        )?)
    }

    /// returns the save data in RON format as a pretty string
    fn to_ron_string(&self) -> Result<String, SaveError> {
        let pattern_strings: Vec<String> = self
            .patterns
            .iter()
            .map(|pattern| life_to_rle(pattern))
            .collect();
        Ok(ron::ser::to_string_pretty(
            &pattern_strings,
            ron::ser::PrettyConfig::default(),
        )?)
    }
}

// #[cfg(test)]
// mod save_tests {
//     use super::*;

//     #[test]
//     fn test_map_serialize() {
//         let _ = std::fs::remove_file(Map::determine_save_path());

//         assert!(!Map::save_exists());

//         let mut map = Map::new_from_string(">>>>1");

//         // TODO
//         // map.add_vehicle((0, 0).into(), 1, crate::consts::SpawnerColors::Blue);

//         map.save().unwrap();

//         assert!(Map::save_exists());

//         let mut deserialized: Map = Map::load().unwrap();

//         deserialized.fixup().unwrap();

//         assert_eq!(deserialized, map);
//     }
// }
