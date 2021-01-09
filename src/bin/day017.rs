use std::fmt;
use std::mem;
use std::str::from_utf8;

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Active,
    Inactive,
}

impl fmt::Debug for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Active => {
                return write!(f, "#");
            }
            CellState::Inactive => {
                return write!(f, ".");
            }
        }
    }
}

fn clear_grid(grid: &mut Vec<Vec<Vec<Vec<CellState>>>>) {
    for w_dim in grid {
        for y_dim in w_dim {
            for z_dim in y_dim {
                for cell in z_dim {
                    *cell = CellState::Inactive;
                }
            }
        }
    }
}

fn count_active_neighbors(
    grid: &Vec<Vec<Vec<Vec<CellState>>>>,
    w: usize,
    z: usize,
    y: usize,
    x: usize,
) -> i32 {
    let mut count = 0;
    if grid[w][z][y][x] == CellState::Active {
        count -= 1; // don't count itself.
    }

    for iw in (w - 1)..(w + 2) {
        for iz in (z - 1)..(z + 2) {
            for iy in (y - 1)..(y + 2) {
                for ix in (x - 1)..(x + 2) {
                    if grid[iw][iz][iy][ix] == CellState::Active {
                        count += 1; // account for itself.
                    }
                }
            }
        }
    }

    return count;
}

fn count_active(grid: &Vec<Vec<Vec<Vec<CellState>>>>) -> i32 {
    let mut active_cnt = 0;
    for w_dim in grid {
        for y_dim in w_dim {
            for z_dim in y_dim {
                for cell in z_dim {
                    if *cell == CellState::Active {
                        active_cnt += 1;
                    }
                }
            }
        }
    }
    return active_cnt;
}

fn part_1(grid: &Vec<Vec<Vec<Vec<CellState>>>>) -> i32 {
    let mut grid0 = grid.clone();
    let mut grid1 = grid.clone();

    let grid_src = &mut grid0;
    let grid_dest = &mut grid1;

    let w = grid.len() / 2;

    for _ in 0..6 {
        mem::swap(grid_src, grid_dest);
        clear_grid(grid_dest);

        for z in 1..(grid_src[w].len() - 1) {
            for y in 1..(grid_src[w][z].len() - 1) {
                for x in 1..(grid_src[w][z][y].len() - 1) {
                    let active_neighbors = count_active_neighbors(grid_src, w, z, y, x);
                    match grid_src[w][z][y][x] {
                        CellState::Active => {
                            if active_neighbors == 2 || active_neighbors == 3 {
                                grid_dest[w][z][y][x] = CellState::Active;
                            }
                        }
                        CellState::Inactive => {
                            if active_neighbors == 3 {
                                grid_dest[w][z][y][x] = CellState::Active;
                            }
                        }
                    }
                }
            }
        }
    }

    return count_active(grid_dest);
}

fn part_2(grid: &Vec<Vec<Vec<Vec<CellState>>>>) -> i32 {
    let mut grid0 = grid.clone();
    let mut grid1 = grid.clone();

    let grid_src = &mut grid0;
    let grid_dest = &mut grid1;

    for _ in 0..6 {
        mem::swap(grid_src, grid_dest);
        clear_grid(grid_dest);

        for w in 1..(grid.len() - 1) {
            for z in 1..(grid_src[w].len() - 1) {
                for y in 1..(grid_src[w][z].len() - 1) {
                    for x in 1..(grid_src[w][z][y].len() - 1) {
                        let active_neighbors = count_active_neighbors(grid_src, w, z, y, x);
                        match grid_src[w][z][y][x] {
                            CellState::Active => {
                                if active_neighbors == 2 || active_neighbors == 3 {
                                    grid_dest[w][z][y][x] = CellState::Active;
                                }
                            }
                            CellState::Inactive => {
                                if active_neighbors == 3 {
                                    grid_dest[w][z][y][x] = CellState::Active;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return count_active(grid_dest);
}

fn main() {
    let input = include_bytes!("day017.input");
    let lines = from_utf8(input).unwrap().lines();

    // Each cycle grows the bound by 1 on each dimension on both directions. So after 6 cycles, the
    // bound grows 6 * 2 on each dimension. Adding extra one so we don't need to test out-of-bound
    // when checking neighbors. Adding the original bound (8) results (6 + 1) * 2 + 8 = 22.
    const CYCLES: usize = 6;
    const W_SIZE: usize = (CYCLES + 1) * 2 + 1;
    const Z_SIZE: usize = (CYCLES + 1) * 2 + 1;
    const Y_SIZE: usize = (CYCLES + 1) * 2 + 8;
    const X_SIZE: usize = (CYCLES + 1) * 2 + 8;

    let mut grid = Vec::new();
    for _w in 0..W_SIZE {
        let mut z_dim = Vec::new();
        for _z in 0..Z_SIZE {
            let mut y_dim = Vec::new();
            for _y in 0..Y_SIZE {
                let mut x_dim = Vec::new();
                x_dim.resize(X_SIZE, CellState::Inactive);
                y_dim.push(x_dim);
            }
            z_dim.push(y_dim);
        }
        grid.push(z_dim);
    }

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    grid[W_SIZE / 2][Z_SIZE / 2][y + (CYCLES + 1)][x + (CYCLES + 1)] =
                        CellState::Inactive
                }
                '#' => {
                    grid[W_SIZE / 2][Z_SIZE / 2][y + (CYCLES + 1)][x + (CYCLES + 1)] =
                        CellState::Active
                }
                _ => panic!("Invalid cell state."),
            }
        }
    }

    println!("Part 1: {}", part_1(&grid));
    println!("Part 1: {}", part_2(&grid));
}
