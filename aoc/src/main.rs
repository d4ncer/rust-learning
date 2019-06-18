use std::fs;

const INPUT: &str = include_str!("../input/day1.txt");

fn parse_input() {}

fn read_input_file() -> i32 {
    let contents = fs::read_to_string("day1.txt").expect("Failed to read file.");
    let v: Vec<&str> = contents.split("\n").collect();
    let v: Vec<i32> = v
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let v: i32 = v.iter().sum();
    println!("v is now {}", v);
    v
}

fn main() {
    read_input_file();
}
