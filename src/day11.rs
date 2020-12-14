use std::fs;

type Map = Vec<Vec<char>>;

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let map = start_rounds(get_map(), adjacent_occupied_seats, 3);
    let count = total_occupied_seats(map);
    print!("Day 11 - A: {:?}", count);
}

fn part_two() {
    let map = start_rounds(get_map(), nearest_occupied_seats, 4);
    let count = total_occupied_seats(map);
    println!(" - B: {:?}", count);
}

fn start_rounds(map: Map, check_seats: fn(usize, usize, &Map) -> i32, limit: i32) -> Map {
    let mut results = (map, false);
    loop {
        results = apply_rules(results.0, check_seats, limit);
        if !results.1 {
            return results.0;
        }
    }
}

fn apply_rules(map: Map, check_seats: fn(usize, usize, &Map) -> i32, limit: i32) -> (Map, bool) {
    let mut map_changed = false;
    let mut new_map = map.clone();

    for (r, row) in map.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            match map[r][c] {
                'L' => {
                    let count = check_seats(r, c, &map);
                    if count == 0 {
                        new_map[r][c] = '#';
                        map_changed = true;
                    }
                }
                '#' => {
                    let count = check_seats(r, c, &map);
                    if count > limit {
                        new_map[r][c] = 'L';
                        map_changed = true;
                    }
                }
                _ => {}
            }
        }
    }
    (new_map, map_changed)
}

fn adjacent_occupied_seats(r: usize, c: usize, map: &Map) -> i32 {
    let rows = map.len();
    let cols = map[0].len();
    let mut count = 0;

    // left + right
    if c + 1 < cols && map[r][c + 1] == '#' {
        count += 1;
    }
    if c != 0 && map[r][c - 1] == '#' {
        count += 1;
    }

    // up three
    if r != 0 {
        if c + 1 < cols && map[r - 1][c + 1] == '#' {
            count += 1;
        }
        if c != 0 && map[r - 1][c - 1] == '#' {
            count += 1;
        }
        if map[r - 1][c] == '#' {
            count += 1;
        }
    }

    // down three
    if r < rows - 1 {
        if c + 1 < cols && map[r + 1][c + 1] == '#' {
            count += 1;
        }
        if c != 0 && map[r + 1][c - 1] == '#' {
            count += 1;
        }
        if map[r + 1][c] == '#' {
            count += 1;
        }
    }
    count
}

fn nearest_occupied_seats(r: usize, c: usize, map: &Map) -> i32 {
    let rows = map.len();
    let cols = map[0].len();
    let mut count = 0;
    let mut col_idx = c;
    let mut row_idx = r;

    // right
    col_idx += 1;
    while col_idx < cols && map[r][col_idx] == '.' {
        col_idx += 1;
    }
    if col_idx < cols {
        if map[r][col_idx] == '#' {
            count += 1;
        }
    }

    // left
    if c != 0 {
        col_idx = c - 1;
        while col_idx != 0 && map[r][col_idx] == '.' {
            col_idx -= 1;
        }
        if map[r][col_idx] == '#' {
            count += 1;
        }
    }
    col_idx = c;

    if r != 0 {
        // up
        row_idx -= 1;
        while row_idx != 0 && map[row_idx][c] == '.' {
            row_idx -= 1;
        }
        if map[row_idx][c] == '#' {
            count += 1;
        }
        row_idx = r;

        // up diagonal right
        row_idx -= 1;
        col_idx += 1;
        while row_idx != 0 && col_idx < cols && map[row_idx][col_idx] == '.' {
            row_idx -= 1;
            col_idx += 1;
        }
        if col_idx < cols {
            if map[row_idx][col_idx] == '#' {
                count += 1;
            }
        }
        row_idx = r;
        col_idx = c;

        // up diagonal left
        row_idx -= 1;
        if c != 0 {
            col_idx -= 1;
            while row_idx != 0 && col_idx != 0 && map[row_idx][col_idx] == '.' {
                row_idx -= 1;
                col_idx -= 1;
            }
            if map[row_idx][col_idx] == '#' {
                count += 1;
            }
        }
        row_idx = r;
        col_idx = c;
    }

    if r < rows - 1 {
        // down
        row_idx += 1;
        while row_idx < rows - 1 && map[row_idx][c] == '.' {
            row_idx += 1;
        }
        if map[row_idx][c] == '#' {
            count += 1;
        }
        row_idx = r;

        // down diagonal right
        row_idx += 1;
        col_idx += 1;
        while row_idx < rows - 1 && col_idx < cols && map[row_idx][col_idx] == '.' {
            row_idx += 1;
            col_idx += 1;
        }
        if col_idx < cols {
            if map[row_idx][col_idx] == '#' {
                count += 1;
            }
        }
        row_idx = r;
        col_idx = c;

        // down diagonal left
        row_idx += 1;
        if c != 0 {
            col_idx -= 1;
            while row_idx < rows - 1 && col_idx != 0 && map[row_idx][col_idx] == '.' {
                row_idx += 1;
                col_idx -= 1;
            }
            if map[row_idx][col_idx] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn total_occupied_seats(map: Map) -> i32 {
    let mut count = 0;
    for row in map {
        for col in row {
            if col == '#' {
                count += 1;
            }
        }
    }
    count
}

fn get_map() -> Map {
    fs::read_to_string("data/day11.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        adjacent_occupied_seats, apply_rules, nearest_occupied_seats, start_rounds,
        total_occupied_seats,
    };

    #[test]
    fn apply_rules_works() {
        let map = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'],
            vec!['L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
        ];
        let (_, changed) = apply_rules(map, adjacent_occupied_seats, 3);
        assert_eq!(changed, true);
    }

    #[test]
    fn part_one_works() {
        let map = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'],
            vec!['L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
        ];
        let new_map = start_rounds(map, adjacent_occupied_seats, 3);
        assert_eq!(total_occupied_seats(new_map), 37);
    }

    #[test]
    fn nearest_occupied_seats_works() {
        let map = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', '#', '.', 'L', '.', '.', 'L', '.', '.'],
            vec!['L', 'L', 'L', '.', '.', '#', 'L', '.', 'L', 'L'],
            vec!['#', '.', '.', '.', 'X', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', '#', '.', 'L', '.', 'L', 'L'],
            vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', '.', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', '.', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', '#', '#'],
        ];
        assert_eq!(nearest_occupied_seats(4, 4, &map), 5);
    }

    #[test]
    fn part_two_works() {
        let map = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'],
            vec!['L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
        ];
        let new_map = start_rounds(map, nearest_occupied_seats, 4);
        assert_eq!(total_occupied_seats(new_map), 26);
    }
}
