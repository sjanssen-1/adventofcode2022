use std::fs::read_to_string;
use anyhow::Result;

fn main() -> Result<()>{
    let datastream_buffer = read_to_string("data/day6.txt")?;

    println!("start-of-packet after {} characters", find_start_marker(&datastream_buffer, 4)?);
    println!("start-of-message after {} characters", find_start_marker(&datastream_buffer, 14)?);

    Ok(())
}

fn find_start_marker(datastream_buffer: &String, distinct: usize) -> Result<usize> {
    for (index, character ) in datastream_buffer.chars().enumerate().skip(distinct-1) {
        let datastream_part = &datastream_buffer[index-(distinct-1)..=index];
        println!("handling subroutine {}", datastream_part);

        let relevant_characters = datastream_part.chars().collect::<Vec<char>>();
        let mut relevant_characters_duplicate = relevant_characters.clone();
        relevant_characters_duplicate.sort();
        relevant_characters_duplicate.dedup();

        if relevant_characters.len() == relevant_characters_duplicate.len() {
            return Ok(index+1);
        }
    }
    panic!("Nothing found")
}