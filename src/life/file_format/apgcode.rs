use std::str::Chars;

use crate::{
    life::{Cell, Life, LifeRule},
    pattern::{Classification, Pattern},
};

/*
 * APGCODE file format
 * https://conwaylife.com/wiki/Apgcode
 * Pros:
 * - Classification and period
 * - Attempt at connonical form to avoid duplicates
 * 
 * Cons:
 * - No size complicates implementation
 * - No name or description (on purpose I suppose)
 * - Seems complicated
 * 
 */


const CLASSIFICATION_LOOKUP: &[(Classification, &'static str)] = &[
    (Classification::StilLife, "xs"),
    (Classification::Oscillator, "xp"),
    (Classification::Spaceship, "xq"),
    (Classification::LinearGrowth, "yl"),
    (Classification::Methuselah, "methuselah"),
    (Classification::Messless, "messless"),
    (Classification::Megasized, "megasized"),
    (Classification::Explosive, "zz_EXPLOSIVE"),
    (Classification::Linear, "zz_LINEAR"),
    (Classification::Quadratic, "zz_QUADRATIZ"),
    (Classification::Replicator, "zz_REPLICATOR"),
    (Classification::Pathological, "PATHOLOGICAL"),
];

fn classification_from_prefix(prefix: &str) -> (Option<Classification>, Option<u32>) {
    for lookup in CLASSIFICATION_LOOKUP {
        if prefix.starts_with(lookup.1) {
            let period: Option<u32> = if prefix.len() > lookup.1.len() {
                Some(prefix[lookup.1.len()..].parse().unwrap_or_default())
            } else {
                None
            };
            return (Some(lookup.0), period);
        }
    }
    (None, None)
}

fn prefix_from_classification(classification: Option<Classification>) -> &'static str {
    if let Some(classification) = classification {
        for lookup in CLASSIFICATION_LOOKUP {
            if lookup.0 == classification {
                return lookup.1;
            }
        }
    }
    ""
}

fn zero_count_read(chr: char, iter: &mut Chars<'_>) -> Option<usize> {
    match chr {
        '0' => Some(1),
        'w' => Some(2),
        'x' => Some(3),
        'y' => {
            let count = (iter.next().unwrap() as usize - '0' as usize) + 4;
            Some(count)
        }
        _ => None,
    }
}


fn zero_count_write(string: &mut String, count: usize) {
    match count {
        0 => {}
        1 => string.push('0'),
        2 => string.push('w'),
        3 => string.push('x'),
        _ if count <= 39 => {
            string.push('y');
            string.push(char::from_digit(count as u32 - 4, 26).expect("Exceeded max zero count!"));
        }
        _ => panic!("No supported way to write {count} zeros!"),
    }
}

#[allow(unused)]
pub fn new_pattern_from_apgcode(apgcode: &str, rule: Option<LifeRule>) -> Pattern {
    let rule = rule.unwrap_or_default();

    let (prefix, suffix) = apgcode.split_once('_').unwrap();

    let (classification, period) = classification_from_prefix(prefix);

    let mut row_of_5_count: usize = 0;
    let row_size = suffix
        .split('z')
        .map(|section| {
            let mut iter = section.chars();

            let mut row_len = 0;
            while let Some(chr) = iter.next() {
                row_len += zero_count_read(chr, &mut iter).unwrap_or(1);
            }
            row_of_5_count += 1;
            row_len
        })
        .max()
        .unwrap();

    let mut life: Life = Life::new_rule(
        crate::life::LifeAlgoSelect::Basic,
        (row_size as u16, (row_of_5_count * 5) as u16),
        rule,
    );

    for (row_of_5_count, row) in suffix.split('z').enumerate() {
        let mut x = 0;
        let mut iter = row.chars();
        while let Some(chr) = iter.next() {
            if let Some(zero_count) = zero_count_read(chr, &mut iter) {
                x += zero_count;
                continue;
            }

            let mut col_vals = if chr >= '0' && chr <= '9' {
                chr as u16 - '0' as u16
            } else if chr >= 'a' && chr <= 'v' {
                (chr as u16 - 'a' as u16) + 10
            } else {
                panic!("Unexpected char: '{chr}' in apgcode")
            };
            for y in 0..6 {
                if col_vals & 1 != 0 {
                    life.insert((x as u16, (row_of_5_count * 5) as u16 + y), Cell::new(1, 0));
                }
                col_vals >>= 1;
            }
            x += 1;
        }
        // break;
    }
    Pattern {
        life,
        classification,
        period,
        ..Default::default()
    }
}


