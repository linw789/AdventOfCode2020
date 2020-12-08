use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::str::FromStr;
use advent_of_code_2020::*;

fn main() -> io::Result<()> {
    let input_file = File::open("src/bin/day001.input")?;
    let lines = io::BufReader::new(input_file).lines().map(|l| l.unwrap());

    let mut entries = Vec::new();

    for line in lines {
        entries.push(i32::from_str(&line).unwrap());
    }

    let mut res = 0;

    bubble_sort(&mut entries);
    let mut head_i = 0;
    let mut tail_i = entries.len() - 1;
    loop {
        if head_i > tail_i {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "No 2 entries summed to 2020."));
        }

        let sum = entries[head_i] + entries[tail_i];
        if sum < 2020 {
            head_i += 1;
        } else if sum > 2020 {
            tail_i -= 1;
        } else {
            res = entries[head_i] * entries[tail_i];
            break;
        }
    }
    println!("sort first, 2-entry result: {}", res);

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
    println!("3-entry result: {}", res);

    return Ok(());
}
