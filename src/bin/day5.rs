use std::fs::read_to_string;
use anyhow::Result;

fn main() -> Result<()> {
    let binding = read_to_string("data/day5_personal.txt")?;
    let (supply_stacks, rearrangement_procedure) = binding.split_once("\n\r").unwrap();

    let mut stacks_part1: Vec<SupplyStack> = parse_supply_stacks(supply_stacks);
    let mut stacks_part2: Vec<SupplyStack> = parse_supply_stacks(supply_stacks);

    for rearrangement in rearrangement_procedure.trim().lines() {
        let rearrangement_orders = rearrangement.split_whitespace().collect::<Vec<&str>>();
        let quantity = rearrangement_orders.get(1).unwrap().parse::<usize>().unwrap();
        let stack_from_index = rearrangement_orders.get(3).unwrap().parse::<usize>().unwrap();
        let stack_to_index = rearrangement_orders.get(5).unwrap().parse::<usize>().unwrap();
        // println!("{} {} {}", quantity, stack_from_index, stack_to_index);


        // part 1
        for _n_ in 1..=quantity {
            let to_move_value = stacks_part1.iter_mut().nth(stack_from_index - 1).unwrap().pop_item();
            stacks_part1.iter_mut().nth(stack_to_index - 1).unwrap().push_item(to_move_value);
        }

        // part 2
        let to_move_values = stacks_part2.iter_mut().nth(stack_from_index - 1).unwrap().drain_items(quantity);
        for to_move_value in &to_move_values {
            stacks_part2.iter_mut().nth(stack_to_index - 1).unwrap().push_item(to_move_value.clone());
        }
    }
    let mut result_part1 = "".to_owned();
    for stack in &stacks_part1 {
        let last = stack.stack.last();
        if last.is_some() {
            result_part1.push_str(last.unwrap());
        }
    }
    println!("Result (part 1) is: {}", result_part1);

    let mut result_part2 = "".to_owned();
    for stack in &stacks_part2 {
        let last = stack.stack.last();
        if last.is_some() {
            result_part2.push_str(last.unwrap());
        }
    }
    println!("Result (part 2) is: {}", result_part2);
    Ok(())
}

fn parse_supply_stacks(supply_stacks: &str) -> Vec<SupplyStack> {
    let mut stacks: Vec<SupplyStack> = Vec::new();

    for (index, stack_line) in supply_stacks.lines().rev().enumerate() {
        if index > 0 {
            for stack in stacks.iter_mut() {
                let value = stack_line.chars().nth(stack.index);
                if value.is_some() && value.unwrap() != ' ' {
                    stack.push_item(String::from(value.unwrap()));
                }
            }
        } else {
            for n in 1..=stack_line.split_whitespace().count() {
                stacks.push(SupplyStack{stack: Vec::new(), index: stack_line.find(n.to_string().as_str()).unwrap()});
            }
        }
    }

    return stacks;
}

struct SupplyStack {
    stack: Vec<String>,
    index: usize,
}
impl SupplyStack {
    fn push_item (&mut self, item: String) {
        self.stack.push(item);
    }

    fn pop_item (&mut self) -> String {
        return self.stack.pop().unwrap();
    }

    fn drain_items(&mut self, quantity: usize) -> Vec<String>{
        return self.stack.drain(self.stack.len()-quantity..=self.stack.len()-1).collect();
    }

    // fn print(&self) {
    //     println!("supply stack:");
    //     for item in &self.stack {
    //         println!("{} ", item);
    //     }
    //     println!("---");
    // }
}