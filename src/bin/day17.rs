use std::fs::read_to_string;
use std::str::FromStr;
use anyhow::{Error, Result};

fn main() -> Result<()>{
    let input = read_to_string("data/day17.txt")?;
    let mut nick: Nick = input.parse().unwrap();
    let mut rock_spawner = RockSpawner::new();

    let mut stopped_rocks: i64 = 0;
    let mut stack: Vec<(usize, usize)> = Vec::new();

    let mut previous_height = 0;
    let mut heights: Vec<usize> = Vec::new();
    while stopped_rocks != 5000 {

        // spawn next rock
        // every rock starts with x=2 (2 units from the left edge)
        // bottom y of rock should be highest y + 4
        // get highest y from the collection of tuples (coords where the rocks reside)
        let highest_y = get_highest_y(&stack);
        let y: usize;
        if highest_y.is_some() {
            heights.push(highest_y.unwrap() - previous_height);
            previous_height = highest_y.unwrap();

            y = highest_y.unwrap() + 4;
        } else {
            y = 3;
        }

        // println!("y: {}", y);
        let mut rock = rock_spawner.spawn_rock(y);
        // println!("spawned rock of shape {:?} at {:?}", rock.shape, rock.coordinates);


        // now loop until we have a hit while alternating between toots and moves
        let mut hit = false;
        while !hit {
            // first toot, adapt coords of shape
            let direction = nick.toot();
            // shift left or right
            rock.shift(direction, &stack);
            // println!("shifted rock {:?} at {:?}",direction, rock.coordinates);
            // move down
            if !rock.down(&stack) {
                // hit!
                hit = true;
                // increment stopped_rocks
                stopped_rocks+=1;
                // register hit in stack
                for (rock_x, rock_y) in &rock.coordinates {
                    stack.push((*rock_x, *rock_y));
                }
                // println!("stored rock at {:?}", rock.coordinates);
                // println!("stopped rocks {}", stopped_rocks);
            }
            else {
                // println!("moved rock down to {:?}", rock.coordinates);
            }
            // no hit --> toot again + move (same logic)
            // continue loop
        }
    }
    println!("{:?}", heights);
    println!("part 1: {}", get_highest_y(&stack).unwrap() + 1);
    Ok(())
}

fn get_highest_y(stack: &Vec<(usize, usize)>) -> Option<usize> {
    if stack.is_empty() {
        return None;
    }
    Some(*stack.iter().map(|coordinate| coordinate.1).collect::<Vec<usize>>().iter().max().unwrap())
}

// gas factory ; dutch oven
#[derive(Debug)]
struct Nick {
    jets_of_gas: Vec<Direction>,
    counter: usize,
}
impl Nick {
    fn toot(&mut self) -> &Direction {
        if self.counter == self.jets_of_gas.len() {
            self.counter = 0;
        }
        let toot = self.jets_of_gas.get(self.counter).unwrap();
        self.counter+=1;
        toot
    }

}
impl FromStr for Nick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut jets_of_gas: Vec<Direction> = Vec::new();
        for char in s.chars() {
            jets_of_gas.push(char.to_string().parse().unwrap());
        }
        Ok(Self{ jets_of_gas, counter: 0 })
    }
}

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}
impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == ">" {
            return Ok(Self::RIGHT);
        }
        return Ok(Self::LEFT);
    }
}

#[derive(Debug)]
enum RockShape {
    MINUS,
    PLUS,
    EL,
    STRAIGHT,
    SQUARE,
}

struct RockSpawner {
    counter: usize,
}
impl RockSpawner {
    fn new() -> Self {
        RockSpawner{ counter: 0 }
    }
    
    fn spawn_rock (&mut self, y: usize) -> Rock {
        if self.counter == 5 {
            self.counter = 0;
        }
        let rock = match self.counter {
            0 => Rock::new(RockShape::MINUS, y),
            1 => Rock::new(RockShape::PLUS, y),
            2 => Rock::new(RockShape::EL, y),
            3 => Rock::new(RockShape::STRAIGHT, y),
            4 => Rock::new(RockShape::SQUARE, y),
            _ => panic!("aoc rekt us"),
        };
        self.counter += 1;
        rock
    }
}

