use std::fs::read_to_string;
use anyhow::Result;

fn main() -> Result<()>{
    let file = read_to_string("data/day20_personal.txt")?;


    println!("sum (part 1): {}", mix(&file, 1, 1));
    println!("sum (part 2): {}", mix(&file, 811589153, 10));
    Ok(())
}

fn mix(file: &str, decryption_key: i64, times: i64) -> i64 {
    let mut numbers: Vec<(usize, i64)> = Vec::new();
    for (idx, number) in file.lines().enumerate() {
        numbers.push((idx, number.parse::<i64>().unwrap() * decryption_key))
    }
    let length = numbers.len();
    for _ in 0..times {
        for i in 0..length {
            let current_idx = numbers.iter().position(|&n| n.0 == i).unwrap();
            let removed = numbers.remove(current_idx);
            let new_index: i64 = (removed.1 + current_idx as i64).rem_euclid(length as i64 - 1);
            numbers.insert(new_index as usize, removed);
        }
    }

    let index = numbers.iter().position(|&(_, x)| x == 0).unwrap();
    [1000, 2000, 3000].iter().fold(0, |acc, i| acc + numbers[(i + index) % length].1)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let mut numbers: Vec<(usize, i64)> = vec![(0,1), (1,2), (2,-3), (3, 3), (4, -2), (5, 0), (6,4)];

        // move 1
        let removed = numbers.remove(0);
        numbers.insert(1, removed);

        let removed = numbers.remove(0);
        numbers.insert((0 + removed.1) as usizee, removed);

        println!("{:?}", numbers);

        numbers.rotate_left(1 + 1);
        let removed = numbers.remove(numbers.len());

        // numbers.rotate_right(1);
        println!("{:?}", numbers);
    }
}