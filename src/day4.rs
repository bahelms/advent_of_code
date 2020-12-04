use std::fs;

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const REQUIRED_FIELDS: [&str; 7] = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

pub fn execute() {
    part_one();
    part_two();
}

fn part_one() {
    let mut valid_passports = 0;
    for passport in get_passports() {
        let fields = parse_fields(&passport);

        if fields.len() == 7 {
            valid_passports += 1;
        }
    }
    println!("Day 4A - Answer: {:?}", valid_passports);
}

fn part_two() {
    let mut valid_passports = 0;
    for passport in get_passports() {
        let fields = parse_fields(&passport);

        if fields_are_valid(&fields) {
            valid_passports += 1;
        }
    }
    println!("Day 4B - Answer: {:?}", valid_passports);
}

fn fields_are_valid(fields: &Vec<Vec<&str>>) -> bool {
    let mut names: Vec<&str> = fields.iter().map(|f| f[0]).collect();
    names.sort();
    if names != REQUIRED_FIELDS {
        return false;
    }

    for field in fields {
        let name = field[0];
        let value = field[1];

        match name {
            "byr" => {
                // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                let year = value.parse::<i32>().expect("must me a number");
                if year < 1920 || year > 2002 {
                    return false;
                }
            }
            "iyr" => {
                // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                let year = value.parse::<i32>().expect("must me a number");
                if year < 2010 || year > 2020 {
                    return false;
                }
            }
            "eyr" => {
                // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                let year = value.parse::<i32>().expect("must me a number");
                if year < 2020 || year > 2030 {
                    return false;
                }
            }
            "ecl" => {
                // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                if !EYE_COLORS.contains(&value) {
                    return false;
                }
            }
            "pid" => {
                // pid (Passport ID) - a nine-digit number, including leading zeroes.
                if value.len() != 9 || !value.chars().all(|ch| ch.is_digit(10)) {
                    return false;
                }
            }
            "hcl" => {
                // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                let mut chars = value.chars();
                if value.len() != 7
                    || chars.next().unwrap() != '#'
                    || !chars.all(|ch| ch.is_digit(16))
                {
                    return false;
                }
            }
            // hgt (Height) - a number followed by either cm or in:
            //     If cm, the number must be at least 150 and at most 193.
            //     If in, the number must be at least 59 and at most 76.
            "hgt" => {
                let index = value.len() - 2;
                let measurement = &value[index..];
                match &value[..index].parse::<i32>() {
                    Ok(height) => match measurement {
                        "cm" => {
                            if *height < 150 || *height > 193 {
                                return false;
                            }
                        }
                        "in" => {
                            if *height < 59 || *height > 76 {
                                return false;
                            }
                        }
                        _ => {
                            return false;
                        }
                    },
                    Err(_) => {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }
    true
}

fn get_passports() -> Vec<String> {
    fs::read_to_string("data/day4.txt")
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect()
}

fn parse_fields<'a>(passport: &'a String) -> Vec<Vec<&'a str>> {
    passport
        .split(&['\n', ' '][..])
        .filter(|&f| !f.starts_with("cid") && f != "")
        .map(|f| f.split(":").collect::<Vec<&str>>())
        .collect()
}
