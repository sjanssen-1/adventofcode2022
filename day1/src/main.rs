use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const FILE_PATH: &str = "src/input.txt";

fn main() -> Result<(), Error> {
    let max_calories: &mut [i32; 3] = &mut [0; 3];
    let meals = read(FILE_PATH)?;

    let mut current_elf_calories = 0;

    for meal in meals {
        if meal.is_empty() {
            println!("Elf carried {} calories in total", current_elf_calories);
            populate_max_calories(current_elf_calories, max_calories);
            current_elf_calories = 0;
            continue;
        }

        println!("Calories of meal: {}", meal);
        let calories_in_meal = meal.parse::<i32>().unwrap();
        current_elf_calories += calories_in_meal;
    }
    println!("And the biggest amount of calories carried was: {}", calculate_max_calories(max_calories));
    Ok(())
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

fn read(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();
    for line in br.lines() {
        v.push(line?);
    }
    Ok(v)
}