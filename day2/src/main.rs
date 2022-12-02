extern crate lazy_static;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;
use std::iter::{Iterator};
use lazy_static::lazy_static;

const FILE_PATH: &str = "src/input.txt";

const LOSE_POINTS: i32 = 0;
const DRAW_POINTS: i32 = 3;
const WIN_POINTS: i32 = 6;

const MOVES: [&str; 3] = ["rock", "paper", "scissors"];

lazy_static! {
    static ref CONVERSION_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("A", "rock");
        m.insert("B", "paper");
        m.insert("C", "scissors");
        m.insert("X", "rock"); // only part 1
        m.insert("Y", "paper"); // only part 1
        m.insert("Z", "scissors"); // only part 1
        m
    };
}

lazy_static! {
    static ref USAGE_SCORE: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("rock", 1); // using rock grants 1 point
        m.insert("paper", 2); // using paper grants 2 points
        m.insert("scissors", 3); // using scissors grants 3 points
        m
    };
}

lazy_static! {
    static ref GAME_SCORE: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("X", LOSE_POINTS); // lose
        m.insert("Y", DRAW_POINTS); // draw
        m.insert("Z", WIN_POINTS); // win
        m
    };
}


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

    let split: Vec<&str> = round.split(" ").collect();
    let opponent_move = *CONVERSION_MAP.get(split.get(0).unwrap()).unwrap();
    let own_move = *CONVERSION_MAP.get(split.get(1).unwrap()).unwrap();

    round_score += USAGE_SCORE.get(own_move).unwrap();
    round_score += determine_round_score_part1(opponent_move, own_move);

    return round_score;
}

fn determine_round_score_part1(opponent_move: &str, own_move: &str) -> i32 {
    let opponent_move_index = MOVES.iter().position(|&x| x == opponent_move).unwrap();
    let own_move_index = MOVES.iter().position(|&x| x == own_move).unwrap();

    return if opponent_move == own_move {
        DRAW_POINTS
    }  else if opponent_move_index == (own_move_index + 1) % 3 {
        LOSE_POINTS
    } else {
        WIN_POINTS
    }
}

fn calculate_round_part2(round: String) -> i32 {
    let mut round_score = 0;

    let split: Vec<&str> = round.split(" ").collect();
    let opponent_move = *CONVERSION_MAP.get(split.get(0).unwrap()).unwrap();
    let needed_outcome = *split.get(1).unwrap();

    let needed_move = calculate_needed_move(needed_outcome, opponent_move);

    round_score += USAGE_SCORE.get(needed_move).unwrap();
    round_score += GAME_SCORE.get(needed_outcome).unwrap();

    return round_score;
}

fn calculate_needed_move<'a>(needed_outcome: &'a str, opponent_move: &'a str) -> &'a str {
    let move_index = MOVES.iter().position(|&x| x == opponent_move).unwrap();
    return if needed_outcome == "Y" {
        opponent_move
    } else if needed_outcome == "Z" {
        MOVES[(move_index + 1) % 3]
    } else {
        MOVES[(move_index + 2) % 3]
    }
}

fn read(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let lines: Vec<String> = br.lines().collect::<Result<_, _>>().unwrap();
    Ok(lines)
}