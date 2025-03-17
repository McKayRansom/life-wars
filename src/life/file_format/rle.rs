use std::str::Split;

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

/*
 *        let rle_glider = "
 *        #C This is a glider.
 *        x = 3, y = 3
 *        bo$2bo$3o!";
 * https://conwaylife.com/wiki/Run_Length_Encoded
 */
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
        // 24bo11b$22bobo11b$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o14b$2o8b
        // 24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b\n
        // 4bo

        // 2.ABC$2.A2.A$.6A.A$2.A3.2A.B$A.A4.2A.C$B2A5.2A$C.A5.A$2.A5.A4.CBA$.\n
        // 10A3.A25.CB$2.A2.A2.A.B2.3A23.2A.A$.ABC3.ABC4.A25.3A$13.ABC23.BA.B$\n
        // 39.A.C$6.A4.A6.A$5.15A$6.A2.A2.A2.A2.A9$15.C$14.A.B$13.4A$12.A.A$12.B\n
        // .A$12.C3A$14.A$14.A.C$4.ABC6.3AB$2.A2.A8.A.A$.6A5.A.A$2.A3.2A4.B3A$2.\n
        // A4.2A3.C.A24.CBA$.2A5.3A3.A25.A$A.A5.A.B2.3AC2
        // 2A38.2A2.2A2.2A11.A9.A8.2A6.2A10.2A2.2A.A2.A.B$.A39.A4.A2.A11.CBA7.AB
        if self.count > 1 {
            let count_str = self.count.to_string();
            if *line_count + count_str.len() + 2 >= 69 {
                println!("Wrapped at {line_count}");
                string.push('\n');
                *line_count = 0;
            }
            *line_count += count_str.len();
            string.push_str(count_str.as_str());
        } else {
            if *line_count + 1 >= 69 {
                println!("Wrapped at {line_count}");
                string.push('\n');
                *line_count = 0;
            }
        }
        string.push(self.tag);
        *line_count += 1;
    }

    fn is_alive(&self) -> bool {
        self.tag != 'b' && self.tag != '.' && self.tag != '$'
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

    // let mut prev_count = 0;
    // let mut prev_state: Option<u8> = None;
    let mut prev_item: Option<RleItem> = None;
    let mut line_count: usize = 0;
    for (x, y, cell) in life.iter() {
        if x == 0 && y != 0 {
            if let Some(item) = prev_item {
                if item.is_alive() {
                    item.write(&mut string, &mut line_count);
                }
                // prev_item.unwrap().write(
                //     &mut string,
                //     // prev_count,
                //     // tag_func(prev_state.unwrap()),
                //     &mut line_count,
                // );
            }

            // prev_state = None;
            // prev_count = 0;
            prev_item = None;

            string.push('$');
            line_count += 1;
        }
        if let Some(item) = &mut prev_item {
            if item.tag != tag_func(cell.get_state()) {
                item.write(&mut string, &mut line_count);
                // rle_write_state(&mut string, prev_count, tag_func(state), &mut line_count);
                // prev_count = 0;
                prev_item = Some(RleItem{ tag: tag_func(cell.get_state()), count: 1});
            } else {
                item.count += 1;
            }
        } else {
            prev_item = Some(RleItem{ tag: tag_func(cell.get_state()), count: 1});
        }
    }
    if let Some(item) = &prev_item {
        item.write(&mut string, &mut line_count);
        // rle_write_state(
        //     &mut string,
        //     prev_count,
        //     tag_func(prev_state.unwrap()),
        //     &mut line_count,
        // );
    }

    string.push('!');

    string
}
