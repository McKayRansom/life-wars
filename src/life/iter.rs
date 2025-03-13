use super::{state_update, Cell, Life};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct LifeIter {
    pub grid: Vec<Vec<Cell>>,
}

impl Life for LifeIter {
    fn size(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn get(&self, pos: (usize, usize)) -> Option<&Cell> {
        self.grid
            .get(pos.1)
            .map(|thing| thing.get(pos.0))
            .unwrap_or(None)
    }

    fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut Cell> {
        self.grid
            .get_mut(pos.1)
            .map(|thing| thing.get_mut(pos.0))
            .unwrap_or(None)
    }
}

impl LifeIter {
    pub fn new(dim: (usize, usize)) -> Self {
        Self {
            grid: vec![vec![Cell::new(0, 0); dim.0]; dim.1],
        }
    }

    // This brute force was bench_256 at 341uS  instead of 289 for iter version...
    // pub fn neighbors(&self, pos: (usize, usize)) -> u8 {
    //     let mut neighbors: u8 = 0;
    //     if pos.0 > 0 {
    //         if pos.1 > 0 {
    //             neighbors += self.get((pos.0 - 1, pos.1 - 1)).unwrap_or(&0);
    //         }
    //         neighbors += self.get((pos.0 - 1, pos.1 + 1)).unwrap_or(&0);
    //         neighbors += self.get((pos.0 - 1, pos.1)).unwrap_or(&0);
    //     }
    //     if pos.1 > 0 {
    //         neighbors += self.get((pos.0, pos.1 - 1)).unwrap_or(&0);
    //         neighbors += self.get((pos.0 + 1, pos.1 - 1)).unwrap_or(&0);
    //     }

    //     neighbors += self.get((pos.0 + 1, pos.1)).unwrap_or(&0);
    //     neighbors += self.get((pos.0 + 1, pos.1 + 1)).unwrap_or(&0);
    //     neighbors += self.get((pos.0, pos.1 + 1)).unwrap_or(&0);

    //     neighbors
    // }

    // This seemingly stupid iterator version is somehow faster?
    pub fn neighbors(&self, faction: u8, pos: (usize, usize)) -> (u8, u8) {
        let mut faction: u8 = faction;
        let mut sum: u8 = 0;
        for dy in -1..2 {
            if let Some(row) = self.grid.get((pos.1 as i32 + dy) as usize) {
                for dx in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if let Some(cell) = row.get((pos.0 as i32 + dx) as usize) {
                        if cell.is_alive() {
                            if cell.get_faction() == faction {
                                sum += 1;
                            } else if sum > 0 {
                                sum -= 1;
                            } else {
                                faction = cell.get_faction();
                                sum += 1;
                            }
                        } 
                    }
                }
            }
        }
        (sum, faction)
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
                        .map(|(x, cell)| state_update(cell.get_state(), self.neighbors(cell.get_faction(), (x, y))))
                        .collect()
                })
                .collect(),
        }
    }
}

impl From<&str> for LifeIter {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|ch| Cell::new(if ch == ' ' { 0 } else { 1 }, 0))
                        .collect()
                })
                .collect(),
        }
    }
}
