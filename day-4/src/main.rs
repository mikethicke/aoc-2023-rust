use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

const INPUT_FILE : &str = "input.txt";

fn main() {
    let path = Path::new(INPUT_FILE);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut total_score = 0;
    let mut total_count = 0;
    let mut card_counts: HashMap<u32,u32> = HashMap::new();

    let line_re = Regex::new(r"Card\W+(\d+): (.*?) \| (.*)" ).unwrap();
    let numbers_re = Regex::new(r"\d+").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let captures = line_re.captures(&line).unwrap();
        let card_number = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let winners_str = captures.get(2).unwrap().as_str();
        let yours_str = captures.get(3).unwrap().as_str();
        let winners: Vec<u32> = numbers_re.find_iter(winners_str).map(|x| x.as_str().parse::<u32>().unwrap()).collect();
        let yours_iter = numbers_re.find_iter(yours_str);
        let mut winner_count = 0;
        for card in yours_iter {
            let card_number = card.as_str().parse::<u32>().unwrap();
            if winners.contains(&card_number) {
                winner_count += 1;
            }
        }
        let count_of_this_card = card_counts.entry(card_number).or_insert(1).clone();
        total_count += count_of_this_card;
        if winner_count > 0 {
            total_score += u32::pow(2, winner_count - 1);
            for i in 1..winner_count + 1 {
                let count_of_forward_card = card_counts.entry(card_number + i).or_insert(1);
                *count_of_forward_card += count_of_this_card;
            }
        }
    }
    println!("Total score: {}", total_score);
    println!("Total count: {}", total_count);
}