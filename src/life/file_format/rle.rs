use std::str::{FromStr, Split};

/*
 *        let rle_glider = "
 *        #C This is a glider.
 *        x = 3, y = 3
 *        bo$2bo$3o!";
 * https://conwaylife.com/wiki/Run_Length_Encoded
 */

use crate::{
    life::{Cell, Life, LifeRule, Pos},
    pattern::{Pattern, PatternMetadata},
};

fn rle_parse_header(it: &mut Split<'_, char>) -> Option<Pattern> {
    let mut metadata = PatternMetadata::default();
    for line in it.by_ref() {
        // parse headers
        if line.starts_with("#") {
            if let Some(name) = line.strip_prefix("#N ") {
                metadata.name = Some(name.into());
            } else if let Some(comment) = line.strip_prefix("#C ") {
                metadata.description = Some(comment.into())
            } else {
                panic!("Unkown header comment: {line}")
            }
        } else if line.starts_with("x") {
            // header
            let mut size= Pos::new(16, 16);
            let mut rule: LifeRule = LifeRule::GOL;
            for field in line.split(", ") {
                let (name, value) = field.split_once(" = ").expect("Failed to parse field");
                match name {
                    "x" => size.x = value.parse().expect("Failed to parse field"),
                    "y" => size.y = value.parse().expect("Failed to parse field"),
                    "rule" => rule = LifeRule::from_str(value).unwrap(),
                    _ => panic!("Unkown header field: {}", name),
                }
            }
            return Some(Pattern {
                life: Life::new_rule(size, rule),
                metadata,
            });
        } else {
            panic!("Unkown line: {}", line);
        }
    }
    None
}

fn rle_parse_body(it: &mut Split<'_, char>, life: &mut Life) {
    let mut pos = Pos::new(0, 0);
    for line in it {
        let mut run_count = 0;
        for chr in line.chars() {
            if let Some(count) = chr.to_digit(10) {
                run_count = (run_count * 10) + count;
            } else {
                if run_count == 0 {
                    run_count = 1;
                }
                match chr {
                    // Default Life tags
                    'b' => pos.x += run_count as i16,
                    '.' => pos.x += run_count as i16,
                    'o' => {
                        for _ in 0..run_count {
                            life.insert(pos, Cell::new(1, 0));
                            pos.x += 1;
                        }
                    }
                    // Generations tags
                    'A' => {
                        for _ in 0..run_count {
                            life.insert(pos, Cell::new(1, 0));
                            pos.x += 1;
                        }
                    }
                    'B' => {
                        for _ in 0..run_count {
                            life.insert(pos, Cell::new(2, 0));
                            pos.x += 1;
                        }
                    }
                    'C' => {
                        for _ in 0..run_count {
                            life.insert(pos, Cell::new(3, 0));
                            pos.x += 1;
                        }
                    }

                    '$' => {
                        pos.y += run_count as i16;
                        pos.x = 0;
                    }
                    '!' => break,

                    _ => panic!("Unkown <tag> '{chr}'"),
                }
                run_count = 0;
            }
        }
    }
}

fn rle_tag(state: u8) -> char {
    match state {
        0 => 'b',
        1 => 'o',
        _ => todo!(),
    }
}

fn rle_tag_generations(state: u8) -> char {
    match state {
        0 => '.',
        1 => 'A',
        2 => 'B',
        3 => 'C',
        _ => todo!(),
    }
}

struct RleItem {
    count: i32,
    tag: char,
}

impl RleItem {
    fn write(&self, string: &mut String, line_count: &mut usize) {
        if self.count > 1 {
            let count_str = self.count.to_string();
            if *line_count + count_str.len() + 2 >= 71 {
                string.push('\n');
                *line_count = 0;
            }
            *line_count += count_str.len();
            string.push_str(count_str.as_str());
        } else {
            // No idea why this is off-by-one, it has to be to match which LifeWiki which is super cursed
            if *line_count + 1 >= 70 {
                string.push('\n');
                *line_count = 0;
            }
        }
        string.push(self.tag);
        *line_count += 1;
    }
}

struct RleWriter {
    item: Option<RleItem>,
    line_count: usize,
    string: String,
    dead: RleItem,
}

