use std::io::{self, BufRead};
use std::vec::Vec;
use std::fs::File;

#[derive(Debug)]
struct Password {
    min: i32,
    max: i32,
    target_letter: String,
    password: String,
}

impl Password {
    pub fn new(min: i32, max: i32, tl: String, pw: String) -> Self {
        return Self {
            min,
            max,
            target_letter: tl,
            password: pw,
        };
    }

    pub fn validate(&self) -> bool {
        let cnt = self.password.matches(&self.target_letter).count() as i32;
        return cnt >= self.min && cnt <= self.max;
    }
}

fn parse_input() -> Vec<Password> {
    let input_file = File::open("src/bin/day002.input").unwrap();
    let lines = io::BufReader::new(input_file).lines().map(|l| l.unwrap());

    let mut passwords = Vec::new();

    for line in lines {
        let mut start = 0;
        let mut end = line.find("-").unwrap();
        let min = line[start..end].parse::<i32>().unwrap();

        start = end + 1;
        end = line.find(" ").unwrap();
        let max = line[start..end].parse::<i32>().unwrap();

        start = end + 1;
        let tl = &line[start..start + 1];
        
        start += 3;
        let pw = &line[start..];

        passwords.push(Password::new(min, max, tl.to_string(), pw.to_string()));
    }

    return passwords;
}

fn validate_passwords(pwds: &Vec<Password>) -> i32 {
    let mut valid_cnt = 0;
    for pwd in pwds {
        if pwd.validate() {
            valid_cnt += 1;
        }
    }
    return valid_cnt;
}

fn main() -> io::Result<()> {
    let passwords = parse_input();
    let res = validate_passwords(&passwords);
    println!("Found {} valid password(s).", res);

    return Ok(());
}
