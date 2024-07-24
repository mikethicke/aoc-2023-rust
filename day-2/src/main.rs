use regex::Regex;
use std::io::BufRead;

const INPUT_FILE : &str = "input.txt";
const CONSTRAINTS: [i32; 3] = [12, 13, 14];

fn is_possible( game: String, constraint: [i32; 3] ) -> bool {
    let sets = game.split(";").collect::<Vec<&str>>();
    let re = Regex::new(r"\W?(\d+) (\w+)\W?").unwrap();
    for set in sets {
        let mut valid = true;
        let values = set.split(",").collect::<Vec<&str>>();
        for value in values {
            let captures = re.captures(value).unwrap();
            let number = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color = captures.get(2).unwrap().as_str();
            if color == "red" && number > constraint[0] {
                valid = false;
            } else if color == "green" && number > constraint[1] {
                valid = false;
            } else if color == "blue" && number > constraint[2] {
                valid = false;
            }
        }
        if ! valid {
            return false;
        }
    }  
    return true;     
}

fn power( game: String, constraint: [i32; 3] ) -> i32 {
    let sets = game.split(";").collect::<Vec<&str>>();
    let re = Regex::new(r"\W?(\d+) (\w+)\W?").unwrap();
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for set in sets {
        let values = set.split(",").collect::<Vec<&str>>();
        for value in values {
            let captures = re.captures(value).unwrap();
            let number = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color = captures.get(2).unwrap().as_str();
            if color == "red" && number > min_red {
                min_red = number;
            } else if color == "green" && number > min_green {
                min_green = number;
            } else if color == "blue" && number > min_blue {
                min_blue = number;
            }
        }
    }
    return min_red * min_green * min_blue;
}

                

fn main() {
    let path = std::path::Path::new(INPUT_FILE);
    let file = std::fs::File::open(&path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut sum_of_valid_games = 0;
    let mut sum_powers = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let line_parts = line.split(":").collect::<Vec<&str>>();
        let game = line_parts[1].to_string();
        let game_id_parts = line_parts[0].split(" ").collect::<Vec<&str>>();
        let game_id = game_id_parts[1].parse::<i32>().unwrap();
        if is_possible(game.clone(), CONSTRAINTS) {
            sum_of_valid_games += game_id;
        }
        sum_powers += power(game, CONSTRAINTS);
    }
    println!("Sum of valid games: {}", sum_of_valid_games);
    println!("Sum of powers: {}", sum_powers);
}
