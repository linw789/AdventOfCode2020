use std::str::from_utf8;
use std::vec::Vec;
use intbits::Bits;
use bit_reverse::BitwiseReverse;
// use bitvec::prelude::*;

const TILE_WIDTH: usize = 10;

struct Tile {
    rows: [u16; TILE_WIDTH],
}

impl Tile {
    pub fn new(rows: [u16; TILE_WIDTH]) -> Self {
        return Self {
            rows
        };
    }

    pub fn set_row(&mut self, row_index: usize, row: u16) {
        self.rows[row_index] = row;
    }

    pub fn get_borders(&self) -> (u16, u16, u16, u16) {
        // south 
        let s = self.rows[0];
        // east 
        let mut e: u16 = 0;
        for (i, row) in self.rows.iter().enumerate() {
            e.set_bit(i, row.bit(0));
        }
        // north 
        let n = self.rows[TILE_WIDTH - 1];
        // west 
        let mut w: u16 = 0;
        for (i, row) in self.rows.iter().enumerate() {
            w.set_bit(i, row.bit(TILE_WIDTH - 1));
        }

        return (s, e, n, w);
    }

    pub fn get_borders_flipped(&self) -> (u16, u16, u16, u16) {
        let (s, e, n, w) = self.get_borders();
        let mut sf = s.swap_bits();
        sf >>= 16 - TILE_WIDTH;

        let mut ef = e.swap_bits();
        ef >>= 16 - TILE_WIDTH;

        let mut nf = n.swap_bits();
        nf >>= 16 - TILE_WIDTH;

        let mut wf = w.swap_bits();
        wf >>= 16 - TILE_WIDTH;

        return (sf, ef, nf, wf);
    }
}

fn main() {
    let input = include_bytes!("day020.input");
    let mut lines = from_utf8(input).unwrap().lines();

    let mut tiles: Vec<(u32, Tile)> = Vec::new();
    loop {
        let line = lines.next();
        if line == None {
            break;
        }

        let line = line.unwrap();
        if let Some(id) = line.strip_prefix("Tile ") {
            let id = id.trim().strip_suffix(":").unwrap().parse::<u32>().unwrap();
            let mut tile = Tile::new([0; TILE_WIDTH]);
            let mut row_i = 0;
            loop {
                let line = lines.next();
                if line == None {
                    break;
                }
                let line = line.unwrap();
                if line.is_empty() {
                    break;
                }
                let row: u16 = line
                    .chars()
                    .rev()
                    .enumerate()
                    .fold(0, |mut acc, (i, c)| {
                        if c == '#' {
                            acc.set_bit(i, true);
                        }
                        acc
                    });
                tile.set_row(row_i, row);
                row_i += 1;
            }
            tiles.push((id, tile));
        }
    }

    let mut neighbours = Vec::new();

    for (id, tile) in &tiles {
        let (s, e, n, w) = tile.get_borders();
        let (sf, ef, nf, wf) = tile.get_borders_flipped();
        let borders = [s, e, n, w, sf, ef, nf, wf];
        let mut neighbour_cnt = 0;

        for (other_id, other_tile) in &tiles {
            if id == other_id {
                continue;
            }

            let (os, oe, on, ow) = other_tile.get_borders();
            let (osf, oef, onf, owf) = other_tile.get_borders_flipped();
            let other_borders = [os, oe, on, ow, osf, oef, onf, owf];

            let mut is_neighbour = false;
            for b in borders.iter() {
                for ob in other_borders.iter() {
                    if b == ob {
                        is_neighbour = true;
                        break;
                    }
                }
                if is_neighbour {
                    break;
                }
            }

            if is_neighbour {
                neighbour_cnt += 1;
            }
        }
        neighbours.push(neighbour_cnt);
    }

    let mut res: usize = 1;
    for (i, cnt) in neighbours.iter().enumerate() {
        if *cnt == 2 {
            let (id, _) = tiles[i];
            res *= id as usize;
        }
    }

    println!("Part 1: {}", res);
}
