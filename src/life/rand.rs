use quad_rand::RandGenerator;

use super::{Cell, Life, Pos};

// https://conwaylife.com/wiki/Static_symmetry

#[derive(Default)]
pub enum RandSymmetry {
    #[default]
    C1, // Symmetric under 360 A.K.A. not symmetric
    C2_1, // rotation around center (odd x odd)
    C2_2, // rotation around midpoint (even x odd)
    C2_4, // rotation around corner (even x even)
    C4_1, // rotation around center (odd x odd)
    C4_4, // rotation around corner (even x even)
    D2_1, // reflectional symmetry the line bisects a row(odd by any)
    D2_2, // The line is between a row of cells
          // D2_x, // The line is diagonal
}

fn rand_cell(rand: &RandGenerator) -> Cell {
    Cell::new(if rand.rand() < u32::MAX / 3 { 1 } else { 0 }, 0)
}

pub fn rand_life(
    life: &mut Life,
    pos: Pos,
    area: Pos,
    seed: u64,
    symmetry: Option<RandSymmetry>,
) {
    let pos: Pos = pos.into();
    let area: Pos = area.into();
    let this = &mut *life;
    let rand = RandGenerator::new();
    rand.srand(seed);

    let symmetry = symmetry.unwrap_or_default();

    match symmetry {
        RandSymmetry::C1 => {
            for pos in pos.iter(area) {
                let cell = rand_cell(&rand);
                this.insert(pos.into(), cell);
            }
            // for y in 0..area.y {
            // for x in 0..area.x {
            // this.insert((pos.x + x, pos.y + area.1 - 1 - y), cell);
            // }
            // }
        }
        RandSymmetry::C2_1 => {
            let pivot = pos + area/2;
            for new_pos in pos.iter(Pos::new(area.x / 2, area.y / 2 )) {
                let cell = rand_cell(&rand);
                this.insert(new_pos.into(), cell);
                this.insert(new_pos.rotate_180(pivot).into(), cell);
            }
        }
        RandSymmetry::C2_2 => todo!(),
        RandSymmetry::C2_4 => todo!(),
        RandSymmetry::C4_1 => {
            let pivot = pos + area/2;
            for new_pos in pos.iter(Pos::new(area.x / 2 + 1, area.y / 2 + 1)) {
                let cell = rand_cell(&rand);
                this.insert(new_pos.into(), cell);
                this.insert(new_pos.rotate_90_cw(pivot).into(), cell);
                this.insert(new_pos.rotate_180(pivot).into(), cell);
                this.insert(new_pos.rotate_90_ccw(pivot).into(), cell);
            }
        }
        RandSymmetry::C4_4 => todo!(),
        RandSymmetry::D2_1 => {
            let y_line = area.y / 2 + 2;
            for new_pos in pos.iter(Pos::new(area.x, area.y / 2 + area.y % 2)) {
                let cell = rand_cell(&rand);
                this.insert(new_pos.into(), cell);
                this.insert(new_pos.reflect_y_odd(y_line).into(), cell);
            }
        }
        RandSymmetry::D2_2 => {
            let y_line = area.y / 2 + 1;
            for new_pos in pos.iter(Pos::new(area.x, area.y / 2)) {
                let cell = rand_cell(&rand);
                this.insert(new_pos.into(), cell);
                this.insert(new_pos.reflect_y_even(y_line).into(), cell);
            }
        }
    }
}

#[cfg(test)]
mod rand_life_tests {
    use super::*;

    #[test]
    fn test_rand() {
        let mut life = Life::default(); // 8x8
        rand_life(&mut life, (2, 2).into(), (4, 4).into(), 1234, None);

        assert_eq!(
            life.to_plaintext(),
            "\
........
........
...OO...
..O.....
....O...
..OOO...
........
........",
            "{life}"
        );
    }

    #[test]
    fn test_rand_d2_1() {
        let mut life = Life::default(); // 8x8
        rand_life(&mut life, (2, 2).into(), (5, 5).into(), 1234, Some(RandSymmetry::D2_1));

        assert_eq!(
            life.to_plaintext(),
            "\
........
........
...OO.O.
........
..O.OOO.
........
...OO.O.
........",
            "{life}"
        );
    }

    #[test]
    fn test_rand_d2_2() {
        let mut life = Life::default(); // 8x8
        rand_life(&mut life, (2, 2).into(), (4, 4).into(), 1234, Some(RandSymmetry::D2_2));

        assert_eq!(
            life.to_plaintext(),
            "\
........
........
...OO...
..O.....
..O.....
...OO...
........
........",
            "{life}"
        );
    }

    #[test]
    fn test_rand_c2_1() {
        let mut life = Life::default(); // 8x8
        rand_life(&mut life, (2, 2).into(), (5, 5).into(), 1234, Some(RandSymmetry::C2_1));

        assert_eq!(
            life.to_plaintext(),
            "\
........
........
...O....
..O.....
........
......O.
.....O..
........",
            "{life}"
        );
    }

    #[test]
    fn test_rand_c4_1() {
        let mut life = Life::default(); // 8x8
        rand_life(&mut life, (2, 2).into(), (5, 5).into(), 1234, Some(RandSymmetry::C4_1));

        assert_eq!(
            life.to_plaintext(),
            "\
........
........
...O....
...O.OO.
........
..OO.O..
.....O..
........",
            "{life}"
        );
    }
}
