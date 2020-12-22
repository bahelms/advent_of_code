use std::collections::{HashMap, VecDeque};

const INPUT: [i32; 6] = [18, 11, 9, 0, 5, 1];

#[derive(Debug)]
struct Cache {
    map: HashMap<i32, VecDeque<i32>>,
}

impl Cache {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: i32, value: i32) {
        match self.map.get_mut(&key) {
            Some(queue) => {
                if queue.len() == 1 {
                    queue.push_front(value);
                } else {
                    queue.push_front(value);
                    queue.pop_back();
                }
            }
            None => {
                let mut queue = VecDeque::new();
                queue.push_front(value);
                self.map.insert(key, queue);
            }
        }
    }

    fn get<'a>(&'a self, key: i32) -> &'a VecDeque<i32> {
        self.map.get(&key).unwrap()
    }
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut cache = Cache::new();
    for (i, &num) in INPUT.iter().enumerate() {
        cache.insert(num, i as i32 + 1);
    }

    let num = find_2020(
        &mut cache,
        *INPUT.last().unwrap(),
        INPUT.len() as i32 + 1,
        2020,
    );
    print!("Day 15 - A: {:?}", num);
}

fn part_two() {
    let mut cache = Cache::new();
    for (i, &num) in INPUT.iter().enumerate() {
        cache.insert(num, i as i32 + 1);
    }

    let num = find_2020(
        &mut cache,
        *INPUT.last().unwrap(),
        INPUT.len() as i32 + 1,
        30_000_000,
    );
    println!(" - B: {:?}", num);
}

fn find_2020(cache: &mut Cache, mut last_num: i32, mut current_turn: i32, stop: i32) -> i32 {
    while current_turn <= stop {
        let previous_turns = cache.get(last_num);
        let previous_turn = previous_turns.front().unwrap();
        let diff = current_turn - previous_turn;
        if diff == 1 && previous_turns.len() == 1 {
            last_num = 0;
            cache.insert(last_num, current_turn);
            current_turn += 1;
        } else {
            last_num = previous_turn - previous_turns.back().unwrap();
            cache.insert(last_num, current_turn);
            current_turn += 1;
        }
    }
    last_num
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1_find_2020() {
        let mut cache = super::Cache::new();
        cache.insert(0, 1);
        cache.insert(3, 2);
        cache.insert(6, 3);
        assert_eq!(super::find_2020(&mut cache, 6, 4, 2020), 436);
    }

    #[test]
    fn test2_find_2020() {
        let mut cache = super::Cache::new();
        cache.insert(1, 1);
        cache.insert(3, 2);
        cache.insert(2, 3);
        assert_eq!(super::find_2020(&mut cache, 2, 4, 2020), 1);
    }

    #[test]
    fn test3_find_2020() {
        let mut cache = super::Cache::new();
        cache.insert(3, 1);
        cache.insert(1, 2);
        cache.insert(2, 3);
        assert_eq!(super::find_2020(&mut cache, 2, 4, 2020), 1836);
    }
}
