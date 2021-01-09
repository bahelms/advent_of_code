use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Int(char),
    Op(char),
    Eof,
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut tokens: Vec<Token> = input
            .chars()
            .filter(|&ch| ch != ' ')
            .map(|ch| match ch {
                '0'..='9' => Token::Int(ch),
                _ => Token::Op(ch),
            })
            .collect();
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }
    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let sum: u64 = get_input().iter().map(|line| calculate(line.clone())).sum();
    print!("Day 18 - A: {:?}", sum);
}

fn part_two() {
    let sum: u64 = get_input().iter().map(|line| calculate(line.clone())).sum();
    println!(" - B: {:?}", sum);
}

fn get_input() -> Vec<String> {
    fs::read_to_string("data/day18.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn calculate(input: String) -> u64 {
    let mut lexer = Lexer::new(input);
    evaluate(&mut lexer, 0)
}

fn evaluate(lexer: &mut Lexer, precedence: u8) -> u64 {
    let mut lhs = match lexer.next() {
        Token::Int(ch) => ch.to_digit(10).unwrap() as u64,
        Token::Op('(') => {
            let lhs = evaluate(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        if let Some((lbp, rbp)) = infix_binding_power(op) {
            if lbp < precedence {
                break;
            }
            lexer.next();
            let rhs = evaluate(lexer, rbp);
            match op {
                '+' => lhs += rhs,
                '*' => lhs *= rhs,
                _ => panic!("unknown operator: {}", op),
            }
            continue;
        }
        break;
    }
    lhs
}

fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    match op {
        '+' | '*' => Some((1, 2)),
        // part two
        // '*' => Some((1, 2)),
        // '+' => Some((3, 4)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn calculate_single_expression() {
        assert_eq!(calculate("2".to_string()), 2);
    }

    #[test]
    fn calculate_arith_expression() {
        assert_eq!(calculate("2 + 5".to_string()), 7);
    }

    #[test]
    fn calculate_deep_arith_expression() {
        assert_eq!(calculate("3 + 5 * 2 + 1".to_string()), 17);
    }

    #[test]
    fn calculate_first_grouped_xpression() {
        assert_eq!(calculate("(3 + 5) * 2".to_string()), 16);
    }

    #[test]
    fn calculate_first_grouped_expression() {
        assert_eq!(
            calculate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()),
            13632
        );
    }
}
