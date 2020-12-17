use std::str::from_utf8;
use std::vec::Vec;

fn four_digit_check(val: &str, min: i32, max: i32) -> i32 {
    if val.len() != 4 {
        return 0;
    }

    let i = val.parse::<i32>().unwrap_or(-1);
    if i >= min && i <= max {
        return 1;
    } else {
        return 0;
    }
}

fn hgt_check(val: &str) -> i32 {
    let unit = &val[val.len() - 2..];
    let (min, max);
    if unit == "cm" {
        min = 150;
        max = 193;
    } else if unit == "in" {
        min = 59;
        max = 76;
    } else {
        return 0;
    }

    let i = val[..val.len() - 2].parse::<i32>().unwrap_or(-1);
    if i >= min && i <= max {
        return 1;
    } else {
        return 0;
    }
}

fn hcl_check(val: &str) -> i32 {
    if &val[0..1] != "#" {
        return 0;
    }
    if val[1..].contains(|c: char| !c.is_ascii_hexdigit()) {
        return 0;
    } else {
        return 1;
    }
}

fn ecl_check(val: &str) -> i32 {
    match val {
        "amb" => return 1,
        "blu" => return 1,
        "brn" => return 1,
        "gry" => return 1,
        "grn" => return 1,
        "hzl" => return 1,
        "oth" => return 1,
        _ => return 0,
    }
}

fn pid_check(val: &str) -> i32 {
    if val.contains(|c: char| !c.is_ascii_digit()) {
        return 0;
    } else if val.len() != 9 {
        return 0;
    } else {
        return 1;
    }
}

fn part_2(passports: &Vec<&str>) -> i32 {
    let mut valid_cnt = 0;
    for passport in passports {
        let mut byr = 0;
        let mut iyr = 0;
        let mut eyr = 0;
        let mut hgt = 0;
        let mut hcl = 0;
        let mut ecl = 0;
        let mut pid = 0;
        let mut cid = 0;
        for field in passport.split_whitespace() {
            let kv: Vec<&str> = field.split(":").collect();
            match kv[0] {
                "byr" => byr += four_digit_check(kv[1], 1920, 2002),
                "iyr" => iyr += four_digit_check(kv[1], 2010, 2020),
                "eyr" => eyr += four_digit_check(kv[1], 2020, 2030),
                "hgt" => hgt += hgt_check(kv[1]),
                "hcl" => hcl += hcl_check(kv[1]),
                "ecl" => ecl += ecl_check(kv[1]),
                "pid" => pid += pid_check(kv[1]),
                "cid" => cid += 1,
                _ => panic!("unrecognized field key"),
            }
        }

        let mut valid = false;
        cid = 1;
        if byr * iyr * eyr * hgt * hcl * ecl * pid * cid == 1 {
            valid_cnt += 1;
            valid = true;
        }

        if !valid {
            // println!("pspt: {}\nvalid: {}\n\n", passport, valid);
        }
    }
    return valid_cnt;
}

fn main() {
    let input = include_bytes!("day004.input");
    let passports: Vec<&str> = from_utf8(input).unwrap().split("\n\n").collect();
    let res = part_2(&passports);
    println!("Part 2, valid passport count: {}", res);
}
