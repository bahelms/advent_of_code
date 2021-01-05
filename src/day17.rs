use std::{collections::HashSet, fmt, fs, ops::Range};

type Slice = Vec<Vec<char>>;

#[derive(Debug)]
struct Dimension {
    slices: Vec<Slice>,
}

impl Dimension {
    fn new() -> Self {
        Self { slices: Vec::new() }
    }

    fn add_slice(&mut self, slice: Slice) {
        self.slices.push(slice);
    }

    fn pad_slices(&mut self) {
        let mut padded_slices: Vec<Slice> = self.slices.iter().map(pad_slice).collect();
        let mut empty_slice = Vec::new();
        for _ in 0..padded_slices[0].len() {
            let mut cubes = Vec::new();
            for _ in 0..padded_slices[0][0].len() {
                cubes.push('.');
            }
            empty_slice.push(cubes);
        }
        self.slices = vec![empty_slice.clone()];
        self.slices.append(&mut padded_slices);
        self.slices.push(empty_slice);
    }

    fn get_cube(&self, z: i32, y: i32, x: i32) -> char {
        if z < 0 || z >= self.slices.len() as i32 {
            return '.';
        }
        let slice = &self.slices[z as usize];
        if y < 0 || y >= slice.len() as i32 || x < 0 || x >= slice[y as usize].len() as i32 {
            return '.';
        }
        self.slices[z as usize][y as usize][x as usize]
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = "\n".to_string();
        for slice in &self.slices {
            string.push_str("\n");
            for cubes in slice {
                string.push_str(&format!("{:?}\n", cubes));
            }
        }
        write!(f, "{}", string)
    }
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut dimension = Dimension::new();
    dimension.add_slice(get_input());
    for _ in 0..6 {
        dimension = run_cycle(dimension);
    }
    print!("Day 17 - A: {:?}", count_active_cubes(&dimension));
}

type Cube = Vec<i32>;
const DIMENSIONS: i32 = 4;

struct Hyperplane {
    dimensional_ranges: Vec<Range<i32>>,
}

impl Hyperplane {
    pub fn new(dimensions: usize) -> Self {
        Self {
            dimensional_ranges: Vec::with_capacity(dimensions),
        }
    }
}