struct Rock {
    shape: RockShape,
    coordinates: Vec<(usize, usize)>,
}
impl Rock {
    fn new(shape: RockShape, y: usize) -> Self {
        let mut coordinates: Vec<(usize, usize)> = Vec::new();
        match shape {
            RockShape::MINUS => {
                coordinates.push((2, y));
                coordinates.push((3, y));
                coordinates.push((4, y));
                coordinates.push((5, y));
            }
            RockShape::PLUS => {
                coordinates.push((2, y+1)); // left leg
                coordinates.push((4, y+1)); // right leg
                coordinates.push((3, y+2)); // top leg
                coordinates.push((3, y)); // down leg
                // center can't hit
            }
            RockShape::EL => {
                coordinates.push((2, y)); // bottom left
                coordinates.push((3, y)); // bottom centre
                coordinates.push((4, y)); // bottom right
                coordinates.push((4, y+2)); // leg top
                coordinates.push((4, y+1)); // leg middle
            }
            RockShape::STRAIGHT => {
                coordinates.push((2, y)); // bottom
                coordinates.push((2, y+1)); // bottom middle
                coordinates.push((2, y+2)); // top middle
                coordinates.push((2, y+3)); // top
            }
            RockShape::SQUARE => {
                coordinates.push((2, y)); // bottom left
                coordinates.push((3, y)); // bottom right
                coordinates.push((2, y+1)); // top left
                coordinates.push((3, y+1)); // top right
            }
        }
        Self{ shape, coordinates }
    }

    fn shift(&mut self, direction: &Direction, stack: &Vec<(usize, usize)>) {
        let x_coordinates = self.coordinates.iter().map(|(x, _)| *x).collect::<Vec<usize>>();
        let min_rock_x = x_coordinates.iter().min().unwrap();
        let max_rock_x = x_coordinates.iter().max().unwrap();

        // can we shift?
        match direction {
            Direction::LEFT => {
                if *min_rock_x == 0 {
                    return;
                }
                for (part_x, part_y) in &self.coordinates {
                    if stack.contains(&(part_x - 1, *part_y)) {
                        return;
                    }
                }
            }
            Direction::RIGHT => {
                if *max_rock_x == 6 {
                    return;
                }
                for (part_x, part_y) in &self.coordinates {
                    if stack.contains(&(part_x + 1, *part_y)) {
                        return;
                    }
                }
            }
        }

        // shift
        match direction {
            Direction::LEFT => {
                for (part_x, _) in self.coordinates.iter_mut() {
                    *part_x -= 1;
                }
            }
            Direction::RIGHT => {
                for (part_x, _) in self.coordinates.iter_mut() {
                    *part_x += 1;
                }
            }
        }
    }

    // return true if we would hit
    fn down(&mut self, stack: &Vec<(usize, usize)>) -> bool{
        let y_coordinates = self.coordinates.iter().map(|(_, y)| *y).collect::<Vec<usize>>();
        let min_rock_y = y_coordinates.iter().min().unwrap();

        // can we move down?
        if *min_rock_y == 0 {
            return false;
        }
        for (part_x, part_y) in &self.coordinates {
            if stack.contains(&(*part_x, part_y-1)) {
                return false;
            }
        }

        // move down
        for (_, part_y) in self.coordinates.iter_mut() {
            *part_y -= 1;
        }
        return true; // return true because we moved
    }
}

#[cfg(test)]
mod tests {
    use crate::get_highest_y;

    #[test]
    fn test() {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((1,2));
        stack.push((0,7));
        stack.push((4,5));

        let mut stack2: Vec<(usize, usize)> = Vec::new();
        stack2.push((3,6));

        assert_eq!(get_highest_y(&stack).unwrap(), 7);

        assert!(matches!(stack, stack2));
    }
}