use std::fs::read_to_string;
use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_to_string("data/day22.txt")?;

    let (map_note, directions_note) = input.split_once("\n\r\n").unwrap();

    let height = map_note.lines().count();
    let width = map_note.lines()
        .map(|x| x.chars().count())
        .max()
        .unwrap();

    let mut map = vec![vec![' '; width]; height];

    for (y, row) in map_note.lines().enumerate() {
        for (x, value) in row.chars().enumerate() {
            map[y][x] = value;
        }
    }


    let mut current_position: (usize, usize) = (0, 0);
    current_position.0 = map.iter().next().unwrap().iter().position(|x| *x == '.').unwrap();
    let mut current_pointing = Pointing::RIGHT;

    let mut directions: Vec<String> = Vec::new();

    let mut buffer = String::new();
    for character in directions_note.chars() {
        buffer.push(character);
        if character.is_alphabetic() {
            directions.push(buffer.to_string());
            buffer.clear();
        }
    }

    println!("{:?}", directions);
    for direction in &directions {
        let amount = *&direction[..direction.len()-1].parse::<usize>().unwrap();
        let dir: char = direction.chars().last().unwrap();
        println!("{} {}", amount, dir);

        // move x
        for _ in 0..amount {

        }

        // turn
        current_pointing = current_pointing.turn(dir);
    }


    println!("start is {:?}", current_position);
    print_map(&map);
    // println!("{} {}", height, width);



    Ok(())
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{:?}", row);
    }
}

struct Me {
    position: (usize, usize),
    pointing: Pointing,
}
impl Me {
    fn step(&mut self, map: &Vec<Vec<char>>) -> bool {
        // todo move 1 step and return true if adapted

        match self.pointing {
            Pointing::LEFT => {
                if map[self.position.1][self.position.0 - 1] == '.' {

                }
            }
            Pointing::RIGHT => {}
            Pointing::UP => {}
            Pointing::DOWN => {}
        }

        true
    }

    fn turn(&mut self, direction: char) {
        self.pointing = self.pointing.turn(direction);
    }
}

enum Pointing {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
impl Pointing {
    fn turn(&self, direction: char) -> Self {
        match (self, direction) {
            (Pointing::LEFT, 'R') => Pointing::UP,
            (Pointing::LEFT, 'L') => Pointing::DOWN,
            (Pointing::RIGHT, 'R') => Pointing::DOWN,
            (Pointing::RIGHT,'L') => Pointing::UP,
            (Pointing::UP, 'R') => Pointing::RIGHT,
            (Pointing::UP, 'L') => Pointing::LEFT,
            (Pointing::DOWN, 'R') => Pointing::LEFT,
            (Pointing::DOWN, 'L') => Pointing::RIGHT,
            _ => panic!("rekt")
        }
    }
}