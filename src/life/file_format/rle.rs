use std::str::Split;


/*
 *        let rle_glider = "
 *        #C This is a glider.
 *        x = 3, y = 3
 *        bo$2bo$3o!";
 * https://conwaylife.com/wiki/Run_Length_Encoded
 */

use crate::life::{Cell, Life, LifeAlgoSelect, LifeRule};

fn rle_parse_header(it: &mut Split<'_, char>) -> Option<Life> {
    let mut name = String::new();
    while let Some(line) = it.next() {
        // parse headers
        if line.starts_with("#N ") {
            name = line[3..].into()
        } else if line.starts_with("#") {
            // ignore tags for now
        } else if line.starts_with("x") {
            // header
            let mut size: (usize, usize) = (16, 16);
            let mut rule: LifeRule = LifeRule::GOL;
            for field in line.split(", ") {
                let (name, value) = field.split_once(" = ").expect("Failed to parse field");
                match name {
                    "x" => size.0 = value.parse().expect("Failed to parse field"),
                    "y" => size.1 = value.parse().expect("Failed to parse field"),
                    "rule" => rule = LifeRule::from_str(value),
                    _ => panic!("Unkown header field: {}", name),
                }
            }
            return Some(Life {
                rule,
                name,
                ..Life::new(LifeAlgoSelect::Basic, size)
            });
        } else {
            panic!("Unkown line: {}", line);
        }
    }
    None
}

pub fn new_life_from_rle(rle: &str) -> Life {
    let mut line_it = rle.split('\n');
    let mut life: Life = rle_parse_header(&mut line_it).expect("Failed to parse header from .rle!");

    let mut pos: (usize, usize) = (0, 0);
    while let Some(line) = line_it.next() {
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
                    'b' => pos.0 += run_count as usize,
                    '.' => pos.0 += run_count as usize,
                    'o' => {
                        for _ in 0..run_count {
                            life.insert(pos, Cell::new(1, 0));
                            pos.0 += 1;
                        }
                    }
                    // Generations tags
                    'A' => {
                        for _ in 0..run_count {
                            life.insert(pos, Cell::new(1, 0));
                            pos.0 += 1;
                        }
                    }
                    'B' => {
                        for _ in 0..run_count {
                            life.insert(pos, Cell::new(2, 0));
                            pos.0 += 1;
                        }
                    }
                    'C' => {
                        for _ in 0..run_count {
                            life.insert(pos, Cell::new(3, 0));
                            pos.0 += 1;
                        }
                    }

                    '$' => {
                        pos.1 += run_count as usize;
                        pos.0 = 0;
                    }
                    '!' => break,

                    _ => panic!("Unkown <tag> '{chr}'"),
                }
                run_count = 0;
            }
        }
    }
    life
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

pub fn life_to_rle(life: &Life) -> String {
    let size = life.size();
    let mut string = String::with_capacity(64);
    if !life.name.is_empty() {
        string.push_str("#N ");
        string.push_str(life.name.as_str());
        string.push('\n');
    }
    string.push_str(
        format!(
            "x = {}, y = {}, rule = {}\n",
            size.0,
            size.1,
            life.rule.to_str().as_str()
        )
        .as_str(),
    );

    let tag_func: fn(u8) -> char = if life.rule.is_generations() {
        rle_tag_generations
    } else {
        rle_tag
    };

    let mut writer = RleWriter {
        item: None,
        line_count: 0,
        string,
        dead: RleItem{ tag: tag_func(0), count: 0},
    };

    for (x, y, cell) in life.iter() {
        if x == 0 && y != 0 {
            writer.push('$');
        }
        writer.push(tag_func(cell.get_state()));
    }
    writer.flush();
    writer.string.push('!');

    writer.string
}
