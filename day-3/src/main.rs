use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;

const INPUT_FILE: &str = "input.txt";

struct Part {
    line: i32,
    start_pos: i32,
    end_pos: i32,
    part_number: i32,
}

struct Gear {
    part_count: i32,
    ratio: i32,
}

fn main() {
    let path = Path::new(INPUT_FILE);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut symbols = HashMap::new();
    let mut gears: HashMap<String, Gear> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut number = String::new();
        let mut start_pos = -1;
        for (pos, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if number == "" {
                    start_pos = pos as i32;
                }
                number.push(c);
                continue;
            }
            if number != "" {
                let part_number = number.parse::<i32>().unwrap();
                let end_pos = pos as i32 - 1;
                let new_part = Part {
                    line: line_number as i32,
                    start_pos,
                    end_pos,
                    part_number,
                };
                parts.push(new_part);
                number = String::new();
            }   
            if c != '.' && ! c.is_numeric() {
                symbols.insert(format!("{}-{}", line_number, pos as i32), c);
            }
        }
        if number != "" {
            let end_pos = line.len() as i32 - 1;
            let part_number = number.parse::<i32>().unwrap();
            let new_part = Part {
                line: line_number as i32,
                start_pos,
                end_pos,
                part_number,
            };
            parts.push(new_part);
        }
    }

    let mut part_sum = 0;
    'part: for part in parts {
        for line in part.line - 1 .. part.line + 2 {
            for i in part.start_pos - 1 .. part.end_pos + 2 {
                let key = format!("{}-{}", line, i);
                if symbols.contains_key(&key) {
                    part_sum += part.part_number;
                    let symbol = symbols.get(&key).unwrap();
                    if *symbol == '*' {
                        if gears.contains_key(&key) {
                            let gear = gears.get_mut(&key).unwrap();
                            gear.part_count += 1;
                            gear.ratio *= part.part_number;
                        } else {
                            let gear = Gear {
                                part_count: 1,
                                ratio: part.part_number,
                            };
                            gears.insert(key, gear);
                        }
                        continue 'part;
                    }
                }
            }
        }
    }

    let mut gear_sum = 0;
    for gear in gears.values() {
        if gear.part_count == 2 {
            gear_sum += gear.ratio;
        }
    }
    println!("Part sum: {}", part_sum);
    println!("Gear sum: {}", gear_sum);

}
