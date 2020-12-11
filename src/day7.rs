use std::{
    collections::{HashMap, HashSet},
    fs,
};

type BagMap = HashMap<String, Vec<Bag>>;

struct Bag {
    count: i32,
    bag_type: String,
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut bag_map: BagMap = HashMap::new();

    for rule in get_rules() {
        let parent_children: Vec<&str> = rule.split(" contain ").collect();
        let parent = parse_bag_type(parent_children[0], 0);
        let children: Vec<Bag> = parent_children[1].split(",").map(parse_bag).collect();

        for child in children {
            match bag_map.get_mut(&child.bag_type) {
                Some(c) => c.push(Bag {
                    bag_type: parent.to_owned(),
                    count: 0,
                }),
                None => {
                    bag_map.insert(
                        child.bag_type,
                        vec![Bag {
                            bag_type: parent.to_owned(),
                            count: 0,
                        }],
                    );
                }
            }
        }
    }

    let mut parents: HashSet<&str> = HashSet::new();
    find_distinct_parents("shiny gold", &bag_map, &mut parents);
    print!("Day 7 - A: {:?}", parents.len());
}

fn part_two() {
    let mut bag_map: BagMap = HashMap::new();

    for rule in get_rules() {
        let parent_children: Vec<&str> = rule.split(" contain ").collect();
        let parent = parse_bag_type(parent_children[0], 0);
        let children: Vec<Bag> = parent_children[1].split(",").map(parse_bag).collect();
        bag_map.insert(parent, children);
    }
    bag_map.insert("no other".to_string(), Vec::new());
    let count = total_bag_count("shiny gold", &bag_map) - 1;
    println!(", B: {:?}", count);
}

fn total_bag_count(bag_type: &str, map: &BagMap) -> i32 {
    let mut sum = 1;
    let bags = map.get(bag_type).unwrap();
    for bag in bags {
        sum += total_bag_count(&bag.bag_type, map) * bag.count;
    }
    sum
}

fn parse_bag(string: &str) -> Bag {
    let words: Vec<&str> = string.trim().split(" ").collect();
    match words[0].parse() {
        Ok(count) => Bag {
            count,
            bag_type: words[1..words.len() - 1].join(" "),
        },
        Err(_) => Bag {
            count: 0,
            bag_type: "no other".to_string(),
        },
    }
}

fn find_distinct_parents<'a>(bag: &str, bag_map: &'a BagMap, set: &mut HashSet<&'a str>) {
    match bag_map.get(bag) {
        Some(parents) => {
            for parent in parents {
                set.insert(&parent.bag_type);
                find_distinct_parents(&parent.bag_type, bag_map, set);
            }
        }
        None => {}
    }
}

fn parse_bag_type(subrule: &str, start: usize) -> String {
    let words: Vec<&str> = subrule.trim().split(" ").collect();
    words[start..words.len() - 1].join(" ")
}

fn get_rules() -> Vec<String> {
    fs::read_to_string("data/day7.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{total_bag_count, Bag, BagMap};
    use std::collections::HashMap;

    #[test]
    fn total_bag_count_works() {
        let mut map: BagMap = HashMap::new();
        map.insert(
            "A".to_string(),
            vec![Bag {
                count: 2,
                bag_type: "B".to_string(),
            }],
        );
        map.insert(
            "B".to_string(),
            vec![Bag {
                count: 2,
                bag_type: "C".to_string(),
            }],
        );
        map.insert(
            "C".to_string(),
            vec![Bag {
                count: 0,
                bag_type: "none".to_string(),
            }],
        );
        map.insert("none".to_string(), Vec::new());
        assert_eq!(total_bag_count("A", &map), 7);
    }
}
