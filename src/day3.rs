use std::fs;

type Map = Vec<String>;

struct Slope {
    right: usize,
    down: usize,
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let map = expand_map(get_map());
    let slope = Slope { right: 3, down: 1 };
    print!("Day 3 - A: {:?}", tree_count_for_slope(&map, &slope));
}

fn part_two() {
    let map = expand_map(get_map());
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let trees = slopes
        .iter()
        .map(|slope| tree_count_for_slope(&map, slope))
        .fold(1, |product, count| product * count);

    println!(", B: {:?}", trees);
}

fn tree_count_for_slope(map: &Map, slope: &Slope) -> i32 {
    let mut index = 0;
    let mut trees = 0;

    for (n, layer) in map.iter().step_by(slope.down).enumerate() {
        if n == 0 {
            continue;
        }

        index += slope.right;
        if layer.chars().nth(index).unwrap() == '#' {
            trees += 1;
        }
    }
    trees
}

fn expand_map(map: Map) -> Map {
    let expansion_multiplier = 81; // (rows - 1) * (cols - 1) / (cols / right) round down / cols
    let mut expanded_map = Vec::new();
    for layer in map {
        expanded_map.push(layer.repeat(expansion_multiplier));
    }
    expanded_map
}

fn get_map() -> Map {
    fs::read_to_string("data/day3.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
