use std::fs;

const SUM: i32 = 2020;

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let entries = get_entries();
    if let Some((entry1, entry2)) = entries_equaling_sum(&entries[..], SUM) {
        print!("Day 1 - A: {}", entry1 * entry2);
    }
}

fn part_two() {
    let entries = get_entries();

    for (i, entry1) in entries.iter().enumerate() {
        let sublist = &entries[i..];
        let remainder = SUM - entry1;

        if let Some((entry2, entry3)) = entries_equaling_sum(sublist, remainder) {
            println!(", B: {}", entry1 * entry2 * entry3);
        }
    }
}

fn entries_equaling_sum(entries: &[i32], sum: i32) -> Option<(i32, i32)> {
    for (i, entry) in entries.iter().enumerate() {
        let remainder = sum - entry;
        let sublist = &entries[i..];
        if let Some(other_entry) = sublist.iter().filter(|&entry| *entry == remainder).next() {
            return Some((*entry, *other_entry));
        }
    }
    None
}

fn get_entries() -> Vec<i32> {
    fs::read_to_string("data/day1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
