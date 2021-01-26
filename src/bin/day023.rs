use std::vec::Vec;

fn part_1(mut cups: Vec<u8>) -> String {
    let cups_n = cups.len();
    let mut curr_pos = 0;
    for _ in 0..100 {
        // println!("cups: {:?}", cups);

        let curr_label = cups[curr_pos];

        let pick_0 = cups[(curr_pos + 1) % cups_n];
        let pick_1 = cups[(curr_pos + 2) % cups_n];
        let pick_2 = cups[(curr_pos + 3) % cups_n];
        cups.remove(cups.iter().position(|&x| x == pick_0).unwrap());
        cups.remove(cups.iter().position(|&x| x == pick_1).unwrap());
        cups.remove(cups.iter().position(|&x| x == pick_2).unwrap());

        let mut dest = curr_label;
        loop {
            dest -= 1;
            if dest == 0 {
                dest = *cups.iter().max().unwrap();
            }
            if dest != pick_0 && dest != pick_1 && dest != pick_2 {
                break;
            }
        }
        // println!("curr: {}, picked: {}, {}, {}, dest: {}", curr_label, pick_0, pick_1, pick_2, dest);

        let dest_pos = cups.iter().position(|&x| x == dest).unwrap();
        cups.insert(dest_pos + 1, pick_0);
        cups.insert(dest_pos + 2, pick_1);
        cups.insert(dest_pos + 3, pick_2);

        curr_pos = (cups.iter().position(|&x| x == curr_label).unwrap() + 1) % cups_n;
    }

    let label_1_pos = cups.iter().position(|&x| x == 1).unwrap();
    let mut res: String = "".to_string();
    let mut pos = label_1_pos + 1;
    while pos != label_1_pos {
        res += &cups[pos].to_string();
        pos = (pos + 1) % cups_n;
    }
    return res;
}

fn main() {
    let cups = vec![7, 3, 9, 8, 6, 2, 5, 4, 1];
    println!("Part 1: {}", part_1(cups.clone()));
}
