use std::str::from_utf8;
use std::vec::Vec;
use bit_set::BitSet;
use itertools::Itertools;

enum Kind {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn part_1(inst_list: &Vec<Kind>) -> i32 {
    let mut visited = BitSet::new();
    let mut sum = 0;
    let mut index: i32 = 0;
    loop {
        if index as usize >= inst_list.len() {
            return sum;
        }

        if visited.contains(index as usize) {
            return sum;
        }

        visited.insert(index as usize);

        let inst = &inst_list[index as usize];
        match inst {
            Kind::Nop(_) => index += 1,
            Kind::Acc(arg) => { sum += arg; index += 1; },
            Kind::Jmp(arg) => index += arg,
        }
    }
}

fn main() {
    let input = include_bytes!("day008.input");
    let lines = from_utf8(input).unwrap().lines();

    let mut instructions: Vec<Kind> = Vec::new();

    for line in lines {
        let kind;
        let (k, v) = line.split_whitespace().next_tuple().unwrap();
        match k {
            "nop" => kind = Kind::Nop(v.parse::<i32>().unwrap()),
            "acc" => kind = Kind::Acc(v.parse::<i32>().unwrap()),
            "jmp" => kind = Kind::Jmp(v.parse::<i32>().unwrap()),
            _ => panic!("Unrecognized instruction kind."),

        }
        instructions.push(kind);
    }

    println!("Part 1, sum before an operation is re-visited: {}", part_1(&instructions));
}
