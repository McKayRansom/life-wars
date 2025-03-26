/*
 * Attempts to identify patterns in a Life Grid
 */

use crate::life::{Life, Pos};

use super::Pattern;

#[derive(Debug)]
struct CellGroup {
    top_left_pos: Pos,
    bot_right_pos: Pos,
}

impl CellGroup {
    pub fn new(pos: Pos) -> Self {
        Self {
            top_left_pos: pos,
            bot_right_pos: pos,
        }
    }

    #[allow(unused)]
    fn within(&self, pos: Pos) -> bool {
        pos.x >= self.top_left_pos.x
            && pos.x <= self.bot_right_pos.x
            && pos.y >= self.top_left_pos.y
            && pos.y <= self.bot_right_pos.y
    }

    fn add(&mut self, pos: Pos) {
        if pos.x < self.top_left_pos.x {
            self.top_left_pos.x = pos.x;
        }
        if pos.y < self.top_left_pos.y {
            self.top_left_pos.y = pos.y;
        }
        if pos.x > self.bot_right_pos.x {
            self.bot_right_pos.x = pos.x;
        }
        if pos.y > self.bot_right_pos.y {
            self.bot_right_pos.y = pos.y;
        }
    }
}

#[derive(Debug)]
struct CellGroupTracker {
    group_grid_map: Vec<Vec<u8>>,
    next_group_id: u8,
    groups: Vec<CellGroup>,
}

impl CellGroupTracker {
    pub fn new(life: &Life) -> Self {
        let size = life.size();
        let mut tracker = Self {
            group_grid_map: vec![vec![0; size.0 as usize]; size.1 as usize],
            next_group_id: 1,
            groups: Vec::new(),
        };

        tracker.setup_tracking(life);

        tracker
    }

    const NEIGHBOR_OFFSETS: &[(i32, i32)] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    fn current_group_for_cell(&self, pos: (u16, u16), offset: (i32, i32)) -> u8 {
        let final_pos: (i32, i32) = (pos.0 as i32 + offset.0, pos.1 as i32 + offset.1);

        // TODO: THIS IS STUPID!
        if final_pos.0 >= 0
            && final_pos.0 < self.group_grid_map[0].len() as i32
            && final_pos.1 >= 0
            && final_pos.1 < self.group_grid_map.len() as i32
        {
            self.group_grid_map[final_pos.1 as usize][final_pos.0 as usize]
        } else {
            0
        }
    }

    fn calc_group_for_cell(&mut self, pos: (u16, u16)) -> u8 {
        for neigh_off in Self::NEIGHBOR_OFFSETS {
            let neigh_group = self.current_group_for_cell(pos, *neigh_off);
            if neigh_group > 0 {
                self.groups[neigh_group as usize - 1].add(pos.into());
                return neigh_group;
            }
        }
        let group_id = self.next_group_id;
        self.next_group_id += 1;
        self.groups.push(CellGroup::new(pos.into()));
        group_id
    }

    fn setup_tracking(&mut self, life: &Life) {
        for (x, y, cell) in life.iter() {
            if cell.is_alive() {
                self.group_grid_map[y as usize][x as usize] = self.calc_group_for_cell((x, y));
            }
        }
    }

    fn update(&mut self, life: &Life) {
        for (x, y, cell) in life.iter() {
            if cell.is_alive() {
                self.group_grid_map[y as usize][x as usize] = self.calc_group_for_cell((x, y));
            }
        }
    }

    fn to_patterns(&self, life: &Life) -> Vec<Pattern> {
        self.groups
            .iter()
            .enumerate()
            .map(|(_group_id, group_extents)| {
                let size = group_extents.bot_right_pos - group_extents.top_left_pos + (1, 1).into();
                let mut new_life = Life::new(size.into());
                for pos in group_extents.top_left_pos.iter(size) {
                    new_life.insert(
                        (pos - group_extents.top_left_pos).into(),
                        *life.get_cell(pos.into()).unwrap(),
                    );
                }
                let mut pattern = Pattern::new_unclassified(new_life);
                pattern.classify();
                pattern
            })
            .collect()
    }
}

pub fn identify(life: &mut Life) -> Vec<Pattern> {
    let mut tracker = CellGroupTracker::new(life);

    life.update();
    tracker.update(life);

    tracker.to_patterns(life)
}

#[cfg(test)]
mod identify_tests {
    use crate::life::LifeOptions;

    use super::*;

    #[test]
    fn test_cell_group_tracker() {
        let life = Life::from_plaintext(
            "\
OO..OO
OO..OO",
            LifeOptions::default(),
        );
        let tracker = CellGroupTracker::new(&life);

        assert_eq!(tracker.group_grid_map[1][1], 1);
        assert_eq!(tracker.group_grid_map[1][4], 2);
    }

    #[test]
    fn test_cell_group_tracker_blinker() {
        let mut life = Life::from_plaintext(
            "\
.O.
.O.
.O.",
            LifeOptions::default(),
        );
        let mut tracker = CellGroupTracker::new(&life);

        assert_eq!(tracker.group_grid_map[1][1], 1);

        life.update();
        tracker.update(&life);

        assert_eq!(tracker.group_grid_map[1][0], 1);
    }

    #[test]
    fn test_identify_block() {
        let mut life = Life::from_plaintext(
            "\
OO..OO
OO..OO",
            LifeOptions::default(),
        );

        let patterns = identify(&mut life);

        assert_eq!(patterns[0].to_apgcode(), "xs4_33");
        assert_eq!(patterns[1].to_apgcode(), "xs4_33");
    }

    #[test]
    fn test_identify_block_blink() {
        let mut life = Life::from_plaintext(
            "\
OO..O.
OO..O.
....O.",
            LifeOptions::default(),
        );

        let patterns = identify(&mut life);

        assert_eq!(patterns[0].to_apgcode(), "xs4_33");
        assert_eq!(patterns[1].to_apgcode(), "xp1_07");
    }

    #[test]
    #[ignore = "Requires a rethink"]
    fn messless_diehard() {
        const DIEHARD: &str = "\
!Name: Die hard
!A methuselah that vanishes at generation 130, which is conjectured to be maximal for patterns of 7 or fewer cells.
!https://www.conwaylife.com/wiki/index.php?title=Die_hard
......O
OO
.O...OOO";

        let mut life = Life::new_ex((64, 64), LifeOptions {
            algo: crate::life::LifeAlgoSelect::Cached,
            ..Default::default()
        });
        let die_hard_life = Life::from_plaintext(DIEHARD, LifeOptions::default());
        life.paste(&die_hard_life, (32, 32), None);

        let _patterns = identify(&mut life);

        // assert_eq!(patterns[0].metadata, PatternMetadata {
        //     classification: Some(Classification::Messless),
        //     ..Default::default()
        // });
    }
}
