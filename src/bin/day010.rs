use advent_of_code_2020::*;
use std::str::from_utf8;
use std::vec::Vec;

fn part_1(adapters: &Vec<i32>) -> i32 {
    let mut chain = Vec::with_capacity(adapters.len() + 2);
    chain.push(0); // Add outlet jolt.
    chain.append(&mut adapters.clone());
    quick_sort(&mut chain);
    chain.push(chain[chain.len() - 1] + 3); // Add device jolt.

    let (diff_one_s, diff_three_s) =
        &chain[0..chain.len() - 1]
            .iter()
            .zip(&chain[1..])
            .fold((0, 0), |(sum1, sum3), (a, b)| {
                let mut s1 = sum1;
                let mut s3 = sum3;
                if b - a == 1 {
                    s1 += 1;
                } else if b - a == 3 {
                    s3 += 1;
                }
                return (s1, s3);
            });

    println!("diff1: {}, diff3: {}", diff_one_s, diff_three_s);
    return diff_one_s * diff_three_s;
}

// Calculate permutation of `jolts` under the condition that both the first and the last element
// cannot be removed.
fn calc_permutation(jolts: &[i32]) -> (i64, i64, i64) {
    if jolts.len() == 4 {
        let permu = if jolts[2] - jolts[0] <= 3 {
            2
        } else {
            1
        };
        // Because the last adapter is the device whose joil is always 3 higher than the second
        // last adapter, the second last must not be missing.
        return (permu, 1, 1);
    }

    let (sub_permu, subs_permu, subss_permu) = calc_permutation(&jolts[1..]);

    let mut permu = sub_permu;

    if jolts[2] - jolts[0] <= 3 {
        // If the second element can be removed;
        permu += subs_permu;
    }

    if jolts[3] - jolts[0] <= 3 {
        // If the third element can be removed. `jolts` is already sorted, if the second element
        // can't be removed, neither can the third.
        permu += subss_permu;
    }

    return (permu, sub_permu, subs_permu);
}

fn part_2(adapters: &Vec<i32>) -> i64 {
    let mut chain = Vec::with_capacity(adapters.len() + 2);
    chain.push(0); // Add outlet jolt.
    chain.append(&mut adapters.clone());
    quick_sort(&mut chain);
    chain.push(chain[chain.len() - 1] + 3); // Add device jolt.

    let (permu, _, _) = calc_permutation(&chain);
    return permu;
}

fn test() {
    let test_input =
        "28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3";
    let adapters: Vec<i32> = test_input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("Part 2, test input result: {}", part_2(&adapters));
}

fn main() {
    let input = include_bytes!("day010.input");
    let adapters: Vec<i32> = from_utf8(input)
        .unwrap()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!(
        "Part 1, number of 1-jolt diff + number of 3-jolt diff: {}",
        part_1(&adapters)
    );
    println!("Part 2, unique permutation: {}", part_2(&adapters));
    test();
}
