use std::ops::{Add, Div, Sub};

use macroquad::math::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

#[inline(always)]
pub const fn pos(x: i16, y: i16) -> Pos {
    Pos::new(x, y)
}

impl Pos {
    #[inline(always)]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn reflect_y_odd(&self, y_line: i16) -> Self {
        Self {
            x: self.x,
            y: y_line + (y_line - self.y),
        }
    }

    pub fn reflect_y_even(&self, y_line: i16) -> Self {
        Self {
            x: self.x,
            y: y_line + (y_line - self.y) + 1,
        }
    }

    pub fn rotate_90_cw_odd(&self, pivot: Self) -> Self {
        Self {
            x: pivot.x + (pivot.y - self.y),
            y: self.x,
        }
    }

    pub fn rotate_90_cw_even(&self, pivot: Self) -> Self {
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
        (0..area.y)
            .flat_map(move |y: i16| (0..area.x).map(move |x| (self.x + x, self.y + y).into()))
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn try_from_vec2(vec2: Vec2, max: Pos) -> Option<Self> {
        if vec2.x < 0. || vec2.y < 0. {
            return None;
        }
        let pos = Self::new(vec2.x as i16, vec2.y as i16);
        if pos.x > max.x || pos.y > max.y {
            None
        } else {
            Some(pos)
        }
    }

    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl From<(i16, i16)> for Pos {
    fn from(value: (i16, i16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Pos> for (i16, i16) {
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

impl Div<i16> for Pos {
    type Output = Self;

    fn div(self, rhs: i16) -> Self::Output {
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
        assert_eq!(Pos::new(1, 2).rotate_90_cw_odd((3, 3).into()), (4, 1).into());
        assert_eq!(Pos::new(1, 2).rotate_90_ccw((3, 3).into()), (2, 5).into());
        assert_eq!(Pos::new(1, 2).rotate_180((3, 3).into()), (5, 4).into());
    }
}
