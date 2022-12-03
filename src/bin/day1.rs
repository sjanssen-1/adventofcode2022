extern crate adventofcode2022;

use adventofcode2022::util;

const FILE_PATH: &str = "data/day1_personal.txt";

fn main() {
    let max_calories: &mut [i32; 3] = &mut [0; 3];
    let meals = util::read_file(FILE_PATH);

    let mut current_elf_calories = 0;

    for meal in meals {
        if meal.is_empty() {
            populate_max_calories(current_elf_calories, max_calories);
            current_elf_calories = 0;
            continue;
        }

        let calories_in_meal = meal.parse::<i32>().unwrap();
        current_elf_calories += calories_in_meal;
    }
    println!("Biggest elf carries {} calories (part 1)", max_calories[2]);
    println!("Top 3 elves carry {} calories (part 2)", calculate_max_calories(max_calories));
}

fn populate_max_calories(accumulated_calories: i32, max_calories: &mut [i32; 3]) {
    for calories in max_calories.iter_mut() {
        if accumulated_calories > *calories {
            *calories = accumulated_calories;
            break;
        }
    }
    max_calories.sort();
}

fn calculate_max_calories(max_calories: &mut [i32; 3]) -> i32 {
    let mut total_calories = 0;
    for calories in max_calories.iter() {
        total_calories += *calories;
    }
    return total_calories;
}