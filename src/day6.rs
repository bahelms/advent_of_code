use std::{collections::HashMap, fs};

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut count = 0;
    for group in get_groups() {
        let mut questions_answered = HashMap::new();
        for answers in group.lines() {
            for answer in answers.chars() {
                questions_answered.insert(answer, 1);
            }
        }
        count += questions_answered.values().sum::<i32>();
    }
    print!("Day 6 - A: {:?}", count);
}

fn part_two() {
    let mut total_count = 0;
    for group in get_groups() {
        total_count += common_answers_for_group(group);
    }
    println!(", B: {:?}", total_count);
}

fn common_answers_for_group(group: String) -> i32 {
    let mut total_count = 0;
    let mut questions_answered = HashMap::new();
    let answers = group.lines().map(String::from).collect::<Vec<String>>();
    for answers_for_person in &answers {
        for answer in answers_for_person.chars() {
            *questions_answered.entry(answer).or_insert(0) += 1;
        }
    }

    for count in questions_answered.values() {
        if count == &answers.len() {
            total_count += 1;
        }
    }
    total_count
}

fn get_groups() -> Vec<String> {
    fs::read_to_string("data/day6.txt")
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::common_answers_for_group;

    #[test]
    fn common_answers_for_group_works() {
        let group = "abcdefg\nxxdexxxx\nnnnntmqred".to_string();
        assert_eq!(common_answers_for_group(group), 2);
    }
}
