use std::fs::read_to_string;
use anyhow::Result;

fn main() -> Result<()>{
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<()> {
    let mut register_x: i16 = 1;
    let mut cycle_count: u32 = 0;
    let mut signal_checker = SignalChecker::new();
    let program = read_to_string("data/day10_personal.txt")?;

    for instruction in program.lines() {
        if instruction == "noop" {
            cycle_count += 1;
            signal_checker.check_signal(register_x, cycle_count);
        } else if instruction.starts_with("addx") {
            cycle_count += 1;
            signal_checker.check_signal(register_x, cycle_count);

            cycle_count += 1;
            signal_checker.check_signal(register_x, cycle_count);
            let increment = instruction.split_once(" ").unwrap().1.parse::<i16>().unwrap();
            register_x += increment;
        }
        if signal_checker.check_end() {
            break;
        }
    }
    println!("Signal strength (part 1): {}", signal_checker.signal_strength());
    Ok(())
}

fn part2() -> Result<()> {
    let mut register_x: i16 = 1;
    let mut cycle_count: u32 = 0;
    let mut crt = CRT::new();
    let program = read_to_string("data/day10_personal.txt")?;

    for instruction in program.lines() {
        if instruction == "noop" {
            cycle_count += 1;
            crt.draw(cycle_count, register_x);
        } else if instruction.starts_with("addx") {
            cycle_count += 1;
            crt.draw(cycle_count, register_x);
            cycle_count += 1;
            crt.draw(cycle_count, register_x);
            let increment = instruction.split_once(" ").unwrap().1.parse::<i16>().unwrap();
            register_x += increment;
        }
        if cycle_count == 240 {
            break;
        }
    }
    crt.show_screen();
    Ok(())
}

struct CRT {
    screen: Vec<Vec<char>>,
    row: usize,
}
impl CRT {
    fn new() -> Self {
        Self{ screen: vec![vec!['.';40]; 6], row: 0 }
    }

    fn draw(&mut self, cycle: u32, register_x: i16) {
        let position_in_row = cycle - (self.row * 40) as u32 - 1;
        if position_in_row == register_x as u32 || position_in_row == (register_x - 1) as u32 || position_in_row == (register_x + 1) as u32 {
            self.screen[self.row][position_in_row as usize] = '#';
        }

        match cycle {
            40 | 80 | 120 | 160 | 200 | 240 => self.row += 1,
            _ => {}
        }
    }

    fn show_screen(&self) {
        let mut screen_string = String::new();
        for y in 0..6 {
            for x in 0..40 {
                screen_string.push(self.screen[y][x]);
            }
            screen_string.push('\r');
            screen_string.push('\n');
        }
        println!("{}", screen_string);
    }

}

struct SignalChecker {
    signal_20: i16,
    signal_60: i16,
    signal_100: i16,
    signal_140: i16,
    signal_180: i16,
    signal_220: i16,
}
impl SignalChecker {
    fn new() -> Self {
        Self{
            signal_20: 0,
            signal_60: 0,
            signal_100: 0,
            signal_140: 0,
            signal_180: 0,
            signal_220: 0,
        }
    }

    fn check_signal(&mut self, register_x: i16, current_cycle: u32) {
        match current_cycle {
            20 => self.signal_20 = 20 * register_x,
            60 => self.signal_60 = 60 * register_x,
            100 => self.signal_100 = 100 * register_x,
            140 => self.signal_140 = 140 * register_x,
            180 => self.signal_180 = 180 * register_x,
            220 => self.signal_220 = 220 * register_x,
            _ => {}
        }
    }

    fn check_end(&self) -> bool {
        self.signal_220 != 0
    }

    fn signal_strength(&self) -> i16 {
        self.signal_20 + self.signal_60 + self.signal_100 + self.signal_140 + self.signal_180 + self.signal_220
    }
}