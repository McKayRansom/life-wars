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
    groups: Vec<Vec<u8>>,
    next_group_id: u8,
    group_extents: Vec<CellGroup>,
}

impl CellGroupTracker {
    pub fn new(life: &Life) -> Self {
        let size = life.size();
        let mut tracker = Self {
            groups: vec![vec![0; size.0 as usize]; size.1 as usize],
            next_group_id: 1,
            group_extents: Vec::new(),
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
            && final_pos.0 < self.groups[0].len() as i32
            && final_pos.1 >= 0
            && final_pos.1 < self.groups.len() as i32
        {
            self.groups[final_pos.1 as usize][final_pos.0 as usize]
        } else {
            0
        }
    }

    fn calc_group_for_cell(&mut self, pos: (u16, u16)) -> u8 {
        for neigh_off in Self::NEIGHBOR_OFFSETS {
            let neigh_group = self.current_group_for_cell(pos, *neigh_off);
            if neigh_group > 0 {
                self.group_extents[neigh_group as usize - 1].add(pos.into());
                return neigh_group;
            }
        }
        let group_id = self.next_group_id;
        self.next_group_id += 1;
        self.group_extents.push(CellGroup::new(pos.into()));
        group_id
    }

    fn setup_tracking(&mut self, life: &Life) {
        for (x, y, cell) in life.iter() {
            if cell.is_alive() {
                self.groups[y as usize][x as usize] = self.calc_group_for_cell((x, y));
            }
        }
    }

    fn update(&mut self, life: &Life) {
        for (x, y, cell) in life.iter() {
            if cell.is_alive() {
                self.groups[y as usize][x as usize] = self.calc_group_for_cell((x, y));
            }
        }
    }

}

// pub fn identify(life: &Life) -> Vec<Pattern> {
//     // we need to identify which cells interact

//     Vec::new()
// }

#[cfg(test)]
mod identify_tests {
    use crate::life::from_plaintext;

    use super::*;

    #[test]
    fn test_cell_group_tracker() {
        let life = from_plaintext(
            "\
OO..OO
OO..OO",
            None,
            None,
        );
        let tracker = CellGroupTracker::new(&life);

        assert_eq!(tracker.groups[1][1], 1);
        assert_eq!(tracker.groups[1][4], 2);
    }

    #[test]
    fn test_cell_group_tracker_blinker() {
        let mut life = from_plaintext(
            "\
.O.
.O.
.O.",
            None,
            None,
        );
        let mut tracker = CellGroupTracker::new(&life);

        assert_eq!(tracker.groups[1][1], 1);

        life.update();
        tracker.update(&life);

        assert_eq!(tracker.groups[1][0], 1);
    }
}
