use crate::life::{algo, Cell, Life, LifeAlgoSelect};

// Based on Plaintext format https://conwaylife.com/wiki/Plaintext
// TODO: TryFrom instead...
pub fn from_plaintext(value: &str, algo: Option<LifeAlgoSelect>) -> Life {
    let mut size: (usize, usize) = (0, 0);
    let mut name = String::new();
    for line in value.lines() {
        if line.starts_with("!") {
            if let Some(pat_name) = line.strip_prefix("!Name: ") {
                name.push_str(pat_name);
            }
        } else {
            size.1 += 1;
            size.0 = size.0.max(line.len());
        }
    }
    let mut life = Life {
        algo: algo::new(algo.unwrap_or_default(), size),
        name,
        ..Default::default()
    };
    let mut pos: (usize, usize) = (0, 0);
    for line in value.lines() {
        if line.starts_with("!") {
            continue;
        }

        for chr in line.chars() {
            match chr {
                '.' => {} // ignore, dead
                'O' => {
                    life.insert(pos, Cell::new(1, 0));
                }
                _ => {
                    unimplemented!("No parse rule in PlainText format for: '{chr}'");
                }
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
        if cell.is_alive() {
            string.push('O');
        } else {
            string.push('.');
        }
    }
    string
}
