use regex::Regex;
use std::{collections::HashMap, fs};

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut mem: HashMap<String, u64> = HashMap::new();
    run(get_instructions(), &mut mem);
    let sum: u64 = mem.values().sum();
    print!("Day 14 - A: {:?}", sum);
}

fn run(instructions: Vec<String>, mem: &mut HashMap<String, u64>) {
    let mut mask = String::new();
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    for instr in instructions {
        if instr.starts_with("mask") {
            mask = instr.split(" = ").last().unwrap().to_string();
        }

        if instr.starts_with("mem") {
            let captures = mem_regex.captures(&instr).unwrap();
            let addr = captures.get(1).unwrap().as_str().to_string();
            let num: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
            mem.insert(addr, apply_mask(num, &mask));
        }
    }
}

fn apply_mask(num: u64, mask: &str) -> u64 {
    let bits = format!("{:0>36b}", num);
    let masked_num: String = bits
        .chars()
        .zip(mask.chars())
        .map(|(num_bit, mask_bit)| if mask_bit == 'X' { num_bit } else { mask_bit })
        .collect();
    u64::from_str_radix(&masked_num, 2).unwrap()
}

fn part_two() {
    let mut mem: HashMap<String, u64> = HashMap::new();
    run_v2(get_instructions(), &mut mem);
    let sum: u64 = mem.values().sum();
    println!(" - B: {:?}", sum);
}

fn run_v2(instructions: Vec<String>, mem: &mut HashMap<String, u64>) {
    let mut mask = String::new();
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    for instr in instructions {
        if instr.starts_with("mask") {
            mask = instr.split(" = ").last().unwrap().to_string();
        }

        if instr.starts_with("mem") {
            let captures = mem_regex.captures(&instr).unwrap();
            let addr = captures.get(1).unwrap().as_str().parse().unwrap();
            let num: u64 = captures.get(2).unwrap().as_str().parse().unwrap();

            for addr in decode_memory_addresses(addr, &mask) {
                mem.insert(addr.to_string(), num);
            }
        }
    }
}

fn decode_memory_addresses(num: u64, mask: &str) -> Vec<u64> {
    let mut addrs: Vec<Vec<char>> = vec![Vec::with_capacity(36)];
    let mut decoded_addrs = Vec::new();
    let bits = format!("{:0>36b}", num);
    // 0010101
    // 0X1001X
    //
    // addrs = [
    //   [0]
    // ]
    //
    // addrs = [
    //   [0, 0, 1, 0, 1, 1],
    //   [0, 1, 1, 0, 1, 1],
    // ]
    //
    // addrs = [
    //   [0, 0, 1, 0, 1, 1, 0],
    //   [0, 1, 1, 0, 1, 1, 0],
    //   [0, 0, 1, 0, 1, 1, 1],
    //   [0, 1, 1, 0, 1, 1, 1],
    // ]
    //
    // reverse each addr
    //
    // 0110100
    // 0110110
    // 1110100
    // 1110110
    //
    //
    let mut zipped: Vec<(char, char)> = bits.chars().zip(mask.chars()).collect();
    zipped.reverse();

    for &(addr_bit, mask_bit) in zipped.iter() {
        match mask_bit {
            '1' => {
                for addr in &mut addrs {
                    addr.push('1');
                }
            }
            'X' => {
                let mut double = addrs.clone();
                for addr in &mut double {
                    addr.push('0');
                }
                for addr in &mut addrs {
                    addr.push('1');
                }
                addrs = [addrs, double].concat();
            }
            _ => {
                for addr in &mut addrs {
                    addr.push(addr_bit);
                }
            }
        }
    }

    for mut addr in addrs {
        addr.reverse();
        let binary: String = addr.iter().collect();
        decoded_addrs.push(u64::from_str_radix(&binary, 2).unwrap())
    }
    decoded_addrs
}

fn get_instructions() -> Vec<String> {
    fs::read_to_string("data/day14.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{apply_mask, decode_memory_addresses, run, run_v2};
    use std::collections::HashMap;

    #[test]
    fn running_the_program() {
        let instrs = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];
        let mut mem = HashMap::new();
        run(instrs, &mut mem);
        assert_eq!(mem.values().sum::<u64>(), 165)
    }

    #[test]
    fn test_apply_mask() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        assert_eq!(apply_mask(11, &mask), 73);
        let mask = "000000000000000000000000000000X1001X";
        assert_eq!(apply_mask(100, &mask), 50)
    }

    #[test]
    fn running_the_program_v2() {
        let instrs = vec![
            "mask = 000000000000000000000000000000X1001X".to_string(),
            "mem[42] = 100".to_string(),
            "mask = 00000000000000000000000000000000X0XX".to_string(),
            "mem[26] = 1".to_string(),
        ];
        let mut mem = HashMap::new();
        run_v2(instrs, &mut mem);
        assert_eq!(mem.values().sum::<u64>(), 208)
    }

    #[test]
    fn decode_memory_addresses_works() {
        let mask = "000000000000000000000000000000X1001X";
        let addrs = decode_memory_addresses(42, &mask);
        assert_eq!(addrs.len(), 4);
        assert_eq!(addrs[3], 26);
        assert_eq!(addrs[2], 27);
        assert_eq!(addrs[1], 58);
        assert_eq!(addrs[0], 59);
    }
}
