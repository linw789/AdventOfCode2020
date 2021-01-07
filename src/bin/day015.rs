use std::collections::HashMap;

fn part_1(start_nums: &[u64], end_spoken_pos: usize) -> u64 {
    let mut num_pos = HashMap::new();

    for (i, n) in (&start_nums[..(start_nums.len() - 1)]).iter().enumerate() {
        num_pos.insert(*n, i);
    }

    let mut last_num = start_nums[start_nums.len() - 1];

    for i in start_nums.len()..end_spoken_pos {
        match num_pos.insert(last_num, i - 1) {
            Some(pos) => {
                last_num = ((i - 1) - pos) as u64;
            }
            None => {
                last_num = 0;
            }
        }
    }

    return last_num;
}

fn part_2(start_nums: &[u64]) -> u64 {
    return part_1(start_nums, 30000000);
}

fn main() {
    let input = [0, 1, 4, 13, 15, 12, 16];

    println!("Part 1: {}", part_1(&input, 2020));
    println!("Part 2: {}", part_2(&input));
}
