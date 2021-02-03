use std::cell::RefCell;
use std::collections::HashMap;
use std::str::from_utf8;
use std::vec::Vec;

#[derive(Clone)]
struct Tile {
    data: Vec<u8>,
    width: usize,
    height: usize,
    // north, west, south, east
    neighbours: RefCell<[Option<u32>; 4]>,
}

impl Tile {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::new();
        data.resize(width * height, 0);
        return Self {
            data,
            width,
            height,
            neighbours: RefCell::new([None; 4]),
        };
    }

    pub fn set_row(&mut self, row_index: usize, row: &[u8]) {
        assert_eq!(self.width, row.len());
        let start = row_index * self.width;
        for i in 0..self.width {
            self.data[start + i] = row[i];
        }
    }

    pub fn row(&self, row_index: usize) -> Vec<u8> {
        let start = row_index * self.width;
        return self.data[start..(start + self.width)].to_vec();
    }

    pub fn col(&self, col_index: usize) -> Vec<u8> {
        assert!(col_index < self.height);
        let mut col = Vec::new();
        col.resize(self.height, 0);
        for i in 0..self.height {
            col[i] = self.data[i * self.width + col_index];
        }
        return col;
    }

    pub fn borders(&self) -> Vec<Vec<u8>> {
        let n = self.row(0);
        let w = self.col(0);
        let s = self.row(self.height - 1);
        let e = self.col(self.width - 1);
        return vec![n, w, s, e];
    }

    pub fn set_neighbour(&self, dir: usize, tile_id: u32) {
        {
            assert_eq!(self.neighbours.borrow()[dir], None);
        }
        self.neighbours.borrow_mut()[dir] = Some(tile_id);
    }

    pub fn neighbour_count(&self) -> usize {
        let neighbours = self.neighbours.borrow();
        let mut cnt = 0;
        for neighbour in neighbours.iter() {
            if *neighbour != None {
                cnt += 1;
            }
        }
        return cnt;
    }

    pub fn flip_vertically(&mut self) {
        for h in 0..(self.height / 2) {
            for i in 0..self.width {
                self.data
                    .swap(h * self.width + i, (self.height - 1 - h) * self.width + i);
            }
        }

        let mut neighbours = self.neighbours.borrow_mut();
        neighbours.swap(0, 2);
    }

    pub fn rotate_counter_clockwise(&mut self) {
        assert_eq!(self.width, self.height);
        let mut rotated = Vec::new();
        rotated.resize(self.width * self.height, 0);
        for h in 0..self.height {
            let col = self.col(self.width - 1 - h);
            for i in 0..self.width {
                rotated[h * self.width + i] = col[i];
            }
        }
        self.data = rotated;

        self.neighbours.borrow_mut().rotate_right(1);
    }

    pub fn add_tile(&mut self, tile: &Tile, x: usize, y: usize) {
        let w_noborder = tile.width - 2;
        let h_noborder = tile.height - 2;

        for h in 0..h_noborder {
            let start = ((y * h_noborder) + h) * self.width + x * w_noborder;
            for w in 0..w_noborder {
                self.data[start + w] = tile.data[(h + 1) * tile.width + (w + 1)];
            }
        }
    }

    pub fn fit_border(&mut self, dir: usize, border: &[u8]) -> bool {
        assert_eq!(self.height, border.len());
        let mut matched = false;
        for _flip in 0..2 {
            for _rot in 0..4 {
                let borders = self.borders();
                if borders[dir] == border {
                    matched = true;
                    break;
                }
                self.rotate_counter_clockwise();
            }
            if matched == false {
                self.flip_vertically();
            }
        }
        return matched;
    }

    pub fn find_monster(&self, monster: &Tile) -> usize {
        let mut monster_cnt = 0;
        for y in 0..(self.height - monster.height + 1) {
            for x in 0..(self.width - monster.width + 1) {
                let mut monster_found = true;
                for my in 0..monster.height {
                    for mx in 0..monster.width {
                        if monster.data[my * monster.width + mx] == 1
                            && self.data[(y + my) * self.width + (x + mx)] != 1
                        {
                            monster_found = false;
                            break;
                        }
                    }
                    if !monster_found {
                        break;
                    }
                }
                if monster_found {
                    monster_cnt += 1;
                }
            }
        }
        return monster_cnt;
    }
}

const TILE_WIDTH: usize = 10;

