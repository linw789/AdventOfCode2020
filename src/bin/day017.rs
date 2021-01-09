use std::str::from_utf8;
use std::mem;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Active,
    Inactive,
}

impl fmt::Debug for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Active => { return write!(f, "#"); },
            CellState::Inactive => { return write!(f, "."); },
        }
    }
}

fn clear_grid(grid: &mut Vec<Vec<Vec<CellState>>>) {
    for y_dim in grid {
        for z_dim in y_dim {
            for cell in z_dim {
                *cell = CellState::Inactive;
            }
        }
    }
}

fn count_active_neighbors(grid: &mut Vec<Vec<Vec<CellState>>>, z: usize, y: usize, x: usize) -> i32 {
    let mut count = 0;
    if grid[z][y][x] == CellState::Active {
        count -= 1; // don't count itself.
    }

    for iz in (z - 1)..(z + 2) {
        for iy in (y - 1)..(y + 2) {
            for ix in (x - 1)..(x + 2) {
                if grid[iz][iy][ix] == CellState::Active {
                    count += 1; // account for itself.
                }
            }
        }
    }

    return count;
}

fn count_active(grid: &Vec<Vec<Vec<CellState>>>) -> i32 {
    let mut active_cnt = 0;
    for y_dim in grid {
        for z_dim in y_dim {
            for cell in z_dim {
                if *cell == CellState::Active {
                    active_cnt += 1;
                }
            }
        }
    }
    return active_cnt;
}

fn part_1(grid: &Vec<Vec<Vec<CellState>>>) -> i32 {
    let mut grid0 = grid.clone();
    let mut grid1 = grid.clone();

    let grid_src = &mut grid0;
    let grid_dest = &mut grid1;

    /*
    for y_dim in &grid_src[3] {
        println!("{:?}", y_dim);
    }
    println!("\n");
    */

    let search_index_end = grid_src.len() - 1;

    for _ in 0..6 {
        mem::swap(grid_src, grid_dest); 
        clear_grid(grid_dest);

        for z in 1..search_index_end {
            for y in 1..search_index_end {
                for x in 1..search_index_end {
                    let active_neighbors = count_active_neighbors(grid_src, z, y, x);
                    match grid_src[z][y][x] {
                        CellState::Active => {
                            if active_neighbors == 2 || active_neighbors == 3 {
                                grid_dest[z][y][x] = CellState::Active;
                            }
                        },
                        CellState::Inactive => {
                            if active_neighbors == 3 {
                                grid_dest[z][y][x] = CellState::Active;
                            }
                        },
                    }
                }
            }
        }
    }

    /*
    for z in 2..5 {
        for y_dim in &grid_dest[z] {
            println!("{:?}", y_dim);
        }
        println!("\n");
    }
    */

    return count_active(grid_dest);
}

fn main() {
    let input = include_bytes!("day017.input");
    let lines = from_utf8(input).unwrap().lines();

    // Each cycle grows the bound by 1 on each dimension on both directions. So after 6 cycles, the
    // bound grows 6 * 2. Adding extra one so we don't need to test out-of-bound when checking
    // neighbors. Adding the original bound (8) results (6 + 1) * 2 + 8 = 22.
    const DIM_MAX: usize = 22;

    let mut grid = Vec::new();
    for _z in 0..DIM_MAX {
        let mut y_dim = Vec::new();
        for _y in 0..DIM_MAX {
            let mut x_dim = Vec::new();
            x_dim.resize(DIM_MAX, CellState::Inactive);
            y_dim.push(x_dim);
        }
        grid.push(y_dim);
    }

    let start_pos = DIM_MAX / 2 - lines.clone().count() / 2;
    let mut y = start_pos;
    for line in lines {
        let mut x = start_pos;
        for c in line.chars() {
            match c {
                '.' => grid[DIM_MAX / 2][y][x] = CellState::Inactive,
                '#' => grid[DIM_MAX / 2][y][x] = CellState::Active,
                _ => panic!("Invalid cell state."),
            }
            x += 1;
        }
        y += 1;
    }

    println!("Part 1: {}", part_1(&grid));
}
