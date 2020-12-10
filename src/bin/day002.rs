use std::io::{self, BufRead};
use std::vec::Vec;
use std::fs::File;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    target_letter: String,
    password: String,
}

impl Password {
    pub fn new(min: usize, max: usize, tl: String, pw: String) -> Self {
        return Self {
            min,
            max,
            target_letter: tl,
            password: pw,
        };
    }

    pub fn validate(&self) -> bool {
        let cnt = self.password.matches(&self.target_letter).count() as usize;
        return cnt >= self.min && cnt <= self.max;
    }

    pub fn validate2(&self) -> bool {
        let mut cnt = 0;
        let pos = self.min - 1;
        if pos >= self.password.len() {
            return false;
        }
        if self.password.get(pos..pos+1).unwrap() == self.target_letter {
            cnt += 1;
        }
        let pos = self.max - 1;
        if pos >= self.password.len() {
            return cnt == 1;
        }
        if self.password.get(pos..pos+1).unwrap() == self.target_letter {
            return cnt != 1;
        } else {
            return cnt == 1;
        }
    }
}

fn parse_input() -> Vec<Password> {
    let input_file = File::open("src/bin/day002.input").unwrap();
    let lines = io::BufReader::new(input_file).lines().map(|l| l.unwrap());

    let mut passwords = Vec::new();

    for line in lines {
        let mut start = 0;
        let mut end = line.find("-").unwrap();
        let min = line[start..end].parse::<usize>().unwrap();

        start = end + 1;
        end = line.find(" ").unwrap();
        let max = line[start..end].parse::<usize>().unwrap();

        start = end + 1;
        let tl = &line[start..start + 1];
        
        start += 3;
        let pw = &line[start..];

        passwords.push(Password::new(min, max, tl.to_string(), pw.to_string()));
    }

    return passwords;
}

fn part_1(pwds: &Vec<Password>) -> usize {
    let mut valid_cnt = 0;
    for pwd in pwds {
        if pwd.validate() {
            valid_cnt += 1;
        }
    }
    return valid_cnt;
}

fn part_2(pwds: &Vec<Password>) -> usize {
    let mut valid_cnt = 0;
    for pwd in pwds {
        let valid = pwd.validate2();
        if valid {
            valid_cnt += 1;
        } 
    }
    return valid_cnt;
}

fn main() -> io::Result<()> {
    let passwords = parse_input();
    let res = part_1(&passwords);
    println!("Part 1: found {} valid password(s).", res);

    let res = part_2(&passwords);
    println!("Part 2: found {} valid password(s).", res);

    return Ok(());
}
