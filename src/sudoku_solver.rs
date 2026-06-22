use rand::Rng;
use std::time::Instant;

use crate::display;

pub fn whole_energy(sudoku: &[[u8; 9]; 9]) -> i32 {
    let mut energy = 0i32;
    let mut row = [0i32; 10];
    let mut col = [0i32; 10];
    for i in 0..9 {
        row = [0i32; 10];
        col = [0i32; 10];
        for j in 0..9 {
            row[sudoku[i][j] as usize] += 1;
            col[sudoku[j][i] as usize] += 1;
        }
        for k in 1..10 {
            if row[k] > 1 { energy += row[k] - 1; }
            if col[k] > 1 { energy += col[k] - 1; }
        }
    }
    energy
}


fn energy_count(sudoku: &[[u8; 9]; 9], p1: (usize, usize), p2: (usize, usize)) -> i32 {
    let (first_row, first_column) = p1;
    let (second_row, second_column) = p2;
    let first_val = sudoku[first_row][first_column] as usize;
    let second_val = sudoku[second_row][second_column] as usize;

    let mut delta = 0i32;

    for i in 0..9 {
        if(sudoku[i][first_column] as usize == first_val && i!=first_row) {
            delta += 1;
        }
        if(sudoku[i][second_column] as usize == second_val && i!=second_row) {
            delta += 1;
        }
        if(sudoku[first_row][i] as usize == first_val && i!=first_column) {
            delta += 1;
        }
        if(sudoku[second_row][i] as usize == second_val && i!=second_column) {
            delta += 1;
        }
    }
    delta
}


pub fn fill_squares(sudoku: &mut [[u8; 9]; 9]) {
    let mut present = [false; 10];
    let mut v : usize;
    let mut next: usize;

    for box_idx in 0..9 {
        let box_row = (box_idx / 3) * 3;
        let box_col = (box_idx % 3) * 3;

        present = [false; 10];

        for row in box_row..box_row + 3 {
            for col in box_col..box_col + 3 {
                v = sudoku[row][col] as usize;
                if v != 0 {
                    if present[v] {
                        panic!("Invalid puzzle: digit {v} appears twice in box {box_idx}");
                    }
                    present[v] = true;
                }
            }
        }

        next = 1usize;
        for row in box_row..box_row + 3 {
            for col in box_col..box_col + 3 {
                if sudoku[row][col] == 0 {
                    while present[next] { next += 1; }
                    sudoku[row][col] = next as u8;
                    present[next] = true;
                    next += 1;
                }
            }
        }
    }
}


fn change_state(sudoku: &mut [[u8; 9]; 9], a: (usize, usize), b: (usize, usize)) {
    let tmp = sudoku[a.0][a.1];
    sudoku[a.0][a.1] = sudoku[b.0][b.1];
    sudoku[b.0][b.1] = tmp;
}

fn mutable_cells_in_box(fixed: &[[bool; 9]; 9], box_index: usize) -> Vec<(usize, usize)> {
    let box_row = (box_index / 3) * 3;
    let box_col = (box_index % 3) * 3;
    let mut cells = Vec::new();
    for row in box_row..box_row + 3 {
        for col in box_col..box_col + 3 {
            if !fixed[row][col] { cells.push((row, col)); }
        }
    }
    cells
}

pub fn solve(sudoku: &mut [[u8; 9]; 9], fixed: &[[bool; 9]; 9], show_progress: bool) -> bool {
    let mut rng = rand::thread_rng();
    let start = Instant::now();

    const T_START: f64 = 0.5;
    const COOLING: f64 = 0.999_95;
    const ITER_PER_RUN: usize = 100_000;
    const MAX_RESTARTS: usize = 200;
    const DISPLAY_EVERY: usize = 10_000;
    const RESYNC_EVERY: usize = 50_000;

    let original = *sudoku;
    let mut total_iter: usize = 0;
    let mut temp = T_START;
    let mut energy : i32;
    let mut delta :i32;

    if show_progress {
        fill_squares(sudoku);
        display::print_sudoku_initial(sudoku, fixed, whole_energy(sudoku), T_START);
    }

    for restart in 0..MAX_RESTARTS {
        *sudoku = original;
        fill_squares(sudoku);


        energy = whole_energy(sudoku);
        total_iter= 0;

        for _ in 0..ITER_PER_RUN {
            total_iter += 1;

            if energy == 0 {
                if whole_energy(sudoku) == 0 {
                    if show_progress {
                        display::print_sudoku_final(sudoku, fixed, total_iter, start.elapsed());
                    }
                    return true;
                }

                energy = whole_energy(sudoku);
            }

            let box_idx = rng.r#gen_range(0..9);
            let candidates = mutable_cells_in_box(fixed, box_idx);
            if candidates.len() < 2 { continue; }

            let i1 = rng.r#gen_range(0..candidates.len());
            let mut i2 = rng.r#gen_range(0..candidates.len());
            while i2 == i1 { 
                i2 = rng.r#gen_range(0..candidates.len());
            }

            let p1 = candidates[i1];
            let p2 = candidates[i2];


            delta = -1 * energy_count(sudoku, p1, p2);
            change_state(sudoku, p1, p2);
            delta += energy_count(sudoku, p1, p2);

            if delta <= 0 {
                energy += delta;
            } else {
                let probability = (-delta as f64 / temp).exp();
                if rng.r#gen::<f64>() < probability {
                    energy += delta;
                } else {
                    change_state(sudoku, p1, p2);
                }
            }

            temp *= COOLING;


            if total_iter % RESYNC_EVERY == 0 {
                energy = whole_energy(sudoku);
            }

            if show_progress && total_iter % DISPLAY_EVERY == 0 {
                display::print_sudoku_step(sudoku, fixed, total_iter, energy, temp, restart + 1);
            }
        }

        if show_progress {
            display::print_sudoku_restart(
                sudoku, fixed,
                restart + 1, total_iter,
                whole_energy(sudoku), temp,
                start.elapsed(),
            );
        }
    }

    if show_progress {
        display::print_sudoku_final(sudoku, fixed, total_iter, start.elapsed());
    }
    false
}
