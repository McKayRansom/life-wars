use std::str::Split;

use super::Cell;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct LifeRule {
    lut: [u32; 4],
}

impl LifeRule {
    // GOL B3/S23
    pub const GOL: Self = Self::new([0b1_00_00_00, 0b01_01_00_00, 0, 0]);
    // SWR B2/S345/4
    pub const STAR_WARS: Self = Self::new([
        0b1_00_00,
        0b10_10_10_01_01_01_10_10_10,
        0b11_11_11_11_11_11_11_11_11,
        0,
    ]);

    pub const fn new(lut: [u32; 4]) -> Self {
        Self { lut }
    }

    pub fn is_generations(&self) -> bool {
        self.lut[2] != 0
    }

    // BXX or X
    fn parse_rule_portion<F>(it: &mut Split<'_, char>, mut func: F)
    where
        F: FnMut(u32) -> (),
    {
        if let Some(portion) = it.next() {
            for chr in portion.chars() {
                if let Some(count) = chr.to_digit(10) {
                    func(count)
                }
            }
        }
    }

    // BXX/SXX or BXX/SXX/X
    // OR
    // SSS/BBB/GGG
    // https://conwaylife.com/wiki/Rulestring
    pub fn from_str(str: &str) -> Self {
        let mut new_rule: Self = Self::new([0, 0, 0, 0]);

        let mut portion_it = str.split('/');

        if str.starts_with('B') {
            Self::parse_rule_portion(&mut portion_it, |count| new_rule.lut[0] |= 1 << (count * 2));
            Self::parse_rule_portion(&mut portion_it, |count| new_rule.lut[1] |= 1 << (count * 2));
        } else {
            Self::parse_rule_portion(&mut portion_it, |count| new_rule.lut[1] |= 1 << (count * 2));
            Self::parse_rule_portion(&mut portion_it, |count| new_rule.lut[0] |= 1 << (count * 2));
        }
        Self::parse_rule_portion(&mut portion_it, |count| {
            match count {
                3 => {} // do nothing
                4 => {
                    // Transition to state 2 instead of state 0
                    for i in 0..9 {
                        if new_rule.lut[1] & 1 << (i * 2) == 0 {
                            new_rule.lut[1] |= 2 << (i * 2);
                        }
                    }
                    new_rule.lut[2] = 0b11_11_11_11_11_11_11_11_11;
                }
                _ => unimplemented!(),
            }
        });

        new_rule
    }

    pub fn to_str(&self) -> String {
        let mut births: u32 = 0;
        let mut survives: u32 = 0;
        for i in 1..9 {
            if (self.lut[0] & (1 << i * 2)) != 0 {
                births = births * 10 | i;
            }
            if (self.lut[1] & (1 << i * 2)) != 0 {
                survives = (survives * 10) + i;
            }
        }
        if self.is_generations() {
            format!("B{births}/S{survives}/4")
        } else {
            format!("B{births}/S{survives}")
        }
    }

    pub fn update(&self, state: u8, (neighbors, faction): (u8, u8)) -> Cell {
        Cell::new(Self::state_update_f(self, state, neighbors), faction)
    }

    fn state_update_f(&self, state: u8, neighbors: u8) -> u8 {
        ((self.lut[state as usize] & (3 << (neighbors as u32 * 2))) >> (neighbors as u32 * 2)) as u8
    }
}

#[cfg(test)]
mod rule_tests {
    use super::*;

    #[test]
    fn test_rule_from_str() {
        assert_eq!(LifeRule::from_str("B3/S23"), LifeRule::GOL);
        assert_eq!(LifeRule::from_str("B2/S345/4"), LifeRule::STAR_WARS);
    }

    #[test]
    fn test_rule_to_str() {
        assert_eq!(LifeRule::GOL.to_str(), "B3/S23");
        assert_eq!(LifeRule::STAR_WARS.to_str(), "B2/S345/4");
    }

    #[test]
    fn test_rule_alt_fmt() {
        assert_eq!(LifeRule::from_str("23/3"), LifeRule::GOL);
        assert_eq!(LifeRule::from_str("345/2/4"), LifeRule::STAR_WARS);
    }

    #[test]
    fn test_rule_life() {
        // GOL B3/S23
        let rule = LifeRule::GOL;

        // Birth
        assert_eq!(rule.update(0, (2, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(0, (3, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(0, (4, 0)), Cell::new(0, 0));

        // Survive
        assert_eq!(rule.update(1, (1, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(1, (2, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(1, (3, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(1, (4, 0)), Cell::new(0, 0));
    }

    #[test]
    fn test_rule_sw() {
        // SW B2/S345/4
        let rule = LifeRule::STAR_WARS;

        // Birth
        assert_eq!(rule.update(0, (1, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(0, (2, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(0, (3, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(0, (4, 0)), Cell::new(0, 0));

        // Survive
        assert_eq!(rule.update(1, (3, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(1, (4, 0)), Cell::new(1, 0));
        assert_eq!(rule.update(1, (5, 0)), Cell::new(1, 0));

        // Refractory
        assert_eq!(rule.update(1, (1, 0)), Cell::new(2, 0));
        assert_eq!(rule.update(1, (2, 0)), Cell::new(2, 0));
        assert_eq!(rule.update(1, (6, 0)), Cell::new(2, 0));

        // Refractory 2
        assert_eq!(rule.update(2, (1, 0)), Cell::new(3, 0));
        assert_eq!(rule.update(2, (3, 0)), Cell::new(3, 0));
        assert_eq!(rule.update(2, (6, 0)), Cell::new(3, 0));

        // Refractory 3
        assert_eq!(rule.update(3, (1, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(3, (3, 0)), Cell::new(0, 0));
        assert_eq!(rule.update(3, (6, 0)), Cell::new(0, 0));
    }
}
