use std::collections::HashSet;
use std::fs::read_to_string;
use anyhow::Result;

#[derive(Copy, Clone)]
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
        let delta_x = knot.current_position.x - self.current_position.x;
        let delta_y = knot.current_position.y - self.current_position.y;
        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            self.previous_position.x = self.current_position.x;
            self.previous_position.y = self.current_position.y;
            self.current_position.x += delta_x.signum();
            self.current_position.y += delta_y.signum();
        }
    }
}

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
        let delta_x = knot.current_position.x - self.current_position.x;
        let delta_y = knot.current_position.y - self.current_position.y;
        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            self.current_position.x += delta_x.signum();
            self.current_position.y += delta_y.signum();
            self.unique_positions.insert(Position{ x: self.current_position.x, y: self.current_position.y });
        }
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i16,
    y: i16,
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