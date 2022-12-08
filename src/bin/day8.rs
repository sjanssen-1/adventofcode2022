use std::fs::read_to_string;
use anyhow::Result;

fn main() -> Result<()>{
    let forest_layout = read_to_string("data/day8_personal.txt")?;
    let forest_width = &forest_layout.lines().next().unwrap().len();
    let forest_length = &forest_layout.lines().count();
    println!("{} {}", forest_width, forest_length);

    let mut forest = vec![vec![0u8; *forest_width]; *forest_length];
    for (tree_line_number ,tree_line) in forest_layout.lines().enumerate() {
        for (tree_number, tree_height) in tree_line.chars().enumerate() {
            forest[tree_line_number][tree_number] = tree_height.to_string().parse::<u8>()?;
        }
    }
    let outside_visible = (forest_width*2)+(forest_length*2)-4;
    let mut inside_visible: usize = 0;
    let mut best_scenic_score: i32 = 0;
    for y in 1..*forest_length-1 {
        for x in 1..*forest_width-1 {
            if is_visible_less_ugly_but_still_ugly(&forest, x, y, forest[y][x], *forest_width, *forest_length) {
                inside_visible+=1;
            }
            let tree_scenic_score = calculate_scenic_score(&forest, x, y, forest[y][x], *forest_width, *forest_length);
            if tree_scenic_score > best_scenic_score {
                best_scenic_score = tree_scenic_score;
            }
        }
    }
    println!("{:?}", forest);
    println!("{}", is_visible2(&forest, 3, 3, forest[3][3], *forest_width, *forest_length));
    println!("{} {} {} {}", outside_visible, inside_visible, outside_visible + inside_visible, best_scenic_score);
    Ok(())
}

fn is_visible_ugly(forest: &Vec<Vec<u8>>, tree_x: usize, tree_y: usize, tree_height: u8, forest_width: usize, forest_length: usize) -> bool {
    let mut is_visible_west = true;
    let mut is_visible_east = true;
    let mut is_visible_north = true;
    let mut is_visible_south = true;

    // check west
    for x in (0..tree_x).rev() {
        // println!("Checking west index {},{} has value {}", x, tree_y, forest[tree_y][x]);
        if forest[tree_y][x] >= tree_height {
            is_visible_west = false;
            break;
        }
    }
    if !is_visible_west {
        // check east
        for x in tree_x+1..forest_width {
            // println!("Checking east index {},{} has value {}", x, tree_y, forest[tree_y][x]);
            if forest[tree_y][x] >= tree_height {
                is_visible_east = false;
                break;
            }
        }
        if !is_visible_east {
            // check north
            for y in (0..tree_y).rev() {
                // println!("Checking north index {},{} has value {}", tree_x, y, forest[y][tree_x]);
                if forest[y][tree_x] >= tree_height {
                    is_visible_north = false;
                    break;
                }
            }
            if !is_visible_north {
                // check south
                for y in tree_y+1..forest_length {
                    // println!("Checking south index {},{} has value {}", tree_x, y, forest[y][tree_x]);
                    if forest[y][tree_x] >= tree_height {
                        is_visible_south = false;
                        break;
                    }
                }
            }
        }
    }
    return is_visible_west || is_visible_east || is_visible_north || is_visible_south;
}

fn is_visible_less_ugly_but_still_ugly(forest: &Vec<Vec<u8>>, tree_x: usize, tree_y: usize, tree_height: u8, forest_width: usize, forest_length: usize) -> bool {
    let mut is_visible_horizontal= true;
    for x in 0..=forest_width {
        if x == tree_x || x == forest_width {
            if is_visible_horizontal {
                return true;
            }
            is_visible_horizontal = true; // reset
            continue;
        }
        is_visible_horizontal &= forest[tree_y][x] < tree_height;
    }

    let mut is_visible_vertical= true;
    for y in 0..=forest_length {
        if y == tree_y || y == forest_length {
            if is_visible_vertical {
                return true;
            }
            is_visible_vertical = true; // reset
            continue;
        }
        is_visible_vertical &= forest[y][tree_x] < tree_height;
    }
    return false;
}

fn calculate_scenic_score(forest: &Vec<Vec<u8>>, tree_x: usize, tree_y: usize, tree_height: u8, forest_width: usize, forest_length: usize) -> i32 {
    let mut trees_west = 0;
    let mut trees_east = 0;
    let mut trees_north = 0;
    let mut trees_south = 0;

    // check west
    for x in (0..tree_x).rev() {
        // println!("Checking west index {},{} has value {}", x, tree_y, forest[tree_y][x]);
        trees_west += 1;
        if forest[tree_y][x] >= tree_height {
            break;
        }
    }
    // check east
    for x in tree_x+1..forest_width {
        // println!("Checking east index {},{} has value {}", x, tree_y, forest[tree_y][x]);
        trees_east += 1;
        if forest[tree_y][x] >= tree_height {
            break;
        }
    }
    // check north
    for y in (0..tree_y).rev() {
        // println!("Checking north index {},{} has value {}", tree_x, y, forest[y][tree_x]);
        trees_north += 1;
        if forest[y][tree_x] >= tree_height {
            break;
        }
    }
    // check south
    for y in tree_y+1..forest_length {
        // println!("Checking south index {},{} has value {}", tree_x, y, forest[y][tree_x]);
        trees_south += 1;
        if forest[y][tree_x] >= tree_height {
            break;
        }
    }
    // println!("{},{} has score {} {} {} {}", tree_x, tree_y, trees_west, trees_east, trees_north, trees_south);
    return trees_west * trees_east * trees_north * trees_south;
}