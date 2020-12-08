use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let input_file = File::open("src/bin/day001.input")?;
    let lines = io::BufReader::new(input_file).lines().map(|l| l.unwrap());

    let mut entries = Vec::new();

    for line in lines {
        entries.push(i32::from_str(&line).unwrap());
    }

    let mut res = 0;

    for i in &entries[..entries.len() - 1] {
        for j in &entries[1..] {
            if i + j == 2020 {
                res = i * j;
                break;
            }
        }
    }
    println!("result: {}", res);

    for i in &entries[..entries.len() - 2] {
        for j in &entries[1..entries.len() - 1] {
            for k in &entries[2..] {
                if i + j + k == 2020 {
                    res = i * j * k;
                    break;
                }
            }
        }
    }
    println!("result: {}", res);

    return Ok(());
}
