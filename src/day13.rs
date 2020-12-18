use std::{collections::HashMap, fs};

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let notes = get_notes();
    let earliest_departure: f64 = notes[0].parse().unwrap();
    let mut minutes_to_id = HashMap::new();
    let minutes: i32 = notes[1]
        .split(",")
        .filter(|&note| note != "x")
        .map(|id| {
            let id: f64 = id.parse().unwrap();
            let minutes = ((earliest_departure / id).ceil() * id) as i32;
            minutes_to_id.insert(minutes, id);
            minutes
        })
        .min()
        .unwrap();
    let answer =
        *minutes_to_id.get(&minutes).unwrap() as i32 * (minutes - earliest_departure as i32);
    print!("Day 13 - A: {:?}", answer);
}

fn part_two() {
    let notes = get_notes();
    println!(" - B: {:?}", earliest_time(prepare_schedule(&notes[1])));
}

fn prepare_schedule(schedule: &String) -> Vec<(usize, usize)> {
    schedule
        .split(",")
        .enumerate()
        .filter(|&(_, id)| id != "x")
        .map(|(i, s)| (i as usize, s.parse().unwrap()))
        .collect()
}

fn earliest_time(buses: Vec<(usize, usize)>) -> usize {
    let mut iteration = 1;
    let (_, first_id) = buses[0];
    let mut lcm = first_id;
    let mut time = first_id;

    for (offset, id) in &buses[1..] {
        while (time + iteration * lcm + offset) % id != 0 {
            iteration += 1;
        }
        time += lcm * iteration;
        lcm *= id;
        iteration = 1;
    }
    time

    // Chinese remainder theorem (not used)
    // t % 17 == 0    t = 0 (mod 17)
    // t % 13 == 11 (13-offset)   t = 11 (mod 13)
    // t % 19 == 16   t = 16 (mod 19)
    //
    // t % ni == bi
    // N = 17 * 13 * 19
    // Ni = N/ni
    // xi = 1 (mod ni)  // inverse of Ni
    //
    // bi Ni   xi P(biNixi)
    // b1 n2n3 x1 b1N1x1
    // bn...
    // t = sum(P) % N
    //
    // 0   247  11 (247x1 = 1 (mod 17)) 0
    // 11  323  6                       21318
    // 16  221  8                       28288
    //
    // 247x1 = 1 (mod 17)  247/17 == 14  rem 9
    //  14x1 = 1 (mod 17)  14*11%17 == 1  brute force which X * 14 % 17 == 1
    //    x1 = 11
}

fn get_notes() -> Vec<String> {
    fs::read_to_string("data/day13.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{earliest_time, prepare_schedule};

    #[test]
    fn earliest_time_works() {
        let s1 = "17,x,13,19".to_string();
        assert_eq!(earliest_time(prepare_schedule(&s1)), 3417);
        let s2 = "67,7,59,61".to_string();
        assert_eq!(earliest_time(prepare_schedule(&s2)), 754018);
        let s3 = "67,x,7,59,61".to_string();
        assert_eq!(earliest_time(prepare_schedule(&s3)), 779210);
        let s4 = "67,7,x,59,61".to_string();
        assert_eq!(earliest_time(prepare_schedule(&s4)), 1261476);
        let s5 = "1789,37,47,1889".to_string();
        assert_eq!(earliest_time(prepare_schedule(&s5)), 1202161486);
        let s5 = "7,13,x,x,59,x,31,19".to_string();
        assert_eq!(earliest_time(prepare_schedule(&s5)), 1068781);
    }
}
