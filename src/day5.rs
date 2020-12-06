use std::fs;

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let highest_id = get_boarding_passes()
        .iter()
        .map(calculate_seat_id)
        .max()
        .unwrap();
    println!("Day 5A - Answer: {:?}", highest_id);
}

fn part_two() {
    let mut ids: Vec<i32> = get_boarding_passes()
        .iter()
        .map(calculate_seat_id)
        .collect();
    ids.sort();

    for (i, id) in ids.iter().enumerate() {
        if id + 1 != ids[i + 1] {
            println!("Day 5B - Answer: {:?}", id + 1);
            break;
        }
    }
}

fn calculate_seat_id(pass: &String) -> i32 {
    let mut rows_partition = 128;
    let mut seats_partition = 8;
    let mut rows = 0..128;
    let mut seats = 0..8;

    for ch in pass.chars() {
        match ch {
            'F' => {
                rows_partition /= 2;
                rows.end -= rows_partition;
            }
            'B' => {
                rows_partition /= 2;
                rows.start += rows_partition;
            }
            'L' => {
                seats_partition /= 2;
                seats.end -= seats_partition;
            }
            'R' => {
                seats_partition /= 2;
                seats.start += seats_partition;
            }
            _ => panic!("Invalid character"),
        }
    }
    rows.start * 8 + seats.start
}

fn get_boarding_passes() -> Vec<String> {
    fs::read_to_string("data/day5.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_seat_id_works() {
        assert_eq!(super::calculate_seat_id(&"BFFFBBFRRR".to_string()), 567);
        assert_eq!(super::calculate_seat_id(&"FFFBBBFRRR".to_string()), 119);
        assert_eq!(super::calculate_seat_id(&"BBFFBBFRLL".to_string()), 820);
    }
}
