extern crate adventofcode2022;

use adventofcode2022::util::read_file;

fn main() {
    let rucksacks = read_file("data/day3_personal.txt");

    let inspection = inspect_rucksack(&rucksacks);
    println!("rucksack sum of item priorities: {}", inspection.0);
    println!("rucksack sum of badge priorities: {}", inspection.1);
}

fn inspect_rucksack (rucksacks: &Vec<String>) -> (i32, i32) {
    let mut item_priority_sum = 0;
    let mut badge_priority_sum = 0;

    let mut badge_group:  Vec<String> = Vec::new();
    for rucksack in rucksacks {
        item_priority_sum += prioritize_item_rearrangement(rucksack.clone());

        badge_group.push((*rucksack).parse().unwrap());
        if  badge_group.len() == 3 {
            badge_priority_sum += identify_group_badge(&badge_group);
            badge_group.clear();
        }

    }

    return (item_priority_sum, badge_priority_sum);
}

fn prioritize_item_rearrangement(rucksack: String) -> i32 {
    let compartments = rucksack.split_at(rucksack.len() / 2);
    for item in compartments.0.chars() {
        if compartments.1.contains(item) {
            return get_priority_value(item);
        }
    }
    return 0; // no doubles
}

fn identify_group_badge(rucksack_group: &Vec<String>) -> i32 {
    assert_eq!(rucksack_group.len(), 3);

    let rucksack = rucksack_group.get(0).unwrap().clone();
    for item in rucksack.chars() {
        if rucksack_group.get(1).unwrap().contains(item)
            && rucksack_group.get(2).unwrap().contains(item) {
            return get_priority_value(item);
        }
    }

    return 0; // no badge found
}

fn get_priority_value (item: char) -> i32 {
    return if item as i32 >= 97 {
        item as i32 - 97 + 1 // a-z
    }
    else {
        item as i32 - 65 + 26 + 1 // A-Z
    }
}