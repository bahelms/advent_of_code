use std::fs;

struct Code {
    op: String,
    int: i32,
    sign: char,
    accessed: i32,
}

struct Program {
    acc: i32,
    ptr: i32,
    codes: Vec<Code>,
}

impl Program {
    pub fn new(codes: Vec<Code>) -> Self {
        Self {
            codes,
            acc: 0,
            ptr: 0,
        }
    }

    fn exec(&mut self) -> i32 {
        self.acc = 0;
        self.ptr = 0;
        loop {
            if self.ptr as usize >= self.codes.len() {
                return 0;
            }

            let code = &mut self.codes[self.ptr as usize];
            code.accessed += 1;
            if code.accessed > 1 {
                return 1;
            }

            match code.op.as_str() {
                "acc" => {
                    match code.sign {
                        '+' => self.acc += code.int,
                        '-' => self.acc -= code.int,
                        _ => {}
                    }
                    self.ptr += 1;
                }
                "jmp" => match code.sign {
                    '+' => self.ptr += code.int,
                    '-' => self.ptr -= code.int,
                    _ => {}
                },
                _ => self.ptr += 1,
            }
        }
    }
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut prg = Program::new(get_codes());
    prg.exec();
    print!("Day 8 - A: {:?}", prg.acc);
}

fn part_two() {
    let codes = get_codes();
    let mut prg = Program::new(codes);
    let mut last_change = 0;

    while prg.exec() == 1 {
        let mut codes = get_codes();
        for i in last_change..codes.len() {
            match codes[i].op.as_str() {
                "jmp" => {
                    codes[i].op = "nop".to_string();
                    last_change = i + 1;
                    break;
                }
                "nop" => {
                    codes[i].op = "jmp".to_string();
                    last_change = i + 1;
                    break;
                }
                _ => {}
            }
        }
        prg.codes = codes;
    }

    println!(", B: {:?}", prg.acc);
}

fn get_codes() -> Vec<Code> {
    fs::read_to_string("data/day8.txt")
        .unwrap()
        .lines()
        .map(parse_code)
        .collect()
}

fn parse_code(line: &str) -> Code {
    let ops: Vec<&str> = line.split(" ").collect();
    Code {
        op: ops[0].to_string(),
        int: ops[1][1..].parse().unwrap(),
        sign: ops[1].chars().nth(0).unwrap(),
        accessed: 0,
    }
}
