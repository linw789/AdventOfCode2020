use std::str::from_utf8;
use std::vec::Vec;

fn part_1(passports: &Vec<&str>) -> i32 {
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
            let key = field.split(":").next().unwrap();
            match key {
                "byr" => byr += 1,
                "iyr" => iyr += 1,
                "eyr" => eyr += 1,
                "hgt" => hgt += 1,
                "hcl" => hcl += 1,
                "ecl" => ecl += 1,
                "pid" => pid += 1,
                "cid" => cid += 1,
                _     => panic!("unrecognized field key"),
            }
        }

        cid = 1;
        if byr * iyr * eyr * hgt * hcl * ecl * pid * cid == 1 {
            valid_cnt += 1;
        }
    }
    return valid_cnt;
}

fn main() {
    let input = include_bytes!("day004.input");
    let passports: Vec<&str> = from_utf8(input).unwrap().split("\n\n").collect();
    let res = part_1(&passports);
    println!("Part 1, valid passport count: {}", res);
}
