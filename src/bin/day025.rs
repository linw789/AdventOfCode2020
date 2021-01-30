fn main() {
    let subject_n: u64 = 7;
    let divider: u64 = 20201227;

    let door_pub = 9717666;
    let card_pub = 20089533;

    let mut door_loop_size = 0;
    let mut val = 1;
    for size in 1.. {
        val = val * subject_n;
        val = val % divider;
        if val == door_pub {
            door_loop_size = size;
            break;
        }
        /*
        if size % 1000 == 0 {
            println!("size: {}k", size / 1000);
        }
        */
    }
    println!("Door loop size: {}", door_loop_size);

    let mut card_loop_size = 0;
    let mut val = 1;
    for size in 1.. {
        val = val * subject_n;
        val = val % divider;
        if val == card_pub {
            card_loop_size = size;
            break;
        }
        /*
        if size % 1000 == 0 {
            println!("size: {}k", size / 1000);
        }
        */
    }
    println!("Card loop size: {}", card_loop_size);

    let mut val = 1;
    for _ in 0..card_loop_size {
        val = val * door_pub;
        val = val % divider;
    }
    println!("Encryption key: {}", val);

    let mut val = 1;
    for _ in 0..door_loop_size {
        val = val * card_pub;
        val = val % divider;
    }
    println!("Encryption key: {}", val);
}
