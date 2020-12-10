use std::io::{self};
use std::vec::Vec;
use std::str::from_utf8;

fn part_1(pwds: &Vec<(i32, i32, &str, &str)>) -> usize {
    let mut valid_cnt = 0;
    for pwd in pwds {
        let repeats = pwd.3.matches(pwd.2).count() as i32;

        if repeats >= pwd.0 && repeats <= pwd.1 {
            valid_cnt += 1;
        }
    }
    return valid_cnt;
}

fn part_2(pwds: &Vec<(i32, i32, &str, &str)>) -> usize {
    let mut valid_cnt = 0;
    for pwd in pwds {
        let lo = pwd.0 as usize - 1;
        let hi = pwd.1 as usize - 1;
        let rune = pwd.2;
        let password = pwd.3;

        if (&password[lo..lo+1] == rune) ^ (&password[hi..hi+1] == rune) {
            valid_cnt += 1;
        } 
    }
    return valid_cnt;
}

fn main() -> io::Result<()> {
    let input = include_bytes!("day002.input");
    let tokens: Vec<&str> = from_utf8(input).unwrap().split_whitespace().collect();

    let mut passwords: Vec<(i32, i32, &str, &str)> = Vec::new();
    for chunk in tokens.chunks(3) {
        let mut range = chunk[0].split('-');
        let lo = range.next().unwrap().parse::<i32>().unwrap();
        let hi = range.next().unwrap().parse::<i32>().unwrap();
        let rune = chunk[1].strip_suffix(':').unwrap();
        let password = chunk[2];
        passwords.push((lo, hi, rune, password));
    }

    let res = part_1(&passwords);
    println!("Part 1: found {} valid password(s).", res);

    let res = part_2(&passwords);
    println!("Part 2: found {} valid password(s).", res);

    return Ok(());
}
