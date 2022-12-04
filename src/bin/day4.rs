extern crate adventofcode2022;

use adventofcode2022::util::read_file;

#[derive(Clone, Copy)]
struct CleaningElf {
    start_id: i32,
    end_id: i32
}
impl CleaningElf {
    fn new(cleaning_range: &str) -> CleaningElf {
        let cleaning_ids: Vec<&str> = cleaning_range.split("-").collect();
        CleaningElf {
            start_id: cleaning_ids.get(0).unwrap().parse::<i32>().unwrap(),
            end_id: cleaning_ids.get(1).unwrap().parse::<i32>().unwrap()
        }
    }

    fn fully_overlaps(self, other_elf: CleaningElf) -> bool {
        self.start_id <= other_elf.start_id && self.end_id >= other_elf.end_id
    }

    fn partially_overlaps(self, other_elf: CleaningElf) -> bool {
        self.start_id <= other_elf.start_id && self.end_id >= other_elf.start_id
    }
}

fn main() {
    let assignment_pairs = read_file("data/day4_personal.txt");

    println!("{} pairs fully overlap", check_assignment_overlaps(&assignment_pairs).0);
    println!("{} pairs partially overlap", check_assignment_overlaps(&assignment_pairs).1);
}

fn check_assignment_overlaps(assignment_pairs: &Vec<String>) -> (i32, i32) {
    let mut full_overlaps = 0;
    let mut partial_overlaps = 0;

    for assignment_pair in assignment_pairs {
        let cleaning_ranges: Vec<&str> = assignment_pair.split(",").collect();
        let elf1 = CleaningElf::new(cleaning_ranges.get(0).unwrap());
        let elf2 = CleaningElf::new(cleaning_ranges.get(1).unwrap());
        if elf1.fully_overlaps(elf2) || elf2.fully_overlaps(elf1)
        {
            full_overlaps += 1;
            partial_overlaps += 1;
        } else if elf1.partially_overlaps(elf2) || elf2.partially_overlaps(elf1) {
            partial_overlaps += 1;
        }

    }
    return (full_overlaps, partial_overlaps);
}