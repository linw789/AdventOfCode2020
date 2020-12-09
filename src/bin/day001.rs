use advent_of_code_2020::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[macro_use]
extern crate exec_time;

#[exec_time]
fn part_1(entries: &[i32]) -> i32 {
    let mut head_i = 0;
    let mut tail_i = entries.len() - 1;
    loop {
        if head_i >= tail_i {
            panic!("No 2 entries summed to 2020.");
        }

        let sum = entries[head_i] + entries[tail_i];
        if sum < 2020 {
            head_i += 1;
        } else if sum > 2020 {
            tail_i -= 1;
        } else {
            return entries[head_i] * entries[tail_i];
        }
    }
}

#[exec_time]
fn part_2(entries: &[i32]) -> i32 {
    for i in &entries[..entries.len() - 2] {
        for j in &entries[1..entries.len() - 1] {
            for k in &entries[2..] {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    panic!("No 3 entries summed to 2020.");
}

fn main() -> io::Result<()> {
    let input_file = File::open("src/bin/day001.input")?;
    let lines = io::BufReader::new(input_file).lines();

    let mut entries: Vec<i32> = lines.map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

    quick_sort(&mut entries);
    let res = part_1(&entries);
    println!("sort first, 2-entry result: {}", res);

    let res = part_2(&entries);
    println!("3-entry result: {}", res);

    return Ok(());
}
