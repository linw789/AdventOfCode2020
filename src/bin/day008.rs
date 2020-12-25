use bit_set::BitSet;
use itertools::Itertools;
use std::str::from_utf8;
use std::vec::Vec;

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
            Kind::Acc(arg) => {
                sum += arg;
                index += 1;
            }
            Kind::Jmp(arg) => index += arg,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum TermState {
    Invalid,
    Terminating, // this instruction leads to termination
    Looping,     // this instruction leads to looping
}

fn test_terminating(
    index: usize,
    inst_list: &Vec<Kind>,
    visited: &mut BitSet,
    term_set: &mut Vec<TermState>,
) -> bool {
    if index >= inst_list.len() {
        return true;
    } else if term_set[index] == TermState::Terminating {
        assert!(
            visited.contains(index) == true,
            "An instruction determined to be terminating must have been visited."
        );
        return true;
    } else if term_set[index] == TermState::Looping {
        assert!(
            visited.contains(index) == true,
            "An instruction determined to be terminating must have been visited."
        );
        return false;
    } else if visited.contains(index) {
        return false;
    } else {
        visited.insert(index);
        let next_index = match &inst_list[index] {
            Kind::Nop(_) => index + 1,
            Kind::Acc(_) => index + 1,
            Kind::Jmp(arg) => ((index as i32) + *arg) as usize,
        };
        if test_terminating(next_index, inst_list, visited, term_set) {
            term_set[index] = TermState::Terminating;
            return true;
        } else {
            term_set[index] = TermState::Looping;
            return false;
        }
    }
}

fn part_2(inst_list: &Vec<Kind>) -> i32 {
    let mut visited = BitSet::new();
    let mut term_set = Vec::new();
    term_set.resize(inst_list.len(), TermState::Invalid);

    // Calculate if an instruction leads to termination when execution starts at the said
    // instruction. Calculating the terminating state of all instructions is O(n), because if an
    // instruction is determined to be terminating/looping, any instruction preceding it must also
    // be terminating/looping. Therefore the terminating state of each instruction only needs to be
    // calculated once.
    for inst_i in 0..inst_list.len() {
        test_terminating(inst_i, inst_list, &mut visited, &mut term_set);
    }
    println!("Calculated instruction terminating state.");

    let mut sum = 0;
    let mut index = 0;
    // Switch nop and jmp everytime util a terminating set is found. Then set `found_bug` to true
    // and stop switching nop and jmp.
    let mut found_bug = false;

    loop {
        if index >= inst_list.len() {
            return sum;
        }

        match &inst_list[index] {
            Kind::Nop(arg) => {
                if found_bug {
                    index += 1;
                } else {
                    // Switch to jmp and test if the next instruction is terminating.
                    let jmp_index = ((index as i32) + *arg) as usize;
                    if term_set[jmp_index] == TermState::Terminating {
                        found_bug = true;
                        index = jmp_index;
                    } else {
                        index += 1;
                    }
                }
            }
            Kind::Acc(arg) => {
                sum += arg;
                index += 1;
            }
            Kind::Jmp(arg) => {
                if found_bug {
                    index = ((index as i32) + *arg) as usize;
                } else {
                    // Switch to nop and test if the next instruction is terminating.
                    if term_set[index + 1] == TermState::Terminating {
                        found_bug = true;
                        index += 1;
                    } else {
                        index = ((index as i32) + *arg) as usize;
                    }
                }
            }
        };
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

    println!(
        "Part 1, sum before an operation is re-visited: {}",
        part_1(&instructions)
    );
    println!("Part 1, sum after fixing bug: {}", part_2(&instructions));
}
