use std::fs;

const SUM: i32 = 2020;

#[allow(dead_code)]
pub fn part_one() {
    let entries = get_entries();
    if let Some((entry1, entry2)) = entry_equaling_sum(&entries[..], SUM) {
        println!("Day 1A - Answer: {}", entry1 * entry2);
    }
}

pub fn part_two() {
    let entries = get_entries();

    for (i, entry1) in entries.iter().enumerate() {
        let sublist = &entries[i..];
        let remainder = SUM - entry1;

        if let Some((entry2, entry3)) = entry_equaling_sum(sublist, remainder) {
            println!("Day 1B - Answer: {}", entry1 * entry2 * entry3);
        }
    }
}

fn entry_equaling_sum(entries: &[i32], sum: i32) -> Option<(i32, i32)> {
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
    let input = fs::read_to_string("data/day1a.txt").unwrap();
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
