use std::fs;

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    print!("Day 10 - A: {:?}", 0);
}

fn part_two() {
    println!(" - B: {:?}", 0);
}

fn get_data() -> Vec<String> {
    fs::read_to_string("data/day10.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
