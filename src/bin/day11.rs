use std::fs::read_to_string;
use std::str::FromStr;
use anyhow::{Error, Result};
use regex::Regex;

fn main() -> Result<()> {
    let monkeys_input = read_to_string("data/day11_personal.txt")?;

    println!("Monkey business while I still have hope (part 1): {}", calculate_monkey_business(&monkeys_input, 20, true));
    println!("Monkey business while I'm thinking of ending things (part 2): {}", calculate_monkey_business(&monkeys_input, 10000, false));
    Ok(())
}

fn calculate_monkey_business(input: &String, rounds: usize, with_hope: bool) -> usize {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey in input.split("\r\n\r\n") {
        monkeys.push(monkey.parse().unwrap());
    }

    let mut magic = 1;
    for monkey in &monkeys {
        magic *= monkey.test.value
    }

    for round in 1..=rounds {
        // println!("Round {}", round);
        for n in 0..monkeys.len() {
            // println!("Monkey {}", n);
            let monkey = &mut monkeys[n];
            let mut throws: Vec<(usize, u64)> = Vec::new();
            for _i_ in 0..monkey.items.len() {
                throws.push(monkey.inspect_item(with_hope, magic));
            }
            for (monkey_to, item) in throws {
                let monkey = &mut monkeys[monkey_to];
                monkey.items.push(item);
            }
        }
    }

    let mut inspects_per_monkey_sorted: Vec<usize> = monkeys.iter().map(|m| m.inspects).collect();
    inspects_per_monkey_sorted.sort();
    return inspects_per_monkey_sorted.last().unwrap() * inspects_per_monkey_sorted.get(inspects_per_monkey_sorted.len()-2).unwrap();
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspects: usize,
}
impl Monkey {
    fn inspect_item(&mut self, i_still_have_hope: bool, magic: u64) -> (usize, u64) {
        self.inspects+=1;
        let mut item = self.items.remove(0);
        if i_still_have_hope { println!(" Monkey inspects an item with worry level of {}", item) }
        item = match self.operation {
            Operation::ADD(value) => {
                if value == 0 {
                    let new_value = &item + &item;
                    if i_still_have_hope { println!("  Worry level increased by {} to {}", item, new_value) }
                    new_value
                } else {
                    let new_value = item + value;
                    if i_still_have_hope { println!("  Worry level increased by {} to {}", value, new_value) }
                    new_value
                }
            }
            Operation::MULTIPLY(value) => {
                if value == 0 {
                    let new_value = &item * &item;
                    if i_still_have_hope { println!("  Worry level is multiplied by {} to {}", item, new_value) }
                    new_value
                } else {
                    let new_value = item * value;
                    if i_still_have_hope { println!("  Worry level is multiplied by {} to {}", value, new_value) }
                    new_value
                }
            },
        };
        if i_still_have_hope {
            item /= 3 as u64;
            println!("  Monkey gets bored with item. Worry level is divided by 3 to {}.", item);
        }
        item %= magic;
        if item % self.test.value == 0 {
            if i_still_have_hope { println!("  Item with worry level {} is thrown to monkey {}.", item, self.test.true_action) }
            (self.test.true_action, item)
        } else {
            if i_still_have_hope { println!("  Item with worry level {} is thrown to monkey {}.", item, self.test.false_action) }
            (self.test.false_action, item)
        }
    }
}
impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let starting_items_re = Regex::new(r"((?:\d+)+),?").unwrap();

        let mut items: Vec<u64> = Vec::new();
        let mut operation: Operation = Operation::ADD(0);
        let mut test: Test = Test{
            value: 0,
            true_action: 0,
            false_action: 0,
        };

        for line in s.lines() {
            if line.contains("Starting items:") {
                for capture in starting_items_re.captures_iter(line) {
                    let value: u64 = capture.get(1).unwrap().as_str().parse().unwrap();
                    items.push(value);
                }
            } else if line.contains("Operation:") {
                let split: Vec<&str> = line.split(" ").collect();
                let operator = split.get(split.len()-2).unwrap();
                let value: u64 = if *split.last().unwrap() == "old" {
                    0
                } else {
                    split.last().unwrap().parse().unwrap()
                };
                if *operator == "*" {
                    operation = Operation::MULTIPLY(value);
                } else { // +
                    operation = Operation::ADD(value);
                }
            } else if line.contains("Test:"){
                let split: Vec<&str> = line.split(" ").collect();
                let value: u64 = split.last().unwrap().parse().unwrap();
                test.value = value;
            } else if line.contains("If true:"){
                let split: Vec<&str> = line.split(" ").collect();
                let value: usize = split.last().unwrap().parse().unwrap();
                test.true_action = value;
            } else if line.contains("If false:"){
                let split: Vec<&str> = line.split(" ").collect();
                let value: usize = split.last().unwrap().parse().unwrap();
                test.false_action = value;
            }
        }

        Ok(Self{items, operation, test, inspects: 0})
    }
}

#[derive(Debug)]
struct Test {
    value: u64,
    true_action: usize,
    false_action: usize,
}

#[derive(Debug)]
enum Operation {
    ADD(u64),
    MULTIPLY(u64),
}