extern crate core;

use std::collections::HashSet;
use std::fs::read_to_string;
use anyhow::{Error, Result};


#[derive(Copy, Clone, Debug)]
struct Knot {
    current_position: Position,
    previous_position: Position,
}
impl Knot {
    fn new() -> Self {
        Self { current_position: Position { x: 0, y: 0 }, previous_position: Position { x: 0, y: 0 } }
    }

    fn move_(&mut self, direction: &str) {
        self.previous_position.x = self.current_position.x;
        self.previous_position.y = self.current_position.y;
        match direction {
            "R" => self.current_position.x += 1,
            "L" => self.current_position.x -= 1,
            "U" => self.current_position.y += 1,
            "D" => self.current_position.y -= 1,
            _ => panic!("get rekt")
        }
    }

    fn follow(&mut self, knot: &Knot) {
        let calculated_move = self.current_position.calculate_follow_move(&knot.current_position);
        if !calculated_move.is_ok() {
            return;
        }
        self.previous_position.x = self.current_position.x;
        self.previous_position.y = self.current_position.y;
        self.current_position.position_move(calculated_move.unwrap());
    }
}

#[derive(Debug)]
struct Tail {
    current_position: Position,
    unique_positions: HashSet<Position>,
}
impl Tail {
    fn new() -> Self {
        let mut unique_positions: HashSet<Position> = HashSet::new();
        unique_positions.insert(Position{x: 0, y: 0});
        Self { current_position: Position{x: 0, y: 0}, unique_positions }
    }

    fn follow(&mut self, knot: &Knot) {
        let calculated_move = self.current_position.calculate_follow_move(&knot.current_position);
        if !calculated_move.is_ok() {
            return;
        }
        self.current_position.position_move(calculated_move.unwrap());
        self.unique_positions.insert(Position{ x: self.current_position.x, y: self.current_position.y });
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Position {
    x: i16,
    y: i16,
}
impl Position {
    fn calculate_follow_move(&self, position: &Position) -> Result<Direction> {
        if self.x - position.x == 2 {
            if self.y == position.y {
                Ok(Direction::Left)
            } else if self.y > position.y {
                Ok(Direction::DownLeft)
            } else {
                Ok(Direction::UpLeft)
            }
        } else if position.x - self.x == 2 {
            if self.y == position.y {
                Ok(Direction::Right)
            } else if self.y > position.y {
                Ok(Direction::DownRight)
            } else {
                Ok(Direction::UpRight)
            }
        } else if self.y - position.y == 2 {
            if self.x == position.x {
                Ok(Direction::Down)
            } else if self.x > position.x {
                Ok(Direction::DownLeft)
            } else {
                Ok(Direction::DownRight)
            }
        } else if position.y - self.y == 2 {
            if self.x == position.x {
                Ok(Direction::Up)
            } else if self.x > position.x {
                Ok(Direction::UpLeft)
            } else {
                Ok(Direction::UpRight)
            }
        } else {
            Err(Error::msg("get rekt"))
        }
    }

    fn position_move(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::UpRight => {
                self.x += 1;
                self.y += 1;
            },
            Direction::UpLeft => {
                self.x -= 1;
                self.y += 1;
            },
            Direction::DownRight => {
                self.x += 1;
                self.y -= 1;
            },
            Direction::DownLeft => {
                self.x -= 1;
                self.y -= 1;
            },
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<()>{
    let mut head = Knot::new();
    let mut tail = Tail::new();
    let moves = read_to_string("data/day9_personal.txt")?;
    for move_ in moves.lines() {
        let (direction, amount) = move_.split_once(" ").unwrap();
        for _n_ in 0..amount.parse::<usize>()? {
            head.move_(direction);
            tail.follow(&head);
        }
    }
    println!{"Unique tail positions (part 1): {}", tail.unique_positions.len()};
    Ok(())
}

fn part2() -> Result<()> {
    let mut head = Knot::new();
    let mut knot1 = Knot::new();
    let mut knot2 = Knot::new();
    let mut knot3 = Knot::new();
    let mut knot4 = Knot::new();
    let mut knot5 = Knot::new();
    let mut knot6 = Knot::new();
    let mut knot7 = Knot::new();
    let mut knot8 = Knot::new();
    let mut tail = Tail::new();
    let moves = read_to_string("data/day9_personal.txt")?;
    for move_ in moves.lines() {
        let (direction, amount) = move_.split_once(" ").unwrap();
        for _n_ in 0..amount.parse::<usize>()? {
            head.move_(direction);
            knot1.follow(&head);
            knot2.follow(&knot1);
            knot3.follow(&knot2);
            knot4.follow(&knot3);
            knot5.follow(&knot4);
            knot6.follow(&knot5);
            knot7.follow(&knot6);
            knot8.follow(&knot7);
            tail.follow(&knot8);
        }
    }
    println!{"Unique tail positions (part 2): {}", tail.unique_positions.len()};
    Ok(())
}