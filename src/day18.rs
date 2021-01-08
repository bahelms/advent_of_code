use std::fs;

// enum Op {
//     Mul(Box<Op>, Box<Op>),
//     Add(Box<Op>, Box<Op>),
//     Num(u32),
// }

// impl Op {
//     fn exec(&self) -> u32 {
//         match self {
//             Self::Mul(lhs, rhs) => lhs.exec() * rhs.exec(),
//             Self::Add(lhs, rhs) => lhs.exec() + rhs.exec(),
//             Self::Num(n) => *n,
//         }
//     }
// }

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let sum: u64 = get_input()
        .iter()
        .map(|line| {
            let input = parse(line);
            calculate(&input[..])
        })
        .sum();
    print!("Day 18 - A: {:?}", sum);
}

fn part_two() {
    println!(" - B: {:?}", 0);
}

fn get_input() -> Vec<String> {
    fs::read_to_string("data/day18.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse(input: &String) -> Vec<char> {
    input.chars().filter(|&ch| ch != ' ').collect()
}

fn calculate(chars: &[char]) -> u64 {
    let mut lhs: u64 = 0;
    let mut pointer = 0;

    while pointer < chars.len() {
        let mut seeker = pointer + 1;
        let ch = chars[pointer];
        match ch.to_digit(10) {
            Some(num) => {
                lhs = num as u64;
                pointer += 1;
            }
            None => {
                // find lhs
                if ch == '(' {
                    let mut groups = 1;
                    while groups > 0 {
                        match chars[seeker] {
                            '(' => groups += 1,
                            ')' => groups -= 1,
                            _ => {}
                        }
                        seeker += 1;
                    }
                    lhs = calculate(&chars[pointer + 1..seeker - 1]);
                    pointer = seeker;
                    continue;
                }

                // find rhs
                let next_char = chars[seeker];
                let rhs = match next_char.to_digit(10) {
                    Some(num) => num as u64,
                    None => match next_char {
                        '(' => {
                            let mut groups = 1;
                            while groups > 0 {
                                seeker += 1;
                                match chars[seeker] {
                                    '(' => groups += 1,
                                    ')' => groups -= 1,
                                    _ => {}
                                }
                            }
                            calculate(&chars[pointer + 2..seeker])
                        }
                        _ => panic!("no rhs next char match"),
                    },
                };

                match ch {
                    '+' => lhs += rhs,
                    '*' => lhs *= rhs,
                    _ => panic!("operator is wrong"),
                }
                pointer = seeker + 1;
            }
        }
    }
    lhs
}

// fn group_by_precedence(chars: Vec<char>) -> Vec<char> {}

#[cfg(test)]
mod tests {
    use super::{calculate, parse};

    // #[test]
    // fn group_by_precedence() {
    //     let input = group_by_precedence(parse(&"2 * 3 + 4".to_string()));
    //     assert_eq!(input, vec!['(', '2', '*', '(', '3', '+', '4', ')', ')']);
    //     assert_eq!(calculate(&input[..]), 14);
    // }

    #[test]
    fn calculate_root_group() {
        let input = parse(&"2 + 3 * 4".to_string());
        assert_eq!(calculate(&input[..]), 20);
    }

    #[test]
    fn calculate_with_starting_nested_group() {
        let input = vec!['(', '2', '*', '3', ')', '+', '(', '4', '*', '5', ')'];
        assert_eq!(calculate(&input[..]), 26);
    }

    #[test]
    fn calculate_with_many_nested_groups() {
        let input = vec![
            '2', '*', '3', '+', '(', '4', '*', '(', '5', '+', '8', ')', ')',
        ];
        assert_eq!(calculate(&input[..]), 58);
    }

    #[test]
    fn calculate_big_one() {
        let input = parse(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string());
        assert_eq!(calculate(&input[..]), 13632);
    }
}
