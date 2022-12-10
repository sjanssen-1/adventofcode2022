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

    fn follow(&mut self, knot: Knot) {
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

    fn follow(&mut self, knot: Knot) {
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
    println!("part 1: {}", boobs(2)?);
    println!("part 2: {}", boobs(10)?);
    Ok(())
}

fn boobs(knots_amount: usize) -> Result<usize> {
    let mut rope = vec![Knot::new(); knots_amount-1];
    let mut tail = Tail::new();
    let moves = read_to_string("data/day9_personal.txt")?;
    for move_ in moves.lines() {
        let (direction, amount) = move_.split_once(" ").unwrap();
        for _n_ in 0..amount.parse::<usize>()? {
            rope[0].move_(direction);
            for m in 1..rope.len() {
                follow(rope[m-1], &mut rope[m]);
            }
            tail.follow(rope[rope.len()-1]);
        }
    }
    Ok(tail.unique_positions.len())
}

fn follow(to_follow: Knot, follower: &mut Knot) {
    follower.follow(to_follow);
}