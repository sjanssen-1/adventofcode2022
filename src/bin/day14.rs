use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use anyhow::Result;

fn main() -> Result<()>{
    let scan = read_to_string("data/day14_personal.txt")?;

    let mut depth :usize = 0;
    for line in scan.lines() {
        for rl in line.split(" -> ") {
            let (_, y) = rl.split_once(",").unwrap();
            let xxx: usize = y.parse().unwrap();
            if xxx > depth {
                depth = xxx
            };
        }
    }

    let mut cave_part1 = Cave::new(depth+2);
    for scan_line in scan.lines() {
        cave_part1.draw_rock(scan_line);
    }
    let mut i = 0;
    while cave_part1.simulate_sand(false) {
        i+=1;
    }
    println!("The amount of sand units was (part 1): {}", i+1);

    i = 0;
    let mut cave_part2 = Cave::new(depth+2);
    for scan_line in scan.lines() {
        cave_part2.draw_rock(scan_line);
    }
    let mut i = 0;
    while cave_part2.simulate_sand(true) {
        i+=1;
    }
    println!("The amount of sand units was (part 2): {}", i+1);

    println!("{}", cave_part2);
    Ok(())
}

#[derive(Debug)]
struct Cave {
    scan: Vec<Vec<char>>,
    sand_origin: (usize, usize),
}
impl Cave {
    fn new(depth: usize) -> Self {
        let mut scan = vec![vec!['.'; 1000]; depth];
        scan[0][500] = '+';
        Self{scan, sand_origin: (500, 0) }
    }

    fn draw_rock(&mut self, rock_line: &str) {
        let mut previous_rock_end: Option<(usize, usize)> = None;
        for rl in rock_line.split(" -> ") {
            let rock_end = rl.split_once(",").unwrap();
            let rock_end_x = rock_end.0.parse::<usize>().unwrap();
            let rock_end_y: usize = rock_end.1.parse().unwrap();
            // println!("Handling = {} {}", rock_end_x, rock_end_y);
            if previous_rock_end.is_some() {
                let previous_rock_end_x = previous_rock_end.as_ref().expect("rekt").0;
                let previous_rock_end_y = previous_rock_end.as_ref().expect("rekt").1;
                let x_range= if previous_rock_end_x < rock_end_x {
                    previous_rock_end_x..=rock_end_x
                } else {
                    rock_end_x..=previous_rock_end_x
                };
                let y_range = if previous_rock_end_y < rock_end_y {
                    previous_rock_end_y..=rock_end_y
                } else {
                    rock_end_y..=previous_rock_end_y
                };
                for y in y_range.clone() {
                    for x in x_range.clone() {
                        // println!("drawing x: {} y: {}", x, y);
                        self.scan[y][x] = '#';
                    }
                }
            }
            // println!("Storing previous = {} {}", rock_end_x, rock_end_y);
            previous_rock_end = Some((rock_end_x, rock_end_y));
        }
    }

    fn simulate_sand(&mut self, go_loco: bool) -> bool {
        let (mut x, mut y) = self.sand_origin;

        for depth in y..self.scan.len() {
            // println!{"Checking sand drop {},{}", x, depth};
            if !go_loco && depth+1 == self.scan.len() {
                break;
            } else if !self.is_blocked_down(x, depth) {
                continue;
            } else if !self.is_blocked_down_left(x, depth) {
                x -= 1;
                continue;
            } else if !self.is_blocked_down_right(x, depth) {
                x += 1;
                continue;
            } else {
                self.scan[depth][x] = 'o';
                if go_loco && x == self.sand_origin.0 && depth == self.sand_origin.1 {
                    return false;
                }
                return true;
            }
        }

            // else if self.scan[depth][x] == 'o' && self.scan[depth][x-1] == '.' {
            //     self.scan[depth][x-1] = 'o';
            //     return true;
            // } else if self.scan[depth][x] == 'o' && self.scan[depth][x+1] == '.' {
            //     self.scan[depth][x+1] = 'o';
            //     return true;
            // } else if self.scan[depth][x] == 'o' && self.scan[depth][x-1] == '.' {
            //     self.scan[depth][x-1] = 'o';
            //     return true;
            // }
        return false;
        // while self.scan[y+counter][x] == '.' || self.scan[y+counter][x-1] == '.' || self.scan[y+counter][x+1] == '.'{
        //
        // }
    }

    fn is_blocked(&self, x: usize, y: usize) -> bool {
        self.is_blocked_down(x,y) && self.is_blocked_down_left(x,y) && self.is_blocked_down_right(x,y)
    }

    fn is_blocked_down(&self, x: usize, y: usize) -> bool {
        if y+1 == self.scan.len() {
            return true;
        }
        self.scan[y+1][x] == '#' || self.scan[y+1][x] == 'o'
    }

    fn is_blocked_down_left(&self, x: usize, y: usize) -> bool {
        if y+1 == self.scan.len() || x == 0 {
            return true;
        }
        (self.scan[y+1][x-1] == '#' || self.scan[y+1][x-1] == 'o')
    }

    fn is_blocked_down_right(&self, x: usize, y: usize) -> bool {
        if y+1 == self.scan.len() || x+1 == self.scan[0].len() {
            return true;
        }
        (self.scan[y+1][x+1] == '#' || self.scan[y+1][x+1] == 'o')
    }
}
impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut cave_string = String::new();
        for line in &self.scan {
            cave_string += &line.iter().collect::<String>();
            cave_string += "\n";
        }
        write!(f, "{}", cave_string)
    }
}

fn normalize_stupid_index(stupid_index: &usize) -> usize {
    stupid_index-400
}


#[cfg(test)]
mod tests {
    use crate::Cave;

    #[test]
    fn test() {
        let mut cave = Cave::new(100);
        cave.draw_rock("498,4 -> 498,6 -> 496,6");
        cave.draw_rock("503,4 -> 502,4 -> 502,9 -> 494,9");

        // cave.simulate_sand();
        // cave.simulate_sand();
        // cave.simulate_sand();
        let mut i = 0;
        while cave.simulate_sand(false) {
            i+=1;
            if i == 30 {
                break;
            }
        }
        println!("{}", i);
        println!("{}", cave);

    }
}