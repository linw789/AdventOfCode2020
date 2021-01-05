use std::str::from_utf8;
use std::vec::Vec;

fn part_1(instructions: &[(u8, i32)]) -> i32 {
    // Assume East is positve x direction, North is positive y direction.
    let mut x = 0;
    let mut y = 0;

    // Left turn rotates counter-clock wise.
    let dirs = [b'E', b'N', b'W', b'S'];
    let mut curr_dir = 0;

    for (action, val) in instructions {
        match *action {
            b'N' => {
                y += val;
            }
            b'S' => {
                y -= val;
            }
            b'E' => {
                x += val;
            }
            b'W' => {
                x -= val;
            }
            b'L' => {
                // `%` operator calculates reminder, but not modulo which is what we want.
                curr_dir = ((curr_dir + (val / 90)) % 4 + 4) % 4;
            }
            b'R' => {
                curr_dir = ((curr_dir - (val / 90)) % 4 + 4) % 4;
            }
            b'F' => match &dirs[curr_dir as usize] {
                b'N' => {
                    y += val;
                }
                b'S' => {
                    y -= val;
                }
                b'E' => {
                    x += val;
                }
                b'W' => {
                    x -= val;
                }
                _ => panic!("unrecognized action."),
            },
            _ => panic!("unrecognized action."),
        }
    }

    return x.abs() + y.abs();
}

fn part_2(insts: &[(u8, i32)]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut wp_x = 10; // East
    let mut wp_y = 1; // North

    for (act, val) in insts {
        match *act {
            b'N' => {
                wp_y += val;
            }
            b'S' => {
                wp_y -= val;
            }
            b'E' => {
                wp_x += val;
            }
            b'W' => {
                wp_x -= val;
            }
            b'L' => match *val {
                90 => {
                    std::mem::swap(&mut wp_x, &mut wp_y);
                    wp_x *= -1;
                }
                180 => {
                    wp_x *= -1;
                    wp_y *= -1;
                }
                270 => {
                    std::mem::swap(&mut wp_x, &mut wp_y);
                    wp_y *= -1;
                }
                _ => panic!("bad rotation: {}", val),
            },
            b'R' => match *val {
                90 => {
                    std::mem::swap(&mut wp_x, &mut wp_y);
                    wp_y *= -1;
                }
                180 => {
                    wp_x *= -1;
                    wp_y *= -1;
                }
                270 => {
                    std::mem::swap(&mut wp_x, &mut wp_y);
                    wp_x *= -1;
                }
                _ => panic!("bad rotation: {}", val),
            },
            b'F' => {
                x += wp_x * val;
                y += wp_y * val;
            }
            _ => panic!("unrecognized action."),
        }
    }

    return x.abs() + y.abs();
}

fn main() {
    let input = include_bytes!("day012.input");
    let instructions: Vec<(u8, i32)> = from_utf8(input)
        .unwrap()
        .lines()
        .map(|s| {
            let action = s[0..1].to_owned().into_bytes()[0];
            let val = s[1..].parse::<i32>().unwrap();
            return (action, val);
        })
        .collect();

    println!("Part 1, manhattan distance: {}", part_1(&instructions));
    println!("Part 2, manhattan distance: {}", part_2(&instructions));
}