impl Iterator for Hyperplane {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn part_two() {
    let active_cubes = compile_active_cubes();
    for _ in 0..6 {
        let hyperplane = create_hyperplane(&active_cubes);

        //   for each cube in hyperplane
        for x in hyperplane.dimensional_ranges[0].clone() {
            for y in hyperplane.dimensional_ranges[1].clone() {
                for z in hyperplane.dimensional_ranges[2].clone() {
                    for w in hyperplane.dimensional_ranges[3].clone() {
                        let cube = vec![x, y, z, w];
                        //     calculate active neighbors total
                        //     if becomes active or stays active
                        //       add to new active cubes set
                    }
                }
            }
        }
    }
    // count latest set
    println!(" - B: {:?}", 0);
}

fn compile_active_cubes() -> HashSet<Cube> {
    let mut active_cubes: HashSet<Cube> = HashSet::new();
    for (y, row) in get_input().iter().enumerate() {
        for (x, &cube_char) in row.iter().enumerate() {
            if cube_char == '#' {
                let mut cube = vec![x as i32, y as i32];
                for _ in 0..DIMENSIONS - 2 {
                    cube.push(0);
                }
                active_cubes.insert(cube);
            }
        }
    }
    active_cubes
}

fn create_hyperplane(active_cubes: &HashSet<Cube>) -> Hyperplane {
    let mut hyperplane = Hyperplane::new(DIMENSIONS as usize);
    for _ in 0..DIMENSIONS {
        hyperplane.dimensional_ranges.push(0..0);
    }

    for cube in active_cubes {
        for (i, &coord) in cube.iter().enumerate() {
            if coord < hyperplane.dimensional_ranges[i].start {
                hyperplane.dimensional_ranges[i].start = coord;
            }
            if coord > hyperplane.dimensional_ranges[i].end {
                hyperplane.dimensional_ranges[i].end = coord;
            }
        }
    }
    hyperplane
}

fn get_input() -> Slice {
    fs::read_to_string("data/day17.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .map(|string| string.chars().collect::<Vec<char>>())
        .collect()
}

fn run_cycle(mut dimension: Dimension) -> Dimension {
    // start
    // .....  .....  .....  .....  .....
    // .....  .....  ..#..  .....  .....
    // .....  .....  ...#.  .....  .....
    // .....  .....  .###.  .....  .....
    // .....  .....  .....  .....  .....
    //
    // 1 cycle
    // .....  .....  .....  .....  .....
    // .....  .....  .....  .....  .....
    // .....  .#...  .#.#.  .#...  .....
    // .....  ...#.  ..##.  ...#.  .....
    // .....  ..#..  ..#..  ..#..  .....
    //
    // 2 cycles
    // .....  .....  .....  .....  .....
    // .....  ..#..  ##...  ..#..  .....
    // .....  .#..#  ##...  .#..#  .....
    // ..#..  ....#  #....  ....#  ..#..
    // .....  .#...  ....#  .#...  .....
    // .....  .....  .###.  .....  .....
    let mut new_dim = Dimension::new();
    dimension.pad_slices();
    for (z, slice) in dimension.slices.iter().enumerate() {
        let mut new_slice = slice.clone();
        for (y, cubes) in slice.iter().enumerate() {
            for (x, cube) in cubes.iter().enumerate() {
                let neighbors = count_active_neighbors(&dimension, z as i32, y as i32, x as i32);
                match cube {
                    '.' => {
                        if neighbors == 3 {
                            new_slice[y][x] = '#';
                        }
                    }
                    '#' => {
                        if neighbors != 2 && neighbors != 3 {
                            new_slice[y][x] = '.';
                        }
                    }
                    _ => panic!("shit ain't right"),
                }
            }
        }
        new_dim.add_slice(new_slice);
    }
    new_dim
}

fn pad_slice(slice: &Slice) -> Slice {
    let mut padded_slice = vec![Vec::new()];
    for _ in 0..slice[0].len() + 2 {
        padded_slice[0].push('.');
    }

    for cubes in slice {
        let mut new_cubes = vec!['.'];
        new_cubes.append(&mut cubes.clone());
        new_cubes.push('.');
        padded_slice.push(new_cubes);
    }
    padded_slice.push(padded_slice[0].clone());
    padded_slice
}

fn count_active_neighbors(dimension: &Dimension, z: i32, y: i32, x: i32) -> i32 {
    let mut count = 0;
    let cubes = vec![
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ];

    // eval self
    for &(y, x) in &cubes {
        if dimension.get_cube(z, y, x) == '#' {
            count += 1;
        }
    }
    if dimension.get_cube(z, y, x) == '#' {
        count -= 1;
    }

    // eval next
    for &(y, x) in &cubes {
        if dimension.get_cube(z + 1, y, x) == '#' {
            count += 1;
        }
    }

    // eval previous
    for &(y, x) in &cubes {
        if dimension.get_cube(z - 1, y, x) == '#' {
            count += 1;
        }
    }

    count
}

fn count_active_cubes(dim: &Dimension) -> i32 {
    let mut count = 0;
    for slice in &dim.slices {
        for cubes in slice {
            for &cube in cubes {
                if cube == '#' {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::Dimension;

    #[test]
    fn run_cycle_works() {
        // If a cube is active and 2 or 3 of its neighbors are also active, the cube remains active.
        // Otherwise, the cube becomes inactive.
        // If a cube is inactive but 3 of its neighbors are active, the cube becomes active.
        // Otherwise, the cube remains inactive.
        // start
        // .....  .....  .....  .....  .....
        // .....  .....  ..#..  .....  .....
        // .....  .....  ...#.  .....  .....
        // .....  .....  .###.  .....  .....
        // .....  .....  .....  .....  .....
        //
        // 1 cycle
        // .....  .....  .....  .....  .....
        // .....  .....  .....  .....  .....
        // .....  .#...  .#.#.  .#...  .....
        // .....  ...#.  ..##.  ...#.  .....
        // .....  ..#..  ..#..  ..#..  .....
        //
        // 2 cycles
        // .....  .....  .....  .....  .....
        // .....  ..#..  ##...  ..#..  .....
        // .....  .#..#  ##...  .#..#  .....
        // ..#..  ....#  #....  ....#  ..#..
        // .....  .#...  ....#  .#...  .....
        // .....  .....  .###.  .....  .....
        let mut dim = Dimension::new();
        dim.add_slice(vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ]);
        let dim = super::run_cycle(dim);
        println!("cycle 1 {}", dim);
        assert_eq!(dim.slices.len(), 3);

        let dim = super::run_cycle(dim);
        println!("cycle 2 {}", dim);
        assert_eq!(dim.slices.len(), 5);
    }

    #[test]
    fn run_cycle_works_with_4_dimensions() {
        // If a cube is active and 2 or 3 of its neighbors are also active, the cube remains active.
        // Otherwise, the cube becomes inactive.
        // If a cube is inactive but 3 of its neighbors are active, the cube becomes active.
        // Otherwise, the cube remains inactive.
        // start
        // .#.
        // ..#
        // ###
        //
        // Active: (x, y, z)
        // 1,0,0
        // 2,1,0
        // 1,2,0
        // 2,2,0
        // 3,2,0
        let mut dim = Dimension::new();
        dim.add_slice(vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ]);
        let dim = super::run_cycle(dim);
        println!("cycle 1 {}", dim);
        assert_eq!(dim.slices.len(), 3);

        let dim = super::run_cycle(dim);
        println!("cycle 2 {}", dim);
        assert_eq!(dim.slices.len(), 5);
    }

    #[test]
    fn part_1_test_input_is_correct() {
        let mut dim = Dimension::new();
        dim.add_slice(vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ]);
        for cycle in 0..6 {
            dim = super::run_cycle(dim);
        }
        assert_eq!(super::count_active_cubes(&dim), 112);
    }

    #[test]
    fn count_active_cubes_works() {
        let mut dim = Dimension::new();
        dim.add_slice(vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ]);
        assert_eq!(super::count_active_cubes(&dim), 5);
    }

    #[test]
    fn pad_slice_adds_a_surrounding_layer_of_inactive_cubes() {
        let slice = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ];
        let slice = super::pad_slice(&slice);
        assert_eq!(slice.len(), 5);
        assert_eq!(slice[0], vec!['.', '.', '.', '.', '.']);
        assert_eq!(slice[2], vec!['.', '.', '.', '#', '.']);
        assert_eq!(slice[4], vec!['.', '.', '.', '.', '.']);
    }

    #[test]
    fn dimension_get_cube_returns_char_at_3d_point() {
        let dim = Dimension {
            slices: vec![vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ]],
        };
        assert_eq!(dim.get_cube(0, 1, 2), '#');
        assert_eq!(dim.get_cube(0, 2, 1), '#');
        assert_eq!(dim.get_cube(0, 0, 0), '.');
    }

