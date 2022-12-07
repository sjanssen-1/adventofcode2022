extern crate adventofcode2022;

use std::fs::read_to_string;
use std::str::{FromStr, Lines};
use anyhow::{Error, Result};
use adventofcode2022::util::read_file;

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}
impl Directory {
    pub fn new(name: String) -> Self {
        Self { name, files: Vec::new(), directories: Vec::new() }
    }

    fn size(&self) -> i32 {
        *&self.files.iter().fold(0, |accumulator, file| accumulator + file.size)
            + *&self.directories.iter().fold(0, |accumulator, dir| accumulator + dir.size())
    }
}
impl FromStr for Directory {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let name = s.split_once(" ").expect("screwed").1;
        Ok(Directory::new(String::from(name)))
    }
}

struct File {
    size: i32,
    name: String,
}
impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (size, name) =  s.split_once(" ").expect("screwed");
        Ok(Self { size: size.parse()?, name: name.to_string() })
    }
}

fn main() -> Result<()> {
    let mut log = read_file("data/day7_personal.txt");

    log.remove(0); // fuck cd /
    let mut root_dir = Directory::new(String::from("/"));
    traverse_dir(&mut root_dir, &log, 0);
    println!("Sum of dirs below size 100000 (part1): {}", sum_below_100000_size_dirs(&root_dir));
    let fs_max = 70_000_000;
    let fs_needed = 30_000_000;
    let fs_used = fs_max - root_dir.size();
    let fs_to_free = fs_needed - fs_used;
    println!("Smallest dir to delete (part2): {}", find_directory_to_delete(&root_dir, fs_to_free));

    Ok(())
}

fn traverse_dir(dir: &mut Directory, log: &Vec<String>, start_idx: usize) -> usize {
    let mut idx: usize = start_idx;
    while idx < log.len() {
        let log_line = &log[idx];
        if log_line == "$ cd .." {
            return idx + 1;
        } else if log_line.starts_with("$ cd") {
            let split = log_line.split(" ").last().expect("screwed");
            idx = traverse_dir(dir.directories.iter_mut().find(|dir| dir.name == split).expect("screwed"), &log, idx+1);
        }
        else if log_line == "$ ls"{
            idx += 1;
        }
        else {
            add_contents(dir, log_line);
            idx += 1;
        }
    }
    idx
}

fn sum_below_100000_size_dirs(dir: &Directory) -> i32 {
    let mut sum = 0;
    let threshold = 100_000;
    if dir.size() < threshold {
        sum += dir.size();
    }
    for d in &dir.directories {
        sum += sum_below_100000_size_dirs(d);
    }
    sum
}

fn find_directory_to_delete(dir: &Directory, fs_to_free: i32) -> i32 {
    let mut smallest_dir_to_delete = dir.size();
    for d in &dir.directories {
        let d_size = d.size();
        if d_size >= fs_to_free && d_size < smallest_dir_to_delete {
            smallest_dir_to_delete = d_size;
        }
        let ds_size = find_directory_to_delete(d, fs_to_free);
        if ds_size >= fs_to_free && ds_size < smallest_dir_to_delete {
           smallest_dir_to_delete = ds_size;
        }
    }
    return smallest_dir_to_delete;
}

fn add_contents(dir: &mut Directory, contents: &str){
    if contents.starts_with("dir") {
        dir.directories.push(contents.parse::<Directory>().expect("screwed"));
    } else { // file
        dir.files.push(contents.parse::<File>().expect("screwed"));
    }
}