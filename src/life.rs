use std::fmt::Write;

pub mod iter;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Cell {
    value: u8,
}

impl Cell {
    const STATE_MASK: u8 = 0xF;
    const FACTION_MASK: u8 = 0xF0;

    pub fn new(state: u8, faction: u8) -> Self {
        Self { value: state | (faction << 4)}
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

pub trait Life {
    // fn new(size: (usize, usize)) -> Self;
    fn size(&self) -> (usize, usize);
    fn get(&self, pos: (usize, usize)) -> Option<&Cell>;
    fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut Cell>;

    // fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut u8)>;
    fn randomize(&mut self) {
        let size = self.size();
        for x in 0..size.0 {
            for y in 0..size.1 {
                *self.get_mut((x, y)).unwrap() = Cell::new(
                    if macroquad::rand::rand() < u32::MAX / 5 {
                        1
                    } else {
                        0
                    },
                    if y < size.1 / 2 {
                        1
                    } else {
                        0
                    }
                );
            }
        }
    }
}

pub fn state_update_f(state: u8, neighbors: u8) -> u8 {
    // SWR B2/S345/4
    if state == 0 {
        if neighbors == 2 { 1 } else { 0 }
    } else if state == 1 {
        if neighbors >= 3 && neighbors <= 5 {
            1
        } else {
            2
        }
    } else if state == 3 {
        0
    } else {
        state + 1
    }
    // GOL B3/S23
    // if state > 0 {
    //     if neighbors >= 2 && neighbors <= 3 {
    //         1
    //     } else {
    //         0
    //     }
    // } else if neighbors == 3 {
    //     1
    // } else {
    //     0
    // }
    // Fake Coral that I like
}

pub fn state_update(state: u8, (neighbors, faction): (u8, u8)) -> Cell {
    Cell::new(state_update_f(state, neighbors), faction)
}

pub fn iter_life<'a>(life: &'a dyn Life) -> impl Iterator<Item = (usize, usize, &'a Cell)> {
    let size = life.size();
    (0..size.1)
        .flat_map(move |y: usize| (0..size.1).map(move |x| (x, y, life.get((x, y)).unwrap())))
}

// pub fn iter_life_mut<'a>(life: &'a mut dyn Life) -> impl Iterator<Item = (usize, usize, &'a mut u8)> {
//     let size = life.size();
//     (0..size.1).flat_map(move |y: usize| (0..size.1).map(move |x| (x, y, life.get_mut((x, y)).unwrap())))
// }

// Should this be Display or Debug?
impl std::fmt::Display for dyn Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, _y, cell) in iter_life(self) {
            if cell.is_alive() {
                f.write_char('*')?;
            } else {
                f.write_char(' ')?;
            }
            if x == 0 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}
