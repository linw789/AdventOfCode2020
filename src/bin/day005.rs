use std::str::from_utf8;
use std::str::Lines;

fn binary_search(min: i32, max: i32, code: &str) -> i32 {
    let mut s = min;
    let mut e = max;

    for c in code.bytes() {
        match c {
            b'F' | b'L' => e = (s + e) / 2,
            b'B' | b'R' => s = (s + e) / 2 + 1,
            _ => panic!("Unrecognized selector."),
        };
    }

    assert_eq!(s, e);
    return s;
}

fn part_1(seatcodes: Lines) -> i32 {
    let mut highest = 0;
    for seat in seatcodes {
        let row = binary_search(0, 127, &seat[..7]);
        let col = binary_search(0, 7, &seat[7..]);
        let id = row * 8 + col;
        if id > highest {
            highest = id;
        }
    }
    return highest;
}

fn part_2(seatcodes: Lines) -> i32 {
    const NUM_SEAT: usize = 128 * 8;
    let mut ids: [i32; NUM_SEAT] = [0; NUM_SEAT];

    for seat in seatcodes {
        let row = binary_search(0, 127, &seat[..7]);
        let col = binary_search(0, 7, &seat[7..]);
        let id = row * 8 + col;
        ids[id as usize] = 1;
    }

    for i in 8..(127 * 8) {
        if ids[i] == 0 && (ids[i - 1] == 1 && ids[i + 1] == 1) {
            return i as i32;
        }
    }

    panic!("No seat id found.");
}

fn main() {
    let input = include_bytes!("day005.input");

    let seat_lines = from_utf8(input).unwrap().lines();
    println!("part 1, highest id: {}", part_1(seat_lines.clone()));

    println!("part 2, seat id: {}", part_2(seat_lines));
}

#[test]
fn test_binary_select() {
    let res = binary_search(0, 3, "FB");
    assert_eq!(res, 1);
}
