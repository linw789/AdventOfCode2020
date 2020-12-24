use std::str::from_utf8;
use std::vec::Vec;
use bit_set::BitSet;

enum Kind {
    Nop,
    Acc,
    Jmp,
}

fn part_1(inst_list: &Vec<(Kind, i32)>) -> i32 {
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
        match inst.0 {
            Kind::Nop => index += 1,
            Kind::Acc => { sum += inst.1; index += 1; },
            Kind::Jmp => index += inst.1,
        }
    }
}

fn main() {
    let input = include_bytes!("day008.input");
    let lines = from_utf8(input).unwrap().lines();

    let mut instructions: Vec<(Kind, i32)> = Vec::new();

    for line in lines {
        let kind;

        let mut parts = line.split_whitespace();
        match parts.next().unwrap() {
            "nop" => kind = Kind::Nop,
            "acc" => kind = Kind::Acc,
            "jmp" => kind = Kind::Jmp,
            _ => panic!("Unrecognized instruction kind."),

        }

        let arg = parts.next().unwrap().parse::<i32>().unwrap();

        instructions.push((kind, arg));
    }

    println!("Part 1, sum before an operation is re-visited: {}", part_1(&instructions));
}
