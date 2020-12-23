use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Range,
};

struct Rule {
    name: String,
    first_range: Range<i32>,
    second_range: Range<i32>,
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let notes = get_notes();
    let rules = extract_rules(&notes[0]);
    let nearby_tickets = extract_tickets(&notes[2]);

    print!(
        "Day 16 - A: {:?}",
        scanning_error_rate(&rules, &nearby_tickets)
    );
}

fn part_two() {
    let notes = get_notes();
    let rules = extract_rules(&notes[0]);
    let my_ticket = extract_my_ticket(&notes[1]);
    let nearby_tickets = extract_tickets(&notes[2]);
    let prepared_tickets = transpose_tickets(&remove_invalid_tickets(&rules, &nearby_tickets));

    let map = map_rules_to_columns(&rules, prepared_tickets);
    let rule_cols = reduce_to_unique(map);
    let answer: i64 = rule_cols
        .iter()
        .filter(|(rule, _)| rule.starts_with("departure"))
        .map(|(_, col)| my_ticket[*col as usize] as i64)
        .fold(1, |product, n| product * n);
    println!(" - B: {:?}", answer);
}

fn remove_invalid_tickets(rules: &Vec<Rule>, tickets: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let valid_numbers = valid_numbers(&rules);
    let mut valid_tickets = Vec::new();
    for ticket in tickets {
        let mut valid = true;
        for num in ticket {
            if !valid_numbers.contains(&num) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_tickets.push(ticket.to_owned());
        }
    }
    valid_tickets
}

fn transpose_tickets(tickets: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed = Vec::new();
    for _ in 0..tickets[0].len() {
        transposed.push(Vec::new());
    }
    for ticket in tickets {
        for (i, &num) in ticket.iter().enumerate() {
            transposed[i].push(num);
        }
    }
    transposed
}

fn extract_my_ticket(ticket_string: &str) -> Vec<i32> {
    ticket_string.trim().split(":\n").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn extract_rules(rules_string: &str) -> Vec<Rule> {
    let rule_regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    rules_string
        .split("\n")
        .map(|line| {
            let matches = rule_regex.captures(&line).unwrap();
            Rule {
                name: matches.get(1).unwrap().as_str().to_string(),
                first_range: matches.get(2).unwrap().as_str().parse().unwrap()
                    ..matches.get(3).unwrap().as_str().parse::<i32>().unwrap() + 1,
                second_range: matches.get(4).unwrap().as_str().parse().unwrap()
                    ..matches.get(5).unwrap().as_str().parse::<i32>().unwrap() + 1,
            }
        })
        .collect()
}

fn extract_tickets(tickets_string: &str) -> Vec<Vec<i32>> {
    let mut tickets = Vec::new();
    let nearby_tickets: Vec<&str> = tickets_string.trim().split("\n").collect();

    for data in &nearby_tickets[1..] {
        tickets.push(data.split(",").map(|n| n.parse().unwrap()).collect());
    }
    tickets
}

fn scanning_error_rate(rules: &Vec<Rule>, tickets: &Vec<Vec<i32>>) -> i32 {
    let valid_numbers = valid_numbers(&rules);
    let mut rate = 0;
    for ticket in tickets {
        for num in ticket {
            if !valid_numbers.contains(num) {
                rate += num;
                break;
            }
        }
    }
    rate
}

fn valid_numbers(rules: &Vec<Rule>) -> HashSet<i32> {
    let mut nums = HashSet::new();
    for rule in rules {
        for num in rule.first_range.start..rule.first_range.end {
            nums.insert(num);
        }
        for num in rule.second_range.start..rule.second_range.end {
            nums.insert(num);
        }
    }
    nums
}

fn map_rules_to_columns(rules: &Vec<Rule>, values: Vec<Vec<i32>>) -> HashMap<String, HashSet<i32>> {
    let mut map = HashMap::new();
    for rule in rules {
        for (i, column) in values.iter().enumerate() {
            let mut not_in_range = false;
            for num in column {
                if !rule.first_range.contains(num) && !rule.second_range.contains(num) {
                    not_in_range = true;
                    break;
                }
            }
            if !not_in_range {
                map.entry(rule.name.to_string())
                    .or_insert(HashSet::new())
                    .insert(i as i32);
            }
        }
    }
    map
}

fn reduce_to_unique(map: HashMap<String, HashSet<i32>>) -> HashMap<String, i32> {
    let mut cols_found = HashSet::new();
    let mut reduced = HashMap::new();
    let mut done = false;

    while !done {
        for (rule, cols) in map.iter() {
            let difference = cols.difference(&cols_found).cloned().collect::<Vec<i32>>();

            if cols.len() == 1 {
                let col = cols.iter().cloned().collect::<Vec<i32>>()[0];
                reduced.insert(rule.to_string(), col);
                cols_found.insert(col);
            } else if difference.len() == 1 {
                reduced.insert(rule.to_string(), difference[0]);
                cols_found.insert(difference[0]);
            }
        }
        done = cols_found.len() == map.keys().len();
    }

    reduced
}

fn get_notes() -> Vec<String> {
    fs::read_to_string("data/day16.txt")
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Rule;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn reduce_to_unique_works() {
        let mut map = HashMap::new();
        map.insert(
            "A".to_string(),
            [1, 2].iter().cloned().collect::<HashSet<i32>>(),
        );
        map.insert(
            "B".to_string(),
            [0, 1, 2].iter().cloned().collect::<HashSet<i32>>(),
        );
        map.insert(
            "C".to_string(),
            [2].iter().cloned().collect::<HashSet<i32>>(),
        );
        let unique_map = super::reduce_to_unique(map);
        assert_eq!(unique_map.get("A").unwrap(), &1);
        assert_eq!(unique_map.get("B").unwrap(), &0);
        assert_eq!(unique_map.get("C").unwrap(), &2);
    }

    #[test]
    fn map_rules_to_columns_works() {
        let rules = vec![
            Rule {
                name: "A".to_string(),
                first_range: 0..2,
                second_range: 4..20,
            },
            Rule {
                name: "B".to_string(),
                first_range: 0..6,
                second_range: 8..20,
            },
            Rule {
                name: "C".to_string(),
                first_range: 0..14,
                second_range: 16..20,
            },
        ];
        let tickets = vec![vec![3, 15, 5], vec![9, 1, 14], vec![18, 5, 9]];
        let map = super::map_rules_to_columns(&rules, tickets);
        assert_eq!(
            map.get("A").unwrap(),
            &[1, 2].iter().cloned().collect::<HashSet<i32>>()
        );
        assert_eq!(
            map.get("B").unwrap(),
            &[0, 1, 2].iter().cloned().collect::<HashSet<i32>>()
        );
        assert_eq!(
            map.get("C").unwrap(),
            &[2].iter().cloned().collect::<HashSet<i32>>()
        );
    }

    #[test]
    fn scanning_error_rate_works() {
        let rules = vec![
            Rule {
                name: "A".to_string(),
                first_range: 1..4,
                second_range: 5..8,
            },
            Rule {
                name: "B".to_string(),
                first_range: 6..12,
                second_range: 33..44,
            },
            Rule {
                name: "C".to_string(),
                first_range: 13..41,
                second_range: 45..51,
            },
        ];
        let tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];
        let rate = super::scanning_error_rate(&rules, &tickets);
        assert_eq!(rate, 71);
    }
}
