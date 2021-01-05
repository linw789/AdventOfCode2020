use std::str::from_utf8;
use std::str::Lines;
use std::collections::HashMap;
use itertools::Itertools;

fn mask(val: u64, mask: &str) -> u64 {
    let mut res = val;
    for (index, bit) in mask.chars().rev().enumerate() {
        match bit {
            'X' => {},
            '0' => {
                res = res & !(1 << index);
            },
            '1' => {
                res = res | (1 << index);
            },
            _ => panic!("Invalid bit mask."),
        }
    }
    return res;
}

fn part_1(lines: Lines) -> u64 {
    let mut mem = HashMap::new();
    let mut mask_str: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    for line in lines {
        if let Some(m) = line.strip_prefix("mask = ") {
            mask_str = m;
            // println!("mask = {}", mask_str);
        } else if let Some(a) = line.strip_prefix("mem[") {
            let (addr, remainder) = a.splitn(2, ']').next_tuple().unwrap();
            let addr = addr.parse::<u64>().unwrap();
            let val = remainder.strip_prefix(" = ").unwrap().trim().parse::<u64>().unwrap();
            // println!("Pre-mask val: {}", val);
            let val = mask(val, mask_str);
            // println!("Post-mask val: {}", val);
            let mem_entry = mem.entry(addr).or_insert(val);
            *mem_entry = val;
        }
    }

    let mut sum = 0;
    for v in mem.values() {
        sum += *v;
    }

    return sum;
}

fn main() {
    let input = include_bytes!("day014.input");
    let lines = from_utf8(input).unwrap().lines();

    println!("Part 1: {}", part_1(lines));
}
