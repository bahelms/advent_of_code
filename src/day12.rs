use std::fs;

const NORTH: i32 = 0;
const EAST: i32 = 1;
const SOUTH: i32 = 2;
const WEST: i32 = 3;

struct Position {
    x: i32, // east/west
    y: i32, // north/south
    direction: i32,
}

#[derive(Debug)]
struct Waypoint {
    x: i32, // east/west
    y: i32, // north/south
}

#[derive(Debug)]
struct Ship {
    x: i32, // east/west
    y: i32, // north/south
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: EAST,
        }
    }

    fn move_east(&mut self, num: i32) {
        self.x += num;
    }

    fn move_west(&mut self, num: i32) {
        self.x -= num;
    }

    fn move_north(&mut self, num: i32) {
        self.y += num;
    }

    fn move_south(&mut self, num: i32) {
        self.y -= num;
    }

    fn turn_left(&mut self, degrees: i32) {
        self.direction -= degrees / 90;
        if self.direction < 0 {
            self.direction += 4;
        }
    }

    fn turn_right(&mut self, degrees: i32) {
        self.direction += degrees / 90;
        if self.direction > 3 {
            self.direction -= 4;
        }
    }

    fn move_forward(&mut self, num: i32) {
        match self.direction {
            NORTH => self.move_north(num),
            EAST => self.move_east(num),
            SOUTH => self.move_south(num),
            WEST => self.move_west(num),
            _ => panic!("that's no direction"),
        }
    }
}

impl Waypoint {
    fn move_east(&mut self, num: i32) {
        self.x += num;
    }

    fn move_west(&mut self, num: i32) {
        self.x -= num;
    }

    fn move_north(&mut self, num: i32) {
        self.y += num;
    }

    fn move_south(&mut self, num: i32) {
        self.y -= num;
    }

    fn rotate_left(&mut self, degrees: i32, ship: &Ship) {
        for _ in 0..degrees / 90 {
            let x_diff = self.x - ship.x;
            let y_diff = self.y - ship.y;
            self.x = ship.x + y_diff * -1;
            self.y = ship.y + x_diff;
        }
    }

    fn rotate_right(&mut self, degrees: i32, ship: &Ship) {
        for _ in 0..degrees / 90 {
            let x_diff = self.x - ship.x;
            let y_diff = self.y - ship.y;
            self.x = ship.x + y_diff;
            self.y = ship.y + x_diff * -1;
        }
    }
}

impl Ship {
    fn move_to_waypoint(&mut self, times: i32, waypoint: &mut Waypoint) {
        for _ in 0..times {
            let x_diff = waypoint.x - self.x;
            let y_diff = waypoint.y - self.y;
            self.x = waypoint.x;
            self.y = waypoint.y;
            waypoint.x = waypoint.x + x_diff;
            waypoint.y = waypoint.y + y_diff;
        }
    }
}

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut position = Position::new();

    for instr in get_instructions() {
        let op = instr.chars().nth(0).unwrap();
        let num: i32 = instr.chars().skip(1).collect::<String>().parse().unwrap();

        match op {
            'N' => position.move_north(num),
            'S' => position.move_south(num),
            'E' => position.move_east(num),
            'W' => position.move_west(num),
            'L' => position.turn_left(num),
            'R' => position.turn_right(num),
            'F' => position.move_forward(num),
            _ => panic!("yo shit broke"),
        }
    }

    let manhattan_distance = position.x.abs() + position.y.abs();
    print!("Day 12 - A: {:?}", manhattan_distance);
}

fn part_two() {
    let mut waypoint = Waypoint { x: 10, y: 1 };
    let mut ship = Ship { x: 0, y: 0 };

    for instr in get_instructions() {
        let op = instr.chars().nth(0).unwrap();
        let num: i32 = instr.chars().skip(1).collect::<String>().parse().unwrap();

        match op {
            'N' => waypoint.move_north(num),
            'S' => waypoint.move_south(num),
            'E' => waypoint.move_east(num),
            'W' => waypoint.move_west(num),
            'L' => waypoint.rotate_left(num, &ship),
            'R' => waypoint.rotate_right(num, &ship),
            'F' => ship.move_to_waypoint(num, &mut waypoint),
            _ => panic!("yo shit broke"),
        }
    }

    let manhattan_distance = ship.x.abs() + ship.y.abs();
    println!(" - B: {:?}", manhattan_distance);
}

fn get_instructions() -> Vec<String> {
    fs::read_to_string("data/day12.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Ship, Waypoint};

    #[test]
    fn rotate_right_works() {
        let mut wp = Waypoint { x: 4, y: 3 };
        let ship = Ship { x: 1, y: 2 };
        wp.rotate_right(90, &ship);
        assert_eq!(wp.x, 2);
        assert_eq!(wp.y, -1);
    }

    #[test]
    fn move_to_waypoint_positive_x_and_y() {
        let mut wp = Waypoint { x: 2, y: 3 };
        let mut ship = Ship { x: 1, y: 0 };
        ship.move_to_waypoint(2, &mut wp);
        assert_eq!(ship.x, 3);
        assert_eq!(ship.y, 6);
        assert_eq!(wp.x, 4);
        assert_eq!(wp.y, 9);
    }

    #[test]
    fn move_to_waypoint_positive_x_negative_y() {
        let mut wp = Waypoint { x: 2, y: -3 };
        let mut ship = Ship { x: 3, y: 2 };
        ship.move_to_waypoint(1, &mut wp);
        assert_eq!(ship.x, 2);
        assert_eq!(ship.y, -3);
        assert_eq!(wp.x, 1);
        assert_eq!(wp.y, -8);
    }

    #[test]
    fn move_to_waypoint_negative_x_negative_y() {
        let mut wp = Waypoint { x: -1, y: -3 };
        let mut ship = Ship { x: -3, y: -2 };
        ship.move_to_waypoint(2, &mut wp);
        assert_eq!(ship.x, 1);
        assert_eq!(ship.y, -4);
        assert_eq!(wp.x, 3);
        assert_eq!(wp.y, -5);
    }
}
