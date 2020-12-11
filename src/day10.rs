use std::{collections::HashMap, fs};

type Graph = HashMap<i32, Vec<i32>>;

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let ratings = get_ratings();
    let diffs = find_differences(&ratings);
    let dist = diffs.get(&1).unwrap() * diffs.get(&3).unwrap();
    print!("Day 10 - A: {:?}", dist);
}

fn part_two() {
    let ratings = get_ratings();
    let final_rating = ratings[ratings.len() - 1];
    let graph = convert_to_graph(&ratings);
    println!(
        " - B: {:?}",
        number_of_combinations(0, &graph, final_rating)
    );
}

fn find_differences(ratings: &Vec<i32>) -> HashMap<i32, i32> {
    let mut diffs = HashMap::new();
    for pair in ratings.windows(2) {
        let diff = pair[1] - pair[0];
        diffs.entry(diff).and_modify(|d| *d += 1).or_insert(1);
    }
    diffs
}

fn number_of_combinations(child: i32, graph: &Graph, final_rating: i32) -> i32 {
    if child == final_rating {
        return 1;
    }

    let mut count = 0;
    let children = graph.get(&child).unwrap();
    for &child in children {
        count += number_of_combinations(child, graph, final_rating);
    }
    count
}

fn convert_to_graph(ratings: &Vec<i32>) -> Graph {
    let mut edges = HashMap::new();
    let ratings_count = ratings.len();

    for (i, &rating) in ratings.iter().enumerate() {
        if i + 1 == ratings_count {
            break;
        }

        let mut children = vec![ratings[i + 1]];

        if i + 2 < ratings_count {
            let next_rating = ratings[i + 2];
            let diff = next_rating - rating;

            if diff == 2 || diff == 3 {
                children.push(next_rating);
                if i + 3 < ratings_count {
                    let next_rating2 = ratings[i + 3];
                    if next_rating2 - rating == 3 {
                        children.push(next_rating2);
                    }
                }
            }
        }
        edges.insert(rating, children);
    }
    edges
}

fn get_ratings() -> Vec<i32> {
    let mut ratings: Vec<i32> = fs::read_to_string("data/day10.txt")
        .unwrap()
        .lines()
        .map(|num| num.parse().unwrap())
        .collect();
    ratings.push(0);
    ratings.sort();
    ratings.push(ratings.last().unwrap() + 3);
    ratings
}

#[cfg(test)]
mod tests {
    use super::{convert_to_graph, find_differences, number_of_combinations};

    #[test]
    fn test_find_differences() {
        let mut ratings = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
        let diffs = find_differences(&ratings);
        assert_eq!(diffs.get(&1).unwrap(), &7);
        assert_eq!(diffs.get(&3).unwrap(), &5);
    }

    #[test]
    fn test_convert_to_graph() {
        let mut ratings = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
        let graph = convert_to_graph(&ratings);
        assert_eq!(graph.get(&4).unwrap(), &vec![5, 6, 7]);
    }

    #[test]
    fn test_number_of_combinations() {
        let mut ratings = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
        let graph = convert_to_graph(&ratings);
        assert_eq!(number_of_combinations(0, &graph, 22), 8);

        let mut ratings = vec![
            0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35,
            38, 39, 42, 45, 46, 47, 48, 49, 52,
        ];
        let graph = convert_to_graph(&ratings);
        assert_eq!(number_of_combinations(0, &graph, 52), 19208);
    }
}
