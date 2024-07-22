use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

const INPUT_FILE : &str = "input.txt";

fn main() {
    let path = Path::new(INPUT_FILE);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut total_fuel = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut first_digit = '~';
        let mut last_digit = '~';
        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit == '~' {
                    first_digit = c;
                } 
                last_digit = c;
            }
        }
        total_fuel += format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
    }
    println!("Total fuel: {}", total_fuel);
}
