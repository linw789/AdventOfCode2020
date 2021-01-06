use itertools::Itertools;
use std::collections::HashMap;
use std::str::from_utf8;
use std::str::Lines;
use intbits::{Bits, BitsIndex};
use num_traits::int::PrimInt;

fn mask(mut val: u64, mask: &str) -> u64 {
    for (index, bit) in mask.chars().rev().enumerate() {
        match bit {
            'X' => {}
            '0' => {
                val = val & !(1 << index);
            }
            '1' => {
                val = val | (1 << index);
            }
            _ => panic!("Invalid bit mask."),
        }
    }
    return val;
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
            let val = remainder
                .strip_prefix(" = ")
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap();
            // println!("Pre-mask val: {}", val);
            let val = mask(val, mask_str);
            // println!("Post-mask val: {}", val);
            mem.insert(addr, val);
        }
    }

    let mut sum = 0;
    for v in mem.values() {
        sum += *v;
    }

    return sum;
}

// Distrubute the bits in `bits` to the positions of set bits in `mask`. If there are n set bits in
// `mask`, The bits to be distributed are the lowest n bits in `bits`. For example, if `mask` is 
// 011010, the lowest three bits in `bits` (000xxx) to the corresponding position in `mask`. This
// effectively permutes set bits in `mask`.
fn scatter_bits<T>(mut bits: T, mut mask: T) -> T 
    where T: std::ops::ShrAssign + PrimInt + Bits + BitsIndex<T>, 
{
    let mut result = T::zero();
    while bits != T::zero() && mask != T::zero() {
        let lowest = mask.trailing_zeros(); // lowest set bit position
        mask.set_bit(lowest.into(), false);
        result.set_bit(lowest.into(), bits.bit(0.into()));
        bits >>= T::one();
    }
    return result;
}

fn part_2(lines: Lines) -> u64 {
    let mut mem = HashMap::new();
    for line in lines {
        let mut or_mask = 0;
        let mut float_mask = 0;
        if let Some(maskstr) = line.strip_prefix("mask = ") {
            for c in maskstr.chars() {
                or_mask <<= 1;
                float_mask <<= 1;
                match c {
                    '0' => {},
                    '1' => {
                        or_mask |= 0b1;
                    },
                    'X' => {
                        float_mask |= 0b1;
                    },
                    _ => panic!("Invalid mask bit."),
                }
            }
        } else if let Some(a) = line.strip_prefix("mem[") {
            let (addr, remainder) = a.splitn(2, ']').next_tuple().unwrap();
            let addr = addr.parse::<u64>().unwrap();
            let val = remainder
                .strip_prefix(" = ")
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap();

            let partial_addr = (addr | or_mask) & !float_mask; // set positions of float bits to zero
            for permu in 0..(1 << float_mask.count_ones()) {
                let addr = partial_addr | scatter_bits(permu, float_mask);
                mem.insert(addr, val);
            }
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

    println!("Part 1: {}", part_1(lines.clone()));

    let bits = 0b0000_0001u8;
    let mask = 0b0000_1010u8;
    println!("bits: {:08b}, mask: {:08b}, result: {:08b}", bits, mask, scatter_bits(bits, mask));
    // println!("Part 2: {}", part_2(lines));
}
