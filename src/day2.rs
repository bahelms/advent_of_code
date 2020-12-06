use std::{fs, ops::Range};

#[derive(Debug)]
struct Policy {
    range: Range<i32>,
    character: char,
}

impl Policy {
    fn new(data: String) -> Self {
        let data: Vec<&str> = data.split(' ').collect();
        let range: Vec<&str> = data[0].split('-').collect();
        Self {
            range: (range[0].parse().unwrap()..range[1].parse::<i32>().unwrap() + 1),
            character: data[1].chars().nth(0).or_else(|| Some(' ')).unwrap(),
        }
    }
}

struct Record {
    policy: Policy,
    password: String,
}

impl Record {
    fn new(data: &str) -> Self {
        let data: Vec<String> = data.split(':').map(String::from).collect();
        Self {
            policy: Policy::new(data[0].to_owned()),
            password: data[1].trim().to_owned(),
        }
    }
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut count = 0;
    let records = get_password_records();
    for record in &records {
        let occurrence = occurrence_of_character(&record);
        if record.policy.range.contains(&occurrence) {
            count += 1;
        }
    }

    print!("Day 2 - A: {:?}", count);
}

fn part_two() {
    let mut count = 0;
    let records = get_password_records();
    for record in &records {
        let pos1 = record
            .password
            .chars()
            .nth((record.policy.range.start - 1) as usize)
            .unwrap();
        let pos2 = record
            .password
            .chars()
            .nth((record.policy.range.end - 2) as usize)
            .unwrap();
        let ch = record.policy.character;

        if pos1 == ch && pos2 != ch || pos1 != ch && pos2 == ch {
            count += 1;
        }
    }

    println!(", B: {:?}", count);
}

fn occurrence_of_character(record: &Record) -> i32 {
    record
        .password
        .chars()
        .filter(|&c| c == record.policy.character)
        .collect::<Vec<char>>()
        .len() as i32
}

fn get_password_records() -> Vec<Record> {
    fs::read_to_string("data/day2.txt")
        .unwrap()
        .lines()
        .map(Record::new)
        .collect()
}
