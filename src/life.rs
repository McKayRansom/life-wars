use std::fmt::Write;

use macroquad::rand::RandomRange;

pub mod iter;

pub trait Life {
    // fn new(size: (usize, usize)) -> Self;
    fn size(&self) -> (usize, usize);
    fn get(&self, pos: (usize, usize)) -> Option<&u8>;
    fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut u8>;

    // fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut u8)>;
    fn randomize(&mut self) {
        let size = self.size();
        for x in 0..size.0 {
            for y in 0..size.1 {
                *self.get_mut((x, y)).unwrap() = RandomRange::gen_range(0, 2);
            }
        }
    }
}

pub fn state_update(state: u8, neighbors: u8) -> u8 {
    // SWR B2/S345/4
    // if state == 0 {
    //     if neighbors == 2 { 1 } else { 0 }
    // } else if state == 1 {
    //     if neighbors >= 3 && neighbors <= 5 {
    //         1
    //     } else {
    //         2
    //     }
    // } else if state == 3 {
    //     0
    // } else {
    //     state + 1
    // }
    // GOL B3/S23
    if state > 0 {
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
    }
}

pub fn iter_life<'a>(life: &'a dyn Life) -> impl Iterator<Item = (usize, usize, &'a u8)> {
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
        for (x, _y, state) in iter_life(self) {
            if *state != 0 {
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
