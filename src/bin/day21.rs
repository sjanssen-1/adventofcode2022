use anyhow::{Error, Result};
use itertools::Itertools;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = read_to_string("data/day21_personal.txt")?;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey in input.lines() {
        monkeys.push(monkey.parse().unwrap());
    }

    let mut known_monkeys: Vec<Monkey> = Vec::new();
    let mut unknown_monkeys: Vec<Monkey> = Vec::new();

    for m in &monkeys {
        if m.number.is_some() {
            known_monkeys.push(m.clone());
        } else {
            unknown_monkeys.push(m.clone());
        }
    }

    while known_monkeys.iter().find(|m| m.name == "root").is_none() {
        for um in &unknown_monkeys {
            let left_hit = known_monkeys.iter().find(|m| m.name == *um.left.as_ref().unwrap());
            let right_hit = known_monkeys.iter().find(|m| m.name == *um.right.as_ref().unwrap());
            if left_hit.is_some() && right_hit.is_some() {
                let left_value = left_hit.unwrap().number.unwrap();
                let right_value = right_hit.unwrap().number.unwrap();
                let operator = um.operator.unwrap();
                // println!("hit on: {} {} {}", left_value, operator, right_value);
                let number = do_math(left_value, operator, right_value);
                // println!("{}", number);

                known_monkeys.push(Monkey{
                    name: um.name.to_string(),
                    number: Some(number),
                    left: None,
                    right: None,
                    operator: None,
                });
            }
        }
    }

    println!("{:#?}", known_monkeys.iter().find(|m| m.name == "root").unwrap());

    Ok(())
}

fn do_math(left: i64, operator: char, right: i64) -> i64{
    return match operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => panic!("rekt")
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    name: String,
    number: Option<i64>,
    left: Option<String>,
    right: Option<String>,
    operator: Option<char>,
}
impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        let mut monkey = Self {
            name: split[0][..split[0].len() - 1].to_string(),
            number: None,
            left: None,
            right: None,
            operator: None,
        };
        if split.len() == 2 {
            monkey.number = Some(split[1].parse().unwrap());
        } else {
            monkey.left = Some(split[1].to_string());
            monkey.operator = Some(split[2].chars().next().unwrap());
            monkey.right = Some(split[3].to_string());
        }
        Ok(monkey)
    }
}
