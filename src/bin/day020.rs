use std::str::from_utf8;
use std::vec::Vec;
use intbits::Bits;
use bit_reverse::BitwiseReverse;
use std::cell::RefCell;

const TILE_WIDTH: usize = 10;

struct Tile {
    id: u32,
    rows: [u16; TILE_WIDTH],
    // north, west, south, east
    adjacent: RefCell<[Option<u32>; 4]>,
}

impl Tile {
    pub fn new(id: u32, rows: [u16; TILE_WIDTH]) -> Self {
        return Self {
            id,
            rows,
            adjacent: RefCell::new([None; 4]),
        };
    }

    pub fn set_row(&mut self, row_index: usize, row: u16) {
        self.rows[row_index] = row;
    }

    pub fn get_row(&self, row_index: usize) -> u16 {
        return self.rows[row_index];
    }

    pub fn set_adjacent(&self, dir: usize, tile_id: u32) {
        let mut adjacent = self.adjacent.borrow_mut();

        assert_eq!(adjacent[dir], None, 
                   "Both {:?} and {} are adjacent to {}", 
                   adjacent[dir], tile_id, self.id);

        adjacent[dir] = Some(tile_id);
    }

    pub fn adjacent_count(&self) -> usize {
        return self.adjacent.borrow().iter().fold(0, |acc, adj| {
            if *adj != None {
                acc + 1
            } else {
                acc 
            }
        });
    }

    pub fn get_borders(&self) -> (u16, u16, u16, u16) {
        // north 
        let n = self.rows[TILE_WIDTH - 1];
        // west 
        let mut w: u16 = 0;
        for (i, row) in self.rows.iter().enumerate() {
            w.set_bit(i, row.bit(TILE_WIDTH - 1));
        }
        // south 
        let s = self.rows[0];
        // east 
        let mut e: u16 = 0;
        for (i, row) in self.rows.iter().enumerate() {
            e.set_bit(i, row.bit(0));
        }
        return (n, w, s, e);
    }

    pub fn get_borders_flipped(&self) -> (u16, u16, u16, u16) {
        let (n, w, s, e) = self.get_borders();

        let mut nf = n.swap_bits();
        nf >>= 16 - TILE_WIDTH;

        let mut wf = w.swap_bits();
        wf >>= 16 - TILE_WIDTH;

        let mut sf = s.swap_bits();
        sf >>= 16 - TILE_WIDTH;

        let mut ef = e.swap_bits();
        ef >>= 16 - TILE_WIDTH;

        return (nf, wf, sf, ef);
    }

    pub fn flip_vertically(&mut self) {
        for r in 0..(TILE_WIDTH / 2) {
            self.rows.swap(r, TILE_WIDTH - 1 - r);
        }
        self.adjacent.borrow_mut().swap(0, 2);
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.adjacent.borrow_mut().rotate_left(1);
    }
}

fn main() {
    let input = include_bytes!("day020.input");
    let mut lines = from_utf8(input).unwrap().lines();

    let mut tiles: Vec<Tile> = Vec::new();
    loop {
        let line = lines.next();
        if line == None {
            break;
        }

        let line = line.unwrap();
        if let Some(id) = line.strip_prefix("Tile ") {
            let id = id.trim().strip_suffix(":").unwrap().parse::<u32>().unwrap();
            let mut tile = Tile::new(id, [0; TILE_WIDTH]);
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
            tiles.push(tile);
        }
    }

    for tile in &tiles {
        let (n, w, s, e) = tile.get_borders();
        let borders = [n, w, s, e];

        for other_tile in &tiles {
            if tile.id == other_tile.id {
                continue;
            }

            let (on, ow, os, oe) = other_tile.get_borders();
            let (onf, owf, osf, oef) = other_tile.get_borders_flipped();
            let other_borders = [on, ow, os, oe, onf, owf, osf, oef];

            let mut is_adjacent = false;
            for i in 0..borders.len() {
                for ob in other_borders.iter() {
                    if borders[i] == *ob {
                        is_adjacent = true;
                        tile.set_adjacent(i, other_tile.id);
                        break;
                    }
                }
                if is_adjacent {
                    break;
                }
            }
        }
    }

    let mut res: usize = 1;
    for tile in tiles.iter() {
        if tile.adjacent_count() == 2 {
            res *= tile.id as usize;
        }
    }

    println!("Part 1: {}", res);
}
