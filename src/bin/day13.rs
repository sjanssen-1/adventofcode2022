use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::read_to_string;
use std::str::FromStr;
use anyhow::{Error, Result};
use json::JsonValue;

fn main() -> Result<()>{
    let input = read_to_string("data/day13_personal.txt")?;

    let mut all: Vec<Packet> = Vec::new();

    let mut sum = 0;
    for (index, pair) in input.split("\n\r").enumerate() {
        let (left, right) = pair.split_once("\r").unwrap();
        let left_packet: Packet = left.parse()?;
        let right_packet: Packet = right.parse()?;

        all.push(left_packet.clone());
        all.push(right_packet.clone());

        if &left_packet < &right_packet {
            sum += index + 1
        }
    }
    println!("Sum is (part 1): {}", sum);


    all.push(Packet{contents: json::parse("[[2]]").unwrap()});
    all.push(Packet{contents: json::parse("[[6]]").unwrap()});
    all.sort();

    let divider_packet_2 = Packet{contents: json::parse("[[2]]").unwrap()};
    let divider_packet_6 = Packet{contents: json::parse("[[6]]").unwrap()};

    let divider_packet_2_index = all.iter().enumerate().find(|x| x.1 == &divider_packet_2).unwrap().0;
    let divider_packet_6_index = all.iter().enumerate().find(|x| x.1 == &divider_packet_6).unwrap().0;
    println!("distress signal (part 2): {}", (divider_packet_2_index + 1) * (divider_packet_6_index + 1));

    Ok(())
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Packet {
    contents: JsonValue,
}
impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {contents: json::parse(s).unwrap()})
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.contents.is_number() && other.contents.is_number() {
            let left = self.contents.as_usize().unwrap();
            let right = other.contents.as_usize().unwrap();
            return left.cmp(&right);
        } else if self.contents.is_array() && other.contents.is_array() {
            let left_size = self.contents.len();
            let right_size = other.contents.len();

            for n in 0..self.contents.len() {
                if n == left_size || n == right_size {
                    break;
                }
                let left = Packet { contents: self.contents[n].clone() };
                let right = Packet { contents: other.contents[n].clone() };
                let compare = left.cmp(&right);
                if compare != Equal {
                    return compare;
                }
            }
            if left_size < right_size {
                Less
            } else if left_size > right_size {
                Greater
            } else {
                Equal
            }
        } else {
            if self.contents.is_array() {
                let mut right_array = Vec::new();
                right_array.push(other.contents.clone());
                let right = Packet { contents: JsonValue::Array(right_array) };
                self.cmp(&right)
            } else {
                let mut left_array = Vec::new();
                left_array.push(self.contents.clone());
                let left = Packet { contents: JsonValue::Array(left_array) };
                left.cmp(other)
            }
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::Packet;

    #[test]
    fn test() {
        let number1 = Packet{contents: json::parse("1").unwrap()};
        let number2 = Packet{contents: json::parse("2").unwrap()};
        assert!(number1 < number2);
        assert!(number2 > number1);
        assert_eq!(number1, number1);

        let array_1 = Packet{contents: json::parse("[1,1,3,1,1]").unwrap()};
        let array_2 = Packet{contents: json::parse("[1,1,5,1,1]").unwrap()};
        assert!(array_1 < array_2);
        assert!(array_2 > array_1);
        assert_eq!(array_1, array_1);

        assert!(number1 < array_1);
    }
}