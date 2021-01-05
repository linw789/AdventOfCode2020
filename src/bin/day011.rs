use std::mem::swap;
use std::str::from_utf8;
use std::vec::Vec;

fn count_occupied_adj_seat_num(rows: &[Vec<u8>], seat_row: i32, seat_col: i32) -> i32 {
    let mut occupied_adj_num = 0;
    for r in (seat_row - 1)..(seat_row + 2) {
        for c in (seat_col - 1)..(seat_col + 2) {
            if (r >= 0 && r < rows.len() as i32)
                && (c >= 0 && c < rows[r as usize].len() as i32)
                && (r != seat_row || c != seat_col)
            {
                if rows[r as usize][c as usize] == b'#' {
                    occupied_adj_num += 1;
                }
            }
        }
    }
    return occupied_adj_num;
}

fn count_visible_occupied_seat_num(room: &[Vec<u8>], row: i32, col: i32) -> i32 {
    let row_len = room.len() as i32;
    let col_len = room[0].len() as i32;

    let mut occupied_num = 0;

    // Test 8 direction.
    let step_xy = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];
    for (step_x, step_y) in &step_xy {
        let mut r = row;
        let mut c = col;
        loop {
            r += step_y;
            c += step_x;
            if r < 0 || r >= row_len || c < 0 || c >= col_len {
                break;
            } else {
                match room[r as usize][c as usize] {
                    b'#' => {
                        occupied_num += 1;
                        break;
                    }
                    b'L' => {
                        break;
                    }
                    _ => {}
                };
            }
        }
    }

    return occupied_num;
}

fn count_occupied_seat_num(room: &[Vec<u8>]) -> i32 {
    let mut n = 0;
    for row in room {
        for c in row {
            if *c == b'#' {
                n += 1;
            }
        }
    }
    return n;
}

fn copy_room(src: &[Vec<u8>], dest: &mut [Vec<u8>]) {
    assert!(src.len() == dest.len());
    for r in 0..src.len() {
        let row_src = &src[r];
        let row_dest = &mut dest[r];
        assert!(row_src.len() == row_dest.len());
        for c in 0..row_src.len() {
            row_dest[c] = row_src[c];
        }
    }
}

fn compare_room(a: &[Vec<u8>], b: &[Vec<u8>]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for r in 0..a.len() {
        let row_a = &a[r];
        let row_b = &b[r];
        if row_a.len() != row_b.len() {
            return false;
        }
        for c in 0..a.len() {
            if row_a[c] != row_b[c] {
                return false;
            }
        }
    }
    return true;
}

fn part_1(rows: &[Vec<u8>]) -> i32 {
    let mut room0 = rows.to_vec();
    let mut room1 = rows.to_vec();

    let mut room_src = &mut room0;
    let mut room_dest = &mut room1;

    loop {
        for r in 0..room_src.len() {
            let row_src = &room_src[r];
            let row_dest = &mut room_dest[r];

            for c in 0..row_src.len() {
                match row_src[c] {
                    b'L' => {
                        if count_occupied_adj_seat_num(room_src, r as i32, c as i32) == 0 {
                            row_dest[c] = b'#';
                        }
                    }
                    b'#' => {
                        if count_occupied_adj_seat_num(room_src, r as i32, c as i32) >= 4 {
                            row_dest[c] = b'L';
                        }
                    }
                    _ => {}
                };
            }
        }

        let occupied_seat_n = count_occupied_seat_num(room_dest);
        // println!("Occupied seat number: {}", occupied_seat_n);

        if compare_room(room_src, room_dest) {
            return occupied_seat_n;
        }

        swap(&mut room_src, &mut room_dest);
        copy_room(room_src, room_dest);
        assert!(compare_room(room_src, room_dest));
    }
}

fn part_2(origianl: &[Vec<u8>]) -> i32 {
    let mut room0 = origianl.to_vec();
    let mut room1 = origianl.to_vec();

    let mut room_src = &mut room0;
    let mut room_dest = &mut room1;

    loop {
        for r in 0..room_src.len() {
            let row_src = &room_src[r];
            let row_dest = &mut room_dest[r];

            for c in 0..row_src.len() {
                match row_src[c] {
                    b'L' => {
                        if count_visible_occupied_seat_num(room_src, r as i32, c as i32) == 0 {
                            row_dest[c] = b'#';
                        }
                    }
                    b'#' => {
                        if count_visible_occupied_seat_num(room_src, r as i32, c as i32) >= 5 {
                            row_dest[c] = b'L';
                        }
                    }
                    _ => {}
                };
            }
        }

        let occupied_seat_n = count_occupied_seat_num(room_dest);
        // println!("Occupied seat number: {}", occupied_seat_n);

        if compare_room(room_src, room_dest) {
            return occupied_seat_n;
        }

        swap(&mut room_src, &mut room_dest);
        copy_room(room_src, room_dest);
        assert!(compare_room(room_src, room_dest));
    }
}

fn main() {
    let input = include_bytes!("day011.input");
    let rows: Vec<Vec<u8>> = from_utf8(input)
        .unwrap()
        .lines()
        .map(|s| s.to_owned().into_bytes())
        .collect();

    // println!("Initial occupied seats: {}", count_occupied_seat_num(&rows));
    println!(
        "Part 1, occupied seat number after stablization: {}",
        part_1(&rows)
    );
    println!(
        "Part 2, occupied seat number after stablization: {}",
        part_2(&rows)
    );
}
