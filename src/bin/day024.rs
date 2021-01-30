use bit_set::BitSet;
use std::str::from_utf8;
use std::str::Lines;

fn parse_grid_neighbour(dir: &str, mut x: usize, mut y: usize) -> (usize, usize) {
    match dir {
        "e" => {
            x += 1;
        }
        "w" => {
            x -= 1;
        }
        "se" => {
            y -= 1;
            if y % 2 == 1 {
                x += 1;
            }
        }
        "sw" => {
            y -= 1;
            if y % 2 == 0 {
                x -= 1;
            }
        }
        "ne" => {
            y += 1;
            if y % 2 == 1 {
                x += 1;
            }
        }
        "nw" => {
            y += 1;
            if y % 2 == 0 {
                x -= 1;
            }
        }
        _ => {
            panic!("Invalid direction.");
        }
    }

    return (x, y);
}

fn part_1(lines: Lines) -> BitSet {
    let dim = 200;
    let mut hex_grid = BitSet::with_capacity(200 * 200);

    for line in lines {
        let mut x: usize = dim / 2;
        let mut y: usize = dim / 2;

        let mut char_iter = line.chars();
        loop {
            match char_iter.next() {
                None => {
                    break;
                }
                Some('e') => {
                    x += 1;
                }
                Some('w') => {
                    x -= 1;
                }
                Some('s') => {
                    y -= 1;
                    match char_iter.next() {
                        Some('e') => {
                            if y % 2 == 1 {
                                x += 1;
                            }
                        }
                        Some('w') => {
                            if y % 2 == 0 {
                                x -= 1;
                            }
                        }
                        _ => {
                            panic!("Invalid character.");
                        }
                    }
                }
                Some('n') => {
                    y += 1;
                    match char_iter.next() {
                        Some('e') => {
                            if y % 2 == 1 {
                                x += 1;
                            }
                        }
                        Some('w') => {
                            if y % 2 == 0 {
                                x -= 1;
                            }
                        }
                        _ => {
                            panic!("Invalid character.");
                        }
                    }
                }
                _ => {
                    panic!("Invalid character.");
                }
            }
        }

        let i = y * dim + x;
        if hex_grid.contains(i) {
            hex_grid.remove(i);
        } else {
            hex_grid.insert(i);
        }
    }

    return hex_grid;
}

fn part_2(mut hex_grid: BitSet) -> usize {
    let days = 100;
    let dim = 200;
    for _ in 0..days {
        let mut pending = Vec::new();

        // Assuming part1 didn't reach the edges, tiles on edges are always white. Saves checking
        // boundary conditions.
        for y in 1..(dim - 1) {
            for x in 1..(dim - 1) {
                let mut black_neighbours = 0;
                for dir in &["e", "w", "se", "sw", "ne", "nw"] {
                    let (nx, ny) = parse_grid_neighbour(dir, x, y);
                    if hex_grid.contains(ny * dim + nx) {
                        black_neighbours += 1;
                    }
                }

                let curr = y * dim + x;
                if hex_grid.contains(curr) {
                    if black_neighbours == 0 || black_neighbours > 2 {
                        pending.push(curr);
                    }
                } else {
                    if black_neighbours == 2 {
                        pending.push(curr);
                    }
                }
            }
        }

        for i in pending.iter().copied() {
            if hex_grid.contains(i) {
                hex_grid.remove(i);
            } else {
                hex_grid.insert(i);
            }
        }
    }

    return hex_grid.len();
}

fn main() {
    let input = include_bytes!("day024.input");
    let lines = from_utf8(input).unwrap().lines();

    let hex_grid = part_1(lines.clone());
    println!("Part 1: {}", hex_grid.len());

    println!("Part 2: {}", part_2(hex_grid));
}
