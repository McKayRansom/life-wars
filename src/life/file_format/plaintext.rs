use crate::{
    life::{Cell, Life, LifeOptions},
    pattern::{Pattern, PatternMetadata},
};

impl Life {
    // Based on Plaintext format https://conwaylife.com/wiki/Plaintext
    // TODO: TryFrom instead...
    pub fn from_plaintext(value: &str, options: LifeOptions) -> Life {
        let mut size: (u16, u16) = (0, 0);
        for line in value.lines() {
            if !line.starts_with("!") {
                size.1 += 1;
                size.0 = size.0.max(line.len() as u16);
            }
        }
        let mut life = Life::new_ex(size, options);

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

    pub fn to_plaintext(&self) -> String {
        let mut string = String::with_capacity(16);

        for (x, y, cell) in self.iter() {
            if x == 0 && y != 0 {
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
}

impl Pattern {
    pub fn from_plaintext(value: &str, options: LifeOptions) -> Pattern {
        let mut metadata = PatternMetadata::default();
        for line in value.lines() {
            if line.starts_with("!") {
                if let Some(pat_name) = line.strip_prefix("!Name: ") {
                    metadata.name = Some(pat_name.into())
                } else {
                    metadata.description = Some(line[1..].into())
                }
            } else {
                break;
            }
        }
        Pattern {
            life: Life::from_plaintext(value, options),
            metadata,
        }
    }

    pub fn to_plaintext(&self) -> String {
        let mut string = String::with_capacity(32);
        if let Some(name) = &self.metadata.name {
            string.push_str("!Name: ");
            string.push_str(name.as_str());
            string.push('\n');
        }

        if let Some(description) = &self.metadata.description {
            string.push('!');
            string.push_str(&description.as_str());
            string.push('\n');
        }

        string.push_str(self.life.to_plaintext().as_str());

        string
    }
}

#[cfg(test)]
mod life_tests {
    use super::*;

    pub const GLIDER_TXT: &str = "\
.O.
..O
OOO";

    #[test]
    fn test_txt_glider() {
        let life: Life = GLIDER_TXT.into();
        assert_eq!(format!("{life}"), GLIDER_TXT);
    }

    pub const STAR_WARS_TXT: &str = "\
CBO";
    #[test]
    fn test_txt_star_wars() {
        let life: Life = STAR_WARS_TXT.into();
        assert_eq!(format!("{life}"), STAR_WARS_TXT);
    }

    pub const FACTION_TXT: &str = "\
O12";

    #[test]
    fn test_txt_faction() {
        let life: Life = FACTION_TXT.into();
        assert_eq!(format!("{life}"), FACTION_TXT);
    }

    /*
    !Author: Richard K. Guy
    !The smallest, most common, and first discovered spaceship.
    !www.conwaylife.com/wiki/index.php?title=Glider
    */
    const COMMENTS_TXT: &str = "\
!Name: TestName
!Some info about it
O12";

    #[test]
    fn comments() {
        let pattern = Pattern::from_plaintext(COMMENTS_TXT, LifeOptions::default());

        assert_eq!(pattern.to_plaintext(), COMMENTS_TXT);
    }
}
