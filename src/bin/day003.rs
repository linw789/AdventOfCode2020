use std::vec::Vec;
use std::str::from_utf8;

fn traverse(rows: &Vec<&str>, rsteps: usize, dsteps: usize) -> i32 {
    let mut posx = 0;
    let mut posy = 0;
    let mut num_tree = 0;

    loop {
        posx += rsteps;
        posy += dsteps;
        if posy >= rows.len() {
            break;
        }
        posx = posx % rows[posy].len();
        if rows[posy][posx..posx+1] == *"#" {
            num_tree += 1;
        }
    }
    return num_tree;
}

fn part_1(rows: &Vec<&str>) -> i32 {
    return traverse(rows, 3, 1);
}

fn part_2(rows: &Vec<&str>) -> i32 {
    let mut total = 1;
    total *= traverse(rows, 1, 1);
    total *= traverse(rows, 3, 1);
    total *= traverse(rows, 5, 1);
    total *= traverse(rows, 7, 1);
    total *= traverse(rows, 1, 2);
    return total;
}

fn main() {
    let input = include_bytes!("day003.input");
    let rows: Vec<&str> = from_utf8(input).unwrap().split_whitespace().collect();
    let res = part_1(&rows);
    println!("Part 1, number of trees: {}", res);

    let res = part_2(&rows);
    println!("Part 2, number of trees: {}", res);
}
