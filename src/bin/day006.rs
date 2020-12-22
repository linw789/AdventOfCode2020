use std::str::{from_utf8, Split};

fn part_1(group_iter: Split<&str>) -> i32 {
    let mut sum: i32 = 0;

    for grp in group_iter {
        let mut answers: [i32; 26] = [0; 26];
        for answer in grp.lines() {
            for byte in answer.bytes() {
                answers[(byte - b'a') as usize] |= 1;
            }
        }
        sum += answers.iter().sum::<i32>();
    }

    return sum;
}

fn part_2(group_iter: Split<&str>) -> i32 {
    let mut sum: i32 = 0;

    for grp in group_iter {
        let mut answers: [i32; 26] = [0; 26];
        let num_person = grp.lines().count() as i32;
        for answer in grp.lines() {
            for byte in answer.bytes() {
                answers[(byte - b'a') as usize] += 1;
            }
        }

        for a in &answers {
            if *a == num_person {
                sum += 1;
            }
        }
    }

    return sum;
}

fn main() {
    let input = include_bytes!("day006.input");
    let groups_iter = from_utf8(input).unwrap().split("\n\n");

    let res = part_1(groups_iter.clone());
    println!("Part 1, total sum: {}", res);

    let res = part_2(groups_iter);
    println!("Part 2, unique total sum: {}", res);
}
