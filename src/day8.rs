use std::fs;

pub fn execute() {
    part_one();
    // part_two();
}

fn part_one() {
    print!("Day 8 - A: {:?}", 0);
}

fn get_rules() -> Vec<String> {
    fs::read_to_string("data/day8.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
