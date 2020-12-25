use std::io::{Error, ErrorKind, Result};
use std::str::from_utf8;
use std::vec::Vec;

fn part_1(nums: &Vec<usize>) -> Result<usize> {
    let mut index = 25;
    loop {
        if index >= nums.len() {
            return Err(Error::new(ErrorKind::NotFound, "No invalid number found."));
        }

        let mut valid = false;
        let s = index - 25;
        let e = s + 25;
        for i in s..e - 1 {
            for j in i + 1..e {
                if nums[i] + nums[j] == nums[index] {
                    valid = true;
                }
            }
        }
        if !valid {
            return Ok(nums[index]);
        }
        index += 1;
    }
}

fn main() {
    let input = include_bytes!("day009.input");
    let nums: Vec<usize> = from_utf8(input)
        .unwrap()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    println!(
        "Part 1, the first invalid number: {}",
        part_1(&nums).unwrap()
    );
}
