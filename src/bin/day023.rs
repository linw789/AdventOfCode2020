fn play(cups: &mut [u32], start: u32, max: u32, moves: usize) {
    let mut curr = start as usize;
    for _ in 0..moves {
        let pick1 = cups[curr] as usize;
        let pick2 = cups[pick1] as usize;
        let pick3 = cups[pick2] as usize;

        let mut dest = curr - 1;
        loop {
            if dest == 0 {
                dest = max as usize;
            }
            if dest != pick1 && dest != pick2 && dest != pick3 {
                break;
            }
            dest -= 1;
        }

        // Break the link between `curr` and its next 3 neighbours.
        cups[curr] = cups[pick3];
        // Link `pick3`'s next to `dest`'s next.
        cups[pick3] = cups[dest];
        // Link `dest`'s next to `pick1`.
        cups[dest] = pick1 as u32;

        curr = cups[curr] as usize;
    }
}

fn part_1(cups: &[u32]) -> String {
    let mut cups_moved = cups.to_vec();
    play(&mut cups_moved, 7, 9, 100);

    let mut res: String = "".to_string();
    let mut label = cups_moved[1];
    loop {
        res += &label.to_string();
        label = cups_moved[label as usize];
        if label as usize == 1 {
            break;
        }
    }
    return res;
}

fn part_2(cups: &[u32]) -> usize {
    let mut cups_moved = cups.to_vec();
    play(&mut cups_moved, 7, 1000_000, 10_000_000);

    let next1 = cups_moved[1] as usize;
    let next2 = cups_moved[next1 as usize] as usize;
    return next1 * next2;
}

fn main() {
    // let input = [3, 8, 9, 1, 2, 5, 4, 6, 7];
    let input = [7, 3, 9, 8, 6, 2, 5, 4, 1];

    let mut cups: Vec<u32> = Vec::new();
    cups.resize(input.len() + 1, 0);

    // Because labels are unique, we can use labels themselves as indices to store their next
    // neighbours.
    let start = input[0];
    let mut last = start as usize;
    for label in input.iter().skip(1) {
        let next = *label;
        cups[last as usize] = next;
        last = next as usize;
    }
    cups[last] = start;

    // println!("Cups: {:?}", cups.iter().enumerate().collect::<Vec<_>>());

    println!("Part 1: {}", part_1(&cups));

    cups.resize(1000_000 + 1, 0);
    for i in 10..=1000_000 {
        cups[last] = i;
        last = i as usize;
    }
    cups[last] = start;

    println!("Part 2: {}", part_2(&cups));
}
