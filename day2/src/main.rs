extern crate lazy_static;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;
use std::iter::{Iterator};
use lazy_static::lazy_static;

lazy_static! {
    static ref USAGE_SCORE_PART1: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("X", 1);
        m.insert("Y", 2);
        m.insert("Z", 3);
        m
    };
}

lazy_static! {
    static ref GAME_SCORE_PART1: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("A X", 3);
        m.insert("B Y", 3);
        m.insert("C Z", 3);
        m.insert("A Y", 6);
        m.insert("B Z", 6);
        m.insert("C X", 6);
        m
    };
}

lazy_static! {
    static ref GAME_SCORE_PART2: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("X", 0);
        m.insert("Y", 3);
        m.insert("Z", 6);
        m
    };
}

lazy_static! {
    static ref USAGE_SCORE_PART2: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("A", 1);
        m.insert("B", 2);
        m.insert("C", 3);
        m
    };
}

const FILE_PATH: &str = "src/input.txt";

fn main() -> Result<(), Error> {
    let strategy_guide = read(FILE_PATH)?;
    simulate_strategy(strategy_guide);
    Ok(())
}

fn simulate_strategy(strategy_guide: Vec<String>) {
    let mut score_part1 = 0;
    let mut score_part2 = 0;
    for round in strategy_guide {
        score_part1 += calculate_round_part1(round.clone());
        score_part2 += calculate_round_part2(round.clone());
    }
    println!("Simulated game score (part1): {}", score_part1);
    println!("Simulated game score (part2): {}", score_part2);
}

fn calculate_round_part1(round: String) -> i32 {
    let mut round_score = 0;
    let own_move = round.split(" ").nth(1).unwrap();

    round_score += USAGE_SCORE_PART1.get(own_move).unwrap();
    round_score += GAME_SCORE_PART1.get(&*round).or(Option::from(&0)).unwrap();

    return round_score;
}

fn calculate_round_part2(round: String) -> i32 {
    let mut round_score = 0;
    let split: Vec<&str> = round.split(" ").collect();

    let opponent_move = split.get(0).unwrap();
    let needed_outcome = split.get(1).unwrap();

    let needed_move = calculate_needed_move(needed_outcome, opponent_move);

    round_score += USAGE_SCORE_PART2.get(needed_move).unwrap();
    round_score += GAME_SCORE_PART2.get(needed_outcome).unwrap();

    return round_score;
}

fn calculate_needed_move<'a>(needed_outcome: &'a str, opponent_move: &'a str) -> &'a str {
    let moves = ["A", "B", "C"];
    let move_index = moves.iter().position(|&x| x == opponent_move).unwrap();
    return if needed_outcome == "Y" {
        opponent_move
    } else if needed_outcome == "Z" {
        moves[if move_index == 2 { 0 } else { move_index+1 }]
    } else {
        moves[if move_index == 0 { 2 } else { move_index-1 }]
    }
}

fn read(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let lines: Vec<String> = br.lines().collect::<Result<_, _>>().unwrap();
    Ok(lines)
}