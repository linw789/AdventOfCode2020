use std::str::from_utf8;
use std::vec::Vec;
use intbits::Bits;

fn get_tile_boundaries(tile: u128, width: usize) -> (u16, u16, u16, u16) {
    let (mut e, mut w, mut s, mut n): (u16, u16, u16, u16) = (0, 0, 0, 0);
    for y in 0..width {
        e.set_bit(y, tile.bit(y * width + width - 1));
        w.set_bit(y, tile.bit(y * width));
    }
    for x in 0..width {
        s.set_bit(x, tile.bit(width * (width - 1) + x));
        n.set_bit(x, tile.bit(x));
    }
    return (e, w, s, n);
}

fn get_flipped_boundaries(tile: u128, width: usize) -> (u16, u16, u16, u16) {
    let (mut ef, mut wf, mut sf, mut nf): (u16, u16, u16, u16) = (0, 0, 0, 0);
    for y in 0..width {
        ef.set_bit(width - y - 1, tile.bit(y * width + width - 1));
        wf.set_bit(width - y - 1, tile.bit(y * width));
    }
    for x in 0..width {
        sf.set_bit(width - x - 1, tile.bit((width - 1) * width + x));
        nf.set_bit(width - x - 1, tile.bit(x));
    }
    return (ef, wf, sf, nf);
}

fn part_1(tiles: &Vec<(u32, u128)>) -> Vec< {
    let mut matches_per_tile = vec![0; tiles.len()];
    for (index, (tile_id, tile)) in tiles.iter().enumerate() {
        let (e, w, s, n) = get_tile_boundaries(*tile, 10);
        let (ef, wf, sf, nf) = get_flipped_boundaries(*tile, 10);

        for (other_tile_id, other_tile) in tiles.iter() {
            if tile_id == other_tile_id {
                continue;
            }

            let (oe, ow, os, on) = get_tile_boundaries(*other_tile, 10);
            let (oef, owf, osf, onf) = get_flipped_boundaries(*other_tile, 10);

            for b in &[e, w, s, n, ef, wf, sf, nf] {
                let mut matched = false;
                for ob in &[oe, ow, os, on, oef, owf, osf, onf] {
                    if b == ob {
                        matches_per_tile[index] += 1;
                        matched = true;
                        break;
                    }
                }
                if matched {
                    break;
                }
            }
        }
    }

    return tiles.iter().enumerate().fold(1, |mut mul, (index, (tile_id, _))| {
        if matches_per_tile[index] == 2 {
            mul *= *tile_id as u64;
        }
        return mul;
    });
}

struct Tile {
    data: u128,
    width: usize,
}

impl Tile {
    pub fn get_boundaries(&self) -> [u16; 8] {
        let mut e = 0;
        let mut w = 0;
        let mut s = 0;
        let mut n = 0;
        for y in 0..self.width {
            e.set_bit(y, self.data.bit(y * width + width - 1));
            w.set_bit(y, self.data.bit(y * width));
        }
        for x in 0..self.width {
            s.set_bit(x, tile.bit(width * (width - 1) + x));
            n.set_bit(x, tile.bit(x));
        }

        // boundaries flipped
        let mut ef = 0; 
        let mut wf = 0;
        let mut sf = 0; 
        let mut nf = 0;
        for y in 0..self.width {
            ef.set_bit(width - y - 1, self.data.bit(y * width + width - 1));
            wf.set_bit(width - y - 1, self.data.bit(y * width));
        }
        for x in 0..self.width {
            sf.set_bit(width - x - 1, self.data.bit((width - 1) * width + x));
            nf.set_bit(width - x - 1, self.data.bit(x));
        }

        return [e, w, s, n, ef, wf, sf, nf];
    }
}

fn main() {
    let input = include_bytes!("day020.input");
    let mut lines = from_utf8(input).unwrap().lines();

    let mut tiles: Vec<(u32, u128)> = Vec::new();
    loop {
        let line = lines.next();
        if line == None {
            break;
        }

        if let Some(title) = line.unwrap().strip_prefix("Tile ") {
            let tile_id = title.strip_suffix(':').unwrap().parse::<u32>().unwrap();

            let mut tile: u128 = 0;
            let mut index = 0;
            loop {
                match lines.next() {
                    Some(line) => {
                        if line.is_empty() {
                            break;
                        }
                        for c in line.chars() {
                            match c {
                                '#' => {
                                    tile.set_bit(index, true);
                                }
                                '.' => {
                                    tile.set_bit(index, false);
                                }
                                _ => {
                                    panic!("Invalid tile character.");
                                }
                            }
                            index += 1;
                        }
                    }
                    None => {
                        break;
                    }
                };
            }
            tiles.push((tile_id, tile));
        } else {
            panic!("Invalid line.");
        }
    }

    println!("Part 1: {}", part_1(&tiles));
}