#[allow(unused)]
pub fn apgcode_from_pattern(pattern: &Pattern) -> String {
    let mut string = String::with_capacity(64);

    string.push_str(prefix_from_classification(pattern.classification));

    if let Some(period) = pattern.period {
        string.push_str(format!("{period}").as_str());
    }

    string.push('_');

    // iterate by cols instead of by rows
    let size = pattern.life.size();
    for row_of_5 in 0..(size.1 / 5 + if size.1 % 5 == 0 { 0 } else { 1 }) {
        let mut zero_count: usize = 0;
        for x in 0..size.0 {
            let mut col_vals = 0;
            for dy in 0..5 {
                col_vals >>= 1;
                if let Some(cell) = pattern.life.get_cell((x, (row_of_5 * 5) + dy)) {
                    if cell.is_alive() {
                        col_vals |= 1 << 4;
                    }
                }
            }
            let col_char = char::from_digit(col_vals as u32, 32).unwrap();
            if col_char != '0' {
                zero_count_write(&mut string, zero_count);
                zero_count = 0;
                string.push(col_char);
            } else {
                zero_count += 1;
            }
        }
        string.push('z');
    }

    // Remove last 'z'
    string.pop();

    string
}

#[cfg(test)]
mod apgcode_tests {
    use crate::{life::life_to_plaintext, pattern::Pattern};

    use super::*;

    /*

    !Name: HWSS
    !Author: John Conway

    !The fourth most common spaceship (after the glider, lightweight spaceship and middleweight spaceship).
    !www.conwaylife.com/wiki/index.php?title=Heavyweight_spaceship
         */
    /*
            !Name: HWSS
    !Author: John Conway
    !The fourth most common spaceship (after the glider, lightweight spaceship and middleweight spaceship).
    !www.conwaylife.com/wiki/index.php?title=Heavyweight_spaceship
    ...OO
    .O....O
    O
    O.....O
    OOOOOO */

    const HEAVYWEIGHT_SPACESHIP_TXT: &str = "!Name: 
.OO....
OO.OOOO
.OOOOOO
..OOOO.
.......";

    #[test]
    fn test_apgcode_spaceship() {
        const HEAVYWEIGHT_SPACESHIP_APG: &str = "xq4_27deee6";

        let pattern: Pattern = new_pattern_from_apgcode(HEAVYWEIGHT_SPACESHIP_APG, None);
        assert_eq!(
            life_to_plaintext(&pattern.life),
            HEAVYWEIGHT_SPACESHIP_TXT,
            "{}",
            pattern.life
        );

        assert_eq!(
            apgcode_from_pattern(&pattern).as_str(),
            HEAVYWEIGHT_SPACESHIP_APG
        );
    }

    #[test]
    fn test_apgcode_still_life() {
        const STIL_LIFE_APG: &str = "xs31_0ca178b96z69d1d96";
        let pattern: Pattern = new_pattern_from_apgcode(STIL_LIFE_APG, None);
        assert_eq!(
            apgcode_from_pattern(&pattern).as_str(),
            STIL_LIFE_APG,
            "{}",
            pattern.life
        );
    }

    #[test]
    fn test_apgcode_queen_bee_shuttle() {
        const QUEEN_BEE_SHUTTLE: &str = "xp30_w33z8kqrqk8zzzx33";

        let pattern: Pattern = new_pattern_from_apgcode(QUEEN_BEE_SHUTTLE, None);
        assert_eq!(
            apgcode_from_pattern(&pattern).as_str(),
            QUEEN_BEE_SHUTTLE,
            "{}",
            pattern.life
        );
    }

    #[test]
    fn test_apgcode_quadpole_tie_ship() {
        const QUADPOLE_TIE_SHIP: &str = "xp2_31a08zy0123cko";

        let pattern: Pattern = new_pattern_from_apgcode(QUADPOLE_TIE_SHIP, None);
        assert_eq!(
            apgcode_from_pattern(&pattern).as_str(),
            QUADPOLE_TIE_SHIP,
            "{}",
            pattern.life
        );
    }
}
