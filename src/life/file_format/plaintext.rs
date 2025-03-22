use crate::life::{algo, Cell, Life, LifeAlgoSelect, LifeRule};

// Based on Plaintext format https://conwaylife.com/wiki/Plaintext
// TODO: TryFrom instead...
pub fn from_plaintext(value: &str, algo: Option<LifeAlgoSelect>, rule: Option<LifeRule>) -> Life {
    let mut size: (u16, u16) = (0, 0);
    let mut name = String::new();
    for line in value.lines() {
        if line.starts_with("!") {
            if let Some(pat_name) = line.strip_prefix("!Name: ") {
                name.push_str(pat_name);
            }
        } else {
            size.1 += 1;
            size.0 = size.0.max(line.len() as u16);
        }
    }
    let mut life = Life {
        algo: algo::new(algo.unwrap_or_default(), size),
        name,
        rule: rule.unwrap_or_default(),
        ..Default::default()
    };
    let mut pos: (u16, u16) = (0, 0);
    for line in value.lines() {
        if line.starts_with("!") {
            continue;
        }

        for chr in line.chars() {
            if let Some(cell) = match chr {
                '.' => None, // ignore, dead
                'O' => Some(Cell::new(1, 0)),
                'B' => Some(Cell::new(2, 0)),
                'C' => Some(Cell::new(3, 0)),
                '1' => Some(Cell::new(1, 1)),
                '2' => Some(Cell::new(1, 2)),
                ' ' => continue,
                _ => unimplemented!("No parse rule in PlainText format for: '{chr}'"),
            } {
                life.insert(pos, cell);
            }
            pos.0 += 1;
        }

        pos.0 = 0;
        pos.1 += 1;
    }
    life
}

pub fn life_to_plaintext(life: &Life) -> String {
    let mut string = String::with_capacity(16);
    string.push_str("!Name: ");
    string.push_str(life.name.as_str());
    for (x, _y, cell) in life.iter() {
        if x == 0 {
            string.push('\n');
        }
        string.push(match (cell.get_state(), cell.get_faction()) {
            (0, _) => '.',
            (1, 0) => 'O',
            (2, 0) => 'B',
            (3, 0) => 'C',
            (1, 1) => '1',
            (1, 2) => '2',
            _ => unimplemented!("No serialize rule for cell {cell:?}"),
        })
    }
    string
}

#[cfg(test)]
mod life_tests {
    use super::*;

    // TODO: Descriptions??
    /*
    !Author: Richard K. Guy
    !The smallest, most common, and first discovered spaceship.
    !www.conwaylife.com/wiki/index.php?title=Glider
    */
    pub const GLIDER_TXT: &str = "\
!Name: Glider
.O.
..O
OOO";

    #[test]
    fn test_txt_glider() {
        let life: Life = GLIDER_TXT.into();
        assert_eq!(format!("{life}"), GLIDER_TXT);
    }

    pub const STAR_WARS_TXT: &str = "\
!Name: Photon
CBO";
    #[test]
    fn test_txt_star_wars() {
        let life: Life = STAR_WARS_TXT.into();
        assert_eq!(format!("{life}"), STAR_WARS_TXT);
    }

    pub const FACTION_TXT: &str = "\
!Name: Faction
O12";

    #[test]
    fn test_txt_faction() {
        let life: Life = FACTION_TXT.into();
        assert_eq!(format!("{life}"), FACTION_TXT);
    }
}
