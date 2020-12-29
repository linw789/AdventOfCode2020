use std::str::from_utf8;
use std::vec::Vec;

fn calc_occupied_adj_seat_num(rows: &Vec<Vec<u8>>, seat_row: i32, seat_col: i32) -> i32 {
    let mut occupied_adj_num = 0;
    for r in (seat_row - 1)..(seat_row + 2) {
        for c in (seat_col - 1)..(seat_col + 2) {
            if r >= 0 && r < rows.len() && c >= 0 && c < rows[r].len() {
                if rows[i][c] == b"#" {
                    occupied_adj_num += 1;
                }
            }
        }
    }
    return occupied_adj_num;
}

fn part_1(rows: &mut Vec<Vec<u8>>) -> i32 {
    
}

fn main() {
    let input = include_bytes!("day011.input");
    let rows: Vec<Vec<u8>> = from_utf8(input).unwrap().lines().map(|s| s.to_owned().as_bytes()).collect();
}