    #[test]
    fn dimension_get_cube_returns_dot_when_point_is_out_of_bounds() {
        let dim = Dimension {
            slices: vec![vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ]],
        };
        assert_eq!(dim.get_cube(0, -1, 0), '.');
        assert_eq!(dim.get_cube(0, 3, 0), '.');
        assert_eq!(dim.get_cube(0, 0, -1), '.');
        assert_eq!(dim.get_cube(0, 0, 3), '.');
        assert_eq!(dim.get_cube(1, 0, 0), '.');
        assert_eq!(dim.get_cube(-1, 0, 0), '.');
    }

    #[test]
    fn count_active_neighbors_works_with_one_slice() {
        let dim = Dimension {
            slices: vec![vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ]],
        };
        assert_eq!(super::count_active_neighbors(&dim, 0, 0, 0), 1);
        assert_eq!(super::count_active_neighbors(&dim, 0, 0, 1), 1);
        assert_eq!(super::count_active_neighbors(&dim, 0, 0, 2), 2);
        assert_eq!(super::count_active_neighbors(&dim, 0, 1, 1), 5);
        assert_eq!(super::count_active_neighbors(&dim, 0, 2, 2), 2);
    }

    #[test]
    fn count_active_neighbors_works_with_two_slices() {
        let dim = Dimension {
            slices: vec![
                vec![
                    vec!['.', '#', '.'],
                    vec!['.', '.', '#'],
                    vec!['#', '#', '#'],
                ],
                vec![
                    vec!['.', '#', '.'],
                    vec!['.', '.', '#'],
                    vec!['#', '#', '#'],
                ],
            ],
        };
        assert_eq!(super::count_active_neighbors(&dim, 0, 0, 0), 2);
        assert_eq!(super::count_active_neighbors(&dim, 1, 0, 0), 2);
        assert_eq!(super::count_active_neighbors(&dim, 1, 1, 1), 10);
    }

    #[test]
    fn count_active_neighbors_works_with_many_slices() {
        let dim = Dimension {
            slices: vec![
                vec![
                    vec!['.', '.', '.'],
                    vec!['.', '#', '.'],
                    vec!['.', '.', '.'],
                ],
                vec![
                    vec!['.', '#', '.'],
                    vec!['#', '.', '#'],
                    vec!['#', '#', '#'],
                ],
                vec![
                    vec!['.', '.', '.'],
                    vec!['.', '.', '#'],
                    vec!['#', '.', '#'],
                ],
            ],
        };
        assert_eq!(super::count_active_neighbors(&dim, 1, 1, 0), 5);
    }
}
