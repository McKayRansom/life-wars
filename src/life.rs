use std::fmt::Write;
use std::hash::Hash;

use macroquad::rand::RandomRange;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Cell {
    pub state: u8,
}

impl Cell {
    pub fn new(state: u8) -> Self {
        Self { state }
    }

    pub fn update(&self, neighbors: u8) -> Self {
        // Bri B2/S/3
        Self {
            // SWR B2/S345/4
            // state: if self.state == 0 {
            //     if neighbors == 2 { 1 } else { 0 }
            // } else if self.state == 1 {
            //     if neighbors >= 3 && neighbors <= 5 {
            //         1
            //     } else {
            //         2
            //     }
            // } else if self.state == 3 {
            //     0
            // } else {
            //     self.state + 1
            // },
            // GOL B3/S23
            state: if self.state > 0 {
                if neighbors < 2 {
                    0
                } else if neighbors < 4 {
                    1
                } else {
                    0
                }
            } else if neighbors == 3 {
                1
            } else {
                0
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Life {
    pub grid: Vec<Vec<Cell>>,
}

impl Life {
    pub fn new(dim: (usize, usize)) -> Self {
        let mut life = Self { grid: Vec::new() };

        for _x in 0..dim.0 {
            life.grid.push(
                (0..dim.1)
                    .map(|_i| Cell::new(RandomRange::gen_range(0, 2)))
                    .collect(),
            )
        }

        life
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<&Cell> {
        self.grid
            .get(pos.1)
            .map(|thing| thing.get(pos.0))
            .unwrap_or(None)
    }

    pub fn neighbors(&self, pos: (usize, usize)) -> u8 {
        // would a brute force be better TBH?
        // TODO: IMPLEMENT WRAPPING!
        (-1..2)
            .map(|y: i32| {
                self.grid
                    .get((pos.1 as i32 + y) as usize)
                    .map(|row| {
                        (-1..2)
                            .map(|x: i32| {
                                if x == 0 && y == 0 {
                                    0
                                } else {
                                    row.get((pos.0 as i32 + x) as usize)
                                        .map(|cell| if cell.state == 1 { 1 } else { 0 })
                                        .unwrap_or(0)
                                }
                            })
                            .sum()
                    })
                    .unwrap_or(0)
            })
            .sum()
    }

    pub fn update(&self) -> Self {
        Self {
            grid: self
                .grid
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, cell)| cell.update(self.neighbors((x, y))))
                        .collect()
                })
                .collect(),
        }
    }
}

// Should this be Display or Debug?
impl std::fmt::Display for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in &self.grid {
            for y in x {
                if y.state != 0 {
                    f.write_char('*')?;
                } else {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl From<&str> for Life {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|ch| Cell::new(if ch == ' ' { 0 } else { 1 }))
                        .collect()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
pub mod life_test {

    use super::*;

    #[test]
    fn life_test_basic() {
        let life: Life = " * 
 * 
 * "
        .into();

        assert_eq!(life.get((0, 0)).unwrap(), &Cell::new(0));
        assert_eq!(life.get((1, 0)).unwrap(), &Cell::new(1));
        assert_eq!(life.get((0, 1)).unwrap(), &Cell::new(0));

        assert_eq!(life.neighbors((0, 0)), 2);
        assert_eq!(life.neighbors((1, 0)), 1);
        assert_eq!(life.neighbors((0, 1)), 3);

        assert_eq!(life.update(), "   \n***\n   ".into());

        // as
    }
}