fn main() {
    let input = include_bytes!("day020.input");
    let mut lines = from_utf8(input).unwrap().lines();

    let mut tiles: HashMap<u32, Tile> = HashMap::new();
    loop {
        let line = lines.next();
        if line == None {
            break;
        }

        let line = line.unwrap();
        if let Some(id) = line.strip_prefix("Tile ") {
            let id = id.trim().strip_suffix(":").unwrap().parse::<u32>().unwrap();
            let mut tile = Tile::new(TILE_WIDTH, TILE_WIDTH);
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
                let row: Vec<u8> = line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect();
                tile.set_row(row_i, &row);
                row_i += 1;
            }
            tiles.insert(id, tile);
        }
    }

    for (id, tile) in &tiles {
        let borders = tile.borders();

        for (other_id, other_tile) in &tiles {
            if id == other_id {
                continue;
            }
            let mut other_borders = other_tile.borders();

            let mut is_adjacent = false;
            for i in 0..borders.len() {
                for ob in other_borders.iter_mut() {
                    if borders[i] == *ob {
                        is_adjacent = true;
                        tile.set_neighbour(i, *other_id);
                        break;
                    }
                    ob.reverse();
                    if borders[i] == *ob {
                        is_adjacent = true;
                        tile.set_neighbour(i, *other_id);
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
    for (id, tile) in tiles.iter() {
        if tile.neighbour_count() == 2 {
            // println!("neighbours: {:?}", tile.neighbours.borrow());
            res *= *id as usize;
        }
    }

    println!("Part 1: {}\n", res);

    let topleft_id = 2593;
    /*
    for (id, tile) in tiles.iter() {
        if tile.neighbour_count() == 2 {
            topleft_id = *id;
            break;
        }
    }
    */
    println!("topleft tile id: {}", topleft_id);

    // Fit to top-left corner.
    let topleft_tile = tiles.get_mut(&topleft_id).unwrap();
    let mut fit = false;
    for _flip in 0..2 {
        for _rot in 0..4 {
            {
                let neighbours = topleft_tile.neighbours.borrow();
                if neighbours[0] == None
                    && neighbours[1] == None
                    && neighbours[2] != None
                    && neighbours[3] != None
                {
                    fit = true;
                    break;
                }
            }
            topleft_tile.rotate_counter_clockwise();
        }
        if fit == false {
            topleft_tile.flip_vertically();
        }
    }
    assert_eq!(fit, true);

    let mut image = Tile::new(12 * (TILE_WIDTH - 2), 12 * (TILE_WIDTH - 2));

    let mut leftmost_id = topleft_id;
    for y in 0.. {
        let leftmost_tile = tiles[&leftmost_id].clone();

        let mut tile_id = leftmost_id;
        for x in 0.. {
            let tile = tiles.remove(&tile_id).unwrap();
            image.add_tile(&tile, x, y);
            let (east_border, east_neighbour_id) = {
                if let Some(id) = tile.neighbours.borrow()[3] {
                    let borders = tile.borders();
                    (borders[3].clone(), id)
                } else {
                    break;
                }
            };
            let east_tile = tiles.get_mut(&east_neighbour_id).unwrap();
            // Fit the west border of the east neighbour tile.
            assert!(east_tile.fit_border(1, &east_border));
            tile_id = east_neighbour_id;
        }

        // Next row.
        let (south_border, south_neighbour_id) = {
            if let Some(id) = leftmost_tile.neighbours.borrow()[2] {
                let borders = leftmost_tile.borders();
                (borders[2].clone(), id)
            } else {
                break;
            }
        };
        let south_tile = &mut tiles.get_mut(&south_neighbour_id).unwrap();
        assert!(south_tile.fit_border(0, &south_border));
        leftmost_id = south_neighbour_id;
    }

    // println!("Image: {:?}", &image.data);

    let monster_str = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
    let mut monster = Tile::new(20, 3);
    for (i, line) in monster_str.lines().enumerate() {
        monster.set_row(
            i,
            &line
                .chars()
                .map(|x| if x == '#' { 1 } else { 0 })
                .collect::<Vec<u8>>(),
        );
    }
    // println!("monster: {:?}", monster.data);

    let mut monster_cnt = 0;
    for _flip in 0..2 {
        for _rot in 0..4 {
            monster_cnt = image.find_monster(&monster);
            if monster_cnt > 0 {
                break;
            }
            image.rotate_counter_clockwise();
        }
        if monster_cnt > 0 {
            break;
        }
        image.flip_vertically();
    }

    println!("Monster found: {}", monster_cnt);

    let pound_cnt_image = image
        .data
        .iter()
        .fold(0, |acc, &x| if x == 1 { acc + 1 } else { acc });
    let pound_cnt_monster = monster
        .data
        .iter()
        .fold(0, |acc, &x| if x == 1 { acc + 1 } else { acc });

    println!(
        "roughness: {}",
        pound_cnt_image - monster_cnt * pound_cnt_monster
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tile_set_get_row() {
        let mut tile = Tile::new(4, 4);
        let row = [1, 2, 3, 4];
        tile.set_row(2, &row);
        assert_eq!(tile.row(0), &[0, 0, 0, 0]);
        assert_eq!(tile.row(1), &[0, 0, 0, 0]);
        assert_eq!(tile.row(2), &row);
        assert_eq!(tile.row(3), &[0, 0, 0, 0]);
    }

    #[test]
    fn tile_column() {
        let mut tile = Tile::new(4, 4);
        tile.set_row(0, &[1, 2, 3, 4]);
        tile.set_row(1, &[5, 6, 7, 8]);
        tile.set_row(2, &[9, 10, 11, 12]);
        tile.set_row(3, &[13, 14, 15, 16]);
        assert_eq!(tile.col(0), &[1, 5, 9, 13]);
        assert_eq!(tile.col(1), &[2, 6, 10, 14]);
        assert_eq!(tile.col(2), &[3, 7, 11, 15]);
        assert_eq!(tile.col(3), &[4, 8, 12, 16]);
    }

    #[test]
    fn tile_flip_vertically() {
        let mut tile = Tile::new(4, 4);
        tile.set_row(0, &[1, 2, 3, 4]);
        tile.set_row(1, &[5, 6, 7, 8]);
        tile.set_row(2, &[9, 10, 11, 12]);
        tile.set_row(3, &[13, 14, 15, 16]);

        tile.set_neighbour(0, 1);
        tile.set_neighbour(1, 2);
        tile.set_neighbour(2, 3);
        tile.set_neighbour(3, 4);

        tile.flip_vertically();
        assert_eq!(tile.row(0), &[13, 14, 15, 16]);
        assert_eq!(tile.row(1), &[9, 10, 11, 12]);
        assert_eq!(tile.row(2), &[5, 6, 7, 8]);
        assert_eq!(tile.row(3), &[1, 2, 3, 4]);

        assert_eq!(
            *tile.neighbours.borrow(),
            [Some(3), Some(2), Some(1), Some(4)]
        );
    }

    #[test]
    fn tile_rotate_counter_clockwise() {
        let mut tile = Tile::new(4, 4);
        tile.set_row(0, &[1, 2, 3, 4]);
        tile.set_row(1, &[5, 6, 7, 8]);
        tile.set_row(2, &[9, 10, 11, 12]);
        tile.set_row(3, &[13, 14, 15, 16]);

        tile.set_neighbour(0, 1);
        tile.set_neighbour(1, 2);
        tile.set_neighbour(2, 3);
        tile.set_neighbour(3, 4);

        tile.rotate_counter_clockwise();

        assert_eq!(tile.row(0), &[4, 8, 12, 16]);
        assert_eq!(tile.row(1), &[3, 7, 11, 15]);
        assert_eq!(tile.row(2), &[2, 6, 10, 14]);
        assert_eq!(tile.row(3), &[1, 5, 9, 13]);

        assert_eq!(
            *tile.neighbours.borrow(),
            [Some(4), Some(1), Some(2), Some(3)]
        );

        tile.rotate_counter_clockwise();

        assert_eq!(tile.row(0), &[16, 15, 14, 13]);
        assert_eq!(tile.row(1), &[12, 11, 10, 9]);
        assert_eq!(tile.row(2), &[8, 7, 6, 5]);
        assert_eq!(tile.row(3), &[4, 3, 2, 1]);

        assert_eq!(
            *tile.neighbours.borrow(),
            [Some(3), Some(4), Some(1), Some(2)]
        );

        tile.rotate_counter_clockwise();

        assert_eq!(tile.row(0), &[13, 9, 5, 1]);
        assert_eq!(tile.row(1), &[14, 10, 6, 2]);
        assert_eq!(tile.row(2), &[15, 11, 7, 3]);
        assert_eq!(tile.row(3), &[16, 12, 8, 4]);

        assert_eq!(
            *tile.neighbours.borrow(),
            [Some(2), Some(3), Some(4), Some(1)]
        );
    }

    #[test]
    fn tile_add_tile() {
        let mut tile0 = Tile::new(4, 4);
        tile0.set_row(0, &[1, 2, 3, 4]);
        tile0.set_row(1, &[5, 6, 7, 8]);
        tile0.set_row(2, &[9, 10, 11, 12]);
        tile0.set_row(3, &[13, 14, 15, 16]);

        let mut tile1 = Tile::new(4, 4);
        tile1.set_row(
            0,
            &tile0.row(0).into_iter().map(|x| 2 * x).collect::<Vec<u8>>(),
        );
        tile1.set_row(
            1,
            &tile0.row(1).into_iter().map(|x| 2 * x).collect::<Vec<u8>>(),
        );
        tile1.set_row(
            2,
            &tile0.row(2).into_iter().map(|x| 2 * x).collect::<Vec<u8>>(),
        );
        tile1.set_row(
            3,
            &tile0.row(3).into_iter().map(|x| 2 * x).collect::<Vec<u8>>(),
        );
        assert_eq!(tile1.row(0), &[2, 4, 6, 8]);
        assert_eq!(tile1.row(1), &[10, 12, 14, 16]);
        assert_eq!(tile1.row(2), &[18, 20, 22, 24]);
        assert_eq!(tile1.row(3), &[26, 28, 30, 32]);

        let mut tile2 = Tile::new(4, 4);
        tile2.set_row(
            0,
            &tile0.row(0).into_iter().map(|x| 3 * x).collect::<Vec<u8>>(),
        );
        tile2.set_row(
            1,
            &tile0.row(1).into_iter().map(|x| 3 * x).collect::<Vec<u8>>(),
        );
        tile2.set_row(
            2,
            &tile0.row(2).into_iter().map(|x| 3 * x).collect::<Vec<u8>>(),
        );
        tile2.set_row(
            3,
            &tile0.row(3).into_iter().map(|x| 3 * x).collect::<Vec<u8>>(),
        );

        let mut tile3 = Tile::new(4, 4);
        tile3.set_row(
            0,
            &tile0.row(0).into_iter().map(|x| 4 * x).collect::<Vec<u8>>(),
        );
        tile3.set_row(
            1,
            &tile0.row(1).into_iter().map(|x| 4 * x).collect::<Vec<u8>>(),
        );
        tile3.set_row(
            2,
            &tile0.row(2).into_iter().map(|x| 4 * x).collect::<Vec<u8>>(),
        );
        tile3.set_row(
            3,
            &tile0.row(3).into_iter().map(|x| 4 * x).collect::<Vec<u8>>(),
        );

        let mut image = Tile::new(4, 4);
        image.add_tile(&tile0, 0, 0);
        image.add_tile(&tile1, 0, 1);
        image.add_tile(&tile2, 1, 0);
        image.add_tile(&tile3, 1, 1);

        assert_eq!(image.row(0), &[6, 7, 18, 21]);
        assert_eq!(image.row(1), &[10, 11, 30, 33]);
        assert_eq!(image.row(2), &[12, 14, 24, 28]);
        assert_eq!(image.row(3), &[20, 22, 40, 44]);
    }

    #[test]
    fn tile_fit_border() {
        let mut tile = Tile::new(4, 4);
        tile.set_row(0, &[1, 2, 3, 4]);
        tile.set_row(1, &[5, 6, 7, 8]);
        tile.set_row(2, &[9, 10, 11, 12]);
        tile.set_row(3, &[13, 14, 15, 16]);

        tile.set_neighbour(0, 1);
        tile.set_neighbour(1, 2);
        tile.set_neighbour(2, 3);
        tile.set_neighbour(3, 4);

        let mut tile1 = tile.clone();
        tile1.fit_border(1, &[1, 5, 9, 13]);

        assert_eq!(tile1.row(0), &[1, 2, 3, 4]);
        assert_eq!(tile1.row(1), &[5, 6, 7, 8]);
        assert_eq!(tile1.row(2), &[9, 10, 11, 12]);
        assert_eq!(tile1.row(3), &[13, 14, 15, 16]);
        assert_eq!(
            *tile1.neighbours.borrow(),
            [Some(1), Some(2), Some(3), Some(4)]
        );

        let mut tile2 = tile.clone();
        tile2.fit_border(1, &[16, 12, 8, 4]);

        assert_eq!(tile2.row(0), &[16, 15, 14, 13]);
        assert_eq!(tile2.row(1), &[12, 11, 10, 9]);
        assert_eq!(tile2.row(2), &[8, 7, 6, 5]);
        assert_eq!(tile2.row(3), &[4, 3, 2, 1]);
        assert_eq!(
            *tile2.neighbours.borrow(),
            [Some(3), Some(4), Some(1), Some(2)]
        );

        let mut tile3 = tile.clone();
        tile3.fit_border(1, &[4, 8, 12, 16]);

        assert_eq!(
            *tile3.neighbours.borrow(),
            [Some(1), Some(4), Some(3), Some(2)]
        );
    }
}
