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
        let mut line = line.unwrap();
        let mut first_digit = '~';
        let mut last_digit = '~';
        while line.len() > 0 {
            let first_char = line.chars().next().unwrap();
            let mut digit_candidate = ' ';
            if first_char.is_digit(10) {
                digit_candidate = first_char;
            } else if line.starts_with("one") {
                digit_candidate = '1';
            } else if line.starts_with("two") {
                digit_candidate = '2';
            } else if line.starts_with("three") {
                digit_candidate = '3';
            } else if line.starts_with("four") {
                digit_candidate = '4';
            } else if line.starts_with("five") {
                digit_candidate = '5';
            } else if line.starts_with("six") {
                digit_candidate = '6';
            } else if line.starts_with("seven") {
                digit_candidate = '7';
            } else if line.starts_with("eight") {
                digit_candidate = '8';
            } else if line.starts_with("nine") {
                digit_candidate = '9';
            }
            if digit_candidate != ' ' {
                if first_digit == '~' {
                    first_digit = digit_candidate;
                }
                last_digit = digit_candidate;
            }
            line = line.split_off(1);
        }
        total_fuel += format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
    }
    println!("Total fuel: {}", total_fuel);
}
