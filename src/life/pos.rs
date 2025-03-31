use std::ops::{Add, Div, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}


#[inline(always)]
pub const fn pos(x: u16, y: u16) -> Pos {
    Pos::new(x, y)
}

impl Pos {
    #[inline(always)]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn reflect_y_odd(&self, y_line: u16) -> Self {
        Self {
            x: self.x,
            y: y_line + (y_line - self.y),
        }
    }

    pub fn reflect_y_even(&self, y_line: u16) -> Self {
        Self {
            x: self.x,
            y: y_line + (y_line - self.y) + 1,
        }
    }

    pub fn rotate_90_cw(&self, pivot: Self) -> Self {
        Self {
            x: pivot.x + (pivot.y - self.y),
            y: self.x,
        }
    }

    pub fn rotate_90_ccw(&self, pivot: Self) -> Self {
        Self {
            x: self.y,
            y: pivot.y + (pivot.x - self.x),
        }
    }

    pub fn rotate_180(&self, pivot: Self) -> Self {
        Self {
            x: pivot.x + (pivot.x - self.x),
            y: pivot.y + (pivot.y - self.y),
        }
    }

    pub fn iter(&self, area: Self) -> impl Iterator<Item = Self> {
        (0..area.y).flat_map(move |y: u16| {
            (0..area.x).map(move |x| (self.x + x, self.y + y).into())
        })
    }
}

impl From<(u16, u16)> for Pos {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Pos> for (u16, u16) {
    fn from(value: Pos) -> Self {
        (value.x, value.y)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


impl Div<u16> for Pos {
    type Output = Self;

    fn div(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}


#[cfg(test)]
mod pos_tests {
    use super::*;

    #[test]
    fn test_reflect() {
        assert_eq!(Pos::new(1, 2).reflect_y_odd(3), (1, 4).into());
        assert_eq!(Pos::new(1, 1).reflect_y_odd(3), (1, 5).into());
        assert_eq!(Pos::new(1, 3).reflect_y_odd(3), (1, 3).into());

        assert_eq!(Pos::new(1, 2).reflect_y_even(2), (1, 3).into());
        assert_eq!(Pos::new(1, 1).reflect_y_even(2), (1, 4).into());
    }

    #[test]
    fn test_rotate() {
        assert_eq!(Pos::new(1, 2).rotate_90_cw((3, 3).into()), (4, 1).into());
        assert_eq!(Pos::new(1, 2).rotate_90_ccw((3, 3).into()), (2, 5).into());
        assert_eq!(Pos::new(1, 2).rotate_180((3, 3).into()), (5, 4).into());
    }
}
