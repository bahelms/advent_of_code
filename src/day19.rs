use regex::Regex;
use std::{collections::HashMap, fs};

type Rules = HashMap<String, String>;

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let (rules, messages) = get_input();
    let regex = compile_regex("0", &rules, true);
    let count = total_valid_messages(messages, regex);
    print!("Day 19 - A: {:?}", count);
}

fn part_two() {
    let (rules, messages) = get_input();
    let regex_42 = compile_regex("42", &rules, false);
    let regex_31 = compile_regex("31", &rules, false);
    let matches = count_matches(messages, regex_42, regex_31);
    println!(" - B: {:?}", matches);
}

fn get_input() -> (Rules, Vec<String>) {
    let file_strs = fs::read_to_string("data/day19.txt").unwrap();
    let input: Vec<&str> = file_strs.split("\n\n").collect();
    (
        parse_rules(input[0]),
        input[1].lines().map(String::from).collect(),
    )
}

fn total_valid_messages(messages: Vec<String>, regex: Regex) -> i32 {
    let mut count = 0;
    for msg in messages {
        if regex.is_match(&msg) {
            count += 1;
        }
    }
    count
}

fn count_matches(messages: Vec<String>, regex_42: Regex, regex_31: Regex) -> i32 {
    let mut count = 0;
    // println!("42 {}", regex_42);
    // println!("31 {}", regex_31);
    for msg in messages {
        let mut start = 0;

        let mut count_42 = 0;
        loop {
            match regex_42.find_at(&msg, start) {
                Some(reg_match) => {
                    // println!(
                    //     "match42 {} - {}:{} - slice {:?}",
                    //     msg,
                    //     start,
                    //     reg_match.end(),
                    //     &msg.chars().collect::<Vec<char>>()[start..reg_match.end()]
                    // );
                    count_42 += 1;
                    start = reg_match.end();
                }
                None => break,
            }
        }

        let mut count_31 = 0;
        loop {
            match regex_31.find_at(&msg, start) {
                Some(reg_match) => {
                    // println!(
                    //     "match31 {} - {}:{} - slice {:?}",
                    //     msg,
                    //     start,
                    //     reg_match.end(),
                    //     &msg.chars().collect::<Vec<char>>()[start..reg_match.end()]
                    // );
                    count_31 += 1;
                    start = reg_match.end();
                }
                None => break,
            }
        }

        // println!("final start {} - len {}", start, msg.len());
        if start == msg.len() && count_42 > count_31 && count_31 > 0 {
            count += 1;
        }
    }
    count
}

fn parse_rules(input: &str) -> Rules {
    input.lines().fold(HashMap::new(), |mut rules, line| {
        let rule_strs: Vec<&str> = line.split(":").collect();
        rules.insert(rule_strs[0].to_string(), rule_strs[1].trim().to_string());
        rules
    })
}

fn compile_regex(rule: &str, rules: &Rules, anchors: bool) -> Regex {
    let mut regex_string = construct_regex(rule.to_string(), &rules);
    if anchors {
        regex_string = format!("^{}$", regex_string);
    }
    Regex::new(&regex_string).unwrap()
}

fn construct_regex(rule: String, rules: &Rules) -> String {
    if rule == "|" {
        return "|".to_string();
    }

    let pattern = rules.get(&rule).unwrap();
    if pattern.starts_with("\"") {
        return pattern.trim_matches('"').to_string();
    } else {
        let regex: String = pattern
            .split(" ")
            .map(|subrule| construct_regex(subrule.to_string(), rules))
            .collect();
        format!("({})", regex)
    }
}

#[cfg(test)]
mod tests {
    use super::{compile_regex, construct_regex, count_matches, parse_rules, total_valid_messages};

    #[test]
    fn matching() {
        let regex = regex::Regex::new("(a(a|b))").unwrap();
        let reg_match = regex.find_at("ab", 0).unwrap();
        assert_eq!(reg_match.end(), 0);
    }

    #[test]
    fn part2_input_works() {
        let rules = parse_rules(
            "42: 9 14 | 10 1\n\
            9: 14 27 | 1 26\n\
            10: 23 14 | 28 1\n\
            1: \"a\"\n\
            5: 1 14 | 15 1\n\
            19: 14 1 | 14 14\n\
            12: 24 14 | 19 1\n\
            16: 15 1 | 14 14\n\
            31: 14 17 | 1 13\n\
            6: 14 14 | 1 14\n\
            2: 1 24 | 14 4\n\
            13: 14 3 | 1 12\n\
            15: 1 | 14\n\
            17: 14 2 | 1 7\n\
            23: 25 1 | 22 14\n\
            28: 16 1\n\
            4: 1 1\n\
            20: 14 14 | 1 15\n\
            3: 5 14 | 16 1\n\
            27: 1 6 | 14 18\n\
            14: \"b\"\n\
            21: 14 1 | 1 14\n\
            25: 1 1 | 1 14\n\
            22: 14 14\n\
            26: 14 22 | 1 20\n\
            18: 15 15\n\
            7: 14 5 | 1 21\n\
            24: 14 1\n\
            ",
        );
        let msgs = vec![
            "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
            "aabaaabaaa".to_string(),
            // "bbabbbbaabaabba".to_string(),
            // "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
            //
            // "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
            //
            // "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
            // "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
            // "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
            // "ababaaaaaabaaab".to_string(),
            // "ababaaaaabbbaba".to_string(),
            // "baabbaaaabbaaaababbaababb".to_string(),
            // "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
            // "aaaaabbaabaaaaababaa".to_string(),
            // "aaaabbaaaabbaaa".to_string(),
            // "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
            // "babaaabbbaaabaababbaabababaaab".to_string(),
            // "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
        ];
        let regex_42 = compile_regex("42", &rules, false);
        let regex_31 = compile_regex("31", &rules, false);
        let count = count_matches(msgs, regex_42, regex_31);
        assert_eq!(count, 12);
    }

    #[test]
    fn regex_matches_exactly() {
        let rules = parse_rules("0: 1 2 | 2 1\n1: \"a\"\n2: 1 3 | 3 1\n3: \"b\"");
        let regex = compile_regex("0", &rules, true);
        assert_eq!(regex.is_match("aab"), true);
        assert_eq!(regex.is_match("aba"), true);
        assert_eq!(regex.is_match("baa"), true);
        assert_eq!(regex.is_match("baaa"), false);
        assert_eq!(regex.is_match("aaab"), false);
    }

    #[test]
    fn construct_regex_works_with_ors() {
        let rules = parse_rules("0: 1 2 | 2 1\n1: \"a\"\n2: 1 3 | 3 1\n3: \"b\"");
        let regex = construct_regex("0".to_string(), &rules);
        assert_eq!(regex, "(a(ab|ba)|(ab|ba)a)");
    }

    #[test]
    fn construct_regex_works() {
        let rules = parse_rules("0: 1 2\n1: \"a\"\n2: \"b\"");
        let regex = construct_regex("0".to_string(), &rules);
        assert_eq!(regex, "(ab)");
    }
}
