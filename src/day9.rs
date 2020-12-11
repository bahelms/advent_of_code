use std::fs;

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    const WIN_SIZE: usize = 25;
    let nums = get_numbers();

    for (i, window) in nums.windows(WIN_SIZE).enumerate() {
        let mut pair_found = false;
        let index = i + WIN_SIZE;
        if index >= nums.len() {
            break;
        }

        let next_num = nums[i + WIN_SIZE];
        'sum: for (idx, num) in window.iter().enumerate() {
            for other in &window[idx + 1..] {
                let sum = num + other;
                if sum == next_num {
                    pair_found = true;
                    break 'sum;
                }
            }
        }
        if !pair_found {
            print!("Day 9 - A: {:?}", next_num);
            return;
        }
    }
}

fn part_two() {
    const TARGET: usize = 57195069;
    let nums = get_numbers();
    for (i, &num) in nums.iter().enumerate() {
        let mut sum = num;
        let mut highest_num = num;
        let mut smallest_num = num;
        for &other in &nums[i + 1..] {
            if other > highest_num {
                highest_num = other;
            }
            if other < smallest_num {
                smallest_num = other;
            }

            sum += other;
            if sum == TARGET {
                println!(" - B: {:?}", smallest_num + highest_num);
                return;
            } else if sum > TARGET {
                break;
            }
        }
    }
}

fn get_numbers() -> Vec<usize> {
    fs::read_to_string("data/day9.txt")
        .unwrap()
        .lines()
        .map(|num| num.parse().unwrap())
        .collect()
}
