use std::cmp::{max, min};
use std::io::{Error, ErrorKind, Result};
use std::str::from_utf8;
use std::vec::Vec;

fn part_1(nums: &Vec<i64>) -> Result<i64> {
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

fn part_2(nums: &Vec<i64>, invalid_num: i64) -> i64 {
    for k in 0..nums.len() - 1 {
        let mut lo = i64::MAX;
        let mut hi = i64::MIN;
        let mut total = 0;
        for i in &nums[k..] {
            lo = min(lo, *i);
            hi = max(hi, *i);
            total += *i;
            if total == invalid_num {
                return lo + hi;
            }
        }
    }
    return -1;
}

fn main() {
    let input = include_bytes!("day009.input");
    let nums: Vec<i64> = from_utf8(input)
        .unwrap()
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let invalid_num = part_1(&nums).unwrap();
    println!("Part 1, the first invalid number: {}", invalid_num);

    println!("Part 2, weakness number: {}", part_2(&nums, invalid_num));
}