impl RleWriter {
    fn push(&mut self, tag: char) {
        if let Some(item) = &mut self.item {
            if item.tag != tag {
                if item.tag == '$' && tag == self.dead.tag {
                    self.dead.count += 1;
                } else {
                    if tag != '$' || item.tag != self.dead.tag {
                        item.write(&mut self.string, &mut self.line_count);
                    }
                    self.item = Some(RleItem { tag, count: 1 });
                    if self.dead.count > 0 {
                        self.dead.write(&mut self.string, &mut self.line_count);
                        self.dead.count = 0;
                    }
                }
            } else {
                if tag == '$' {
                    self.dead.count = 0;
                }
                item.count += 1;
            }
        } else {
            self.item = Some(RleItem { tag, count: 1 });
        }
    }

    fn flush(&mut self) {
        if let Some(item) = &self.item {
            if item.tag != self.dead.tag {
                item.write(&mut self.string, &mut self.line_count);
            }
        }
    }
}

impl Pattern {
    pub fn from_rle(rle: &str) -> Self {
        let mut line_it = rle.split('\n');
        let mut pattern =
            rle_parse_header(&mut line_it).expect("Failed to parse header from .rle!");
        rle_parse_body(&mut line_it, &mut pattern.life);
        pattern
    }

    pub fn to_rle(&self) -> String {
        let size = self.life.size();
        let mut string = String::with_capacity(64);
        if let Some(name) = &self.metadata.name {
            string.push_str("#N ");
            string.push_str(name.as_str());
            string.push('\n');
        }
        if let Some(desc) = &self.metadata.description {
            string.push_str("#C ");
            string.push_str(desc.as_str());
            string.push('\n');
        }
        string.push_str(
            format!(
                "x = {}, y = {}, rule = {}\n",
                size.x,
                size.y,
                self.life.rule.to_str().as_str()
            )
            .as_str(),
        );

        let tag_func: fn(u8) -> char = if self.life.rule.is_generations() {
            rle_tag_generations
        } else {
            rle_tag
        };

        let mut writer = RleWriter {
            item: None,
            line_count: 0,
            string,
            dead: RleItem {
                tag: tag_func(0),
                count: 0,
            },
        };

        for (x, y, cell) in self.life.iter() {
            if x == 0 && y != 0 {
                writer.push('$');
            }
            writer.push(tag_func(cell.get_state()));
        }
        writer.flush();
        writer.string.push('!');

        writer.string
    }
}

#[cfg(test)]
mod rle_tests {
    use super::*;

    const GLIDER_RLE: &str = "\
#C This is a glider.
x = 3, y = 3, rule = B3/S23
bo$2bo$3o!";

    #[test]
    fn test_rle_glider() {
        let pattern = Pattern::from_rle(GLIDER_RLE);

        assert_eq!(pattern.life.size(), (3, 3).into());
        assert_eq!(pattern.to_rle(), GLIDER_RLE);
    }

    const GOSPER_RLE: &str = "\
#N Gosper glider gun
x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o
4bobo$10bo5bo7bo$11bo3bo$12b2o!";

    #[test]
    fn test_rle_gosper() {
        let pattern = Pattern::from_rle(GOSPER_RLE);
        assert_eq!(pattern.life.rule, LifeRule::GOL);
        assert_eq!(pattern.life.size(), (36, 9).into());
        assert_eq!(pattern.life.algo.get((24, 0).into()).unwrap(), &Cell::new(1, 0));
        assert_eq!(pattern.to_rle(), GOSPER_RLE);
    }

    const STAR_WARS_RLE: &str = "\
x = 43, y = 48, rule = B2/S345/4
2.ABC$2.A2.A$.6A.A$2.A3.2A.B$A.A4.2A.C$B2A5.2A$C.A5.A$2.A5.A4.CBA$.
10A3.A25.CB$2.A2.A2.A.B2.3A23.2A.A$.ABC3.ABC4.A25.3A$13.ABC23.BA.B$
39.A.C$6.A4.A6.A$5.15A$6.A2.A2.A2.A2.A9$15.C$14.A.B$13.4A$12.A.A$12.B
.A$12.C3A$14.A$14.A.C$4.ABC6.3AB$2.A2.A8.A.A$.6A5.A.A$2.A3.2A4.B3A$2.
A4.2A3.C.A24.CBA$.2A5.3A3.A25.A$A.A5.A.B2.3AC22.3A$B.A5.A.C3.A.B21.B
2A.A$C9A4.A.A23.CB$2.A2.A2.A3.4A$4.ABC5.B.A$11.2AC$11.AB$6.A4.A6.A$5.
15A$6.A2.A2.A2.A2.A!";

    #[test]
    fn test_rle_star_wars() {
        let pattern = Pattern::from_rle(STAR_WARS_RLE);
        assert_eq!(pattern.life.rule, LifeRule::STAR_WARS);
        assert_eq!(pattern.to_rle(), STAR_WARS_RLE);
    }
}
