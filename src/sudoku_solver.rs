use rand::Rng;
use std::time::Instant;

use crate::display;

// Convention: tuple (row, col) — .0 = row, .1 = col

// ── Energy ────────────────────────────────────────────────────────────────────

/// Total number of row/column conflicts on the board.
/// For each row and column: if digit k appears n times, contributes n-1 to energy.
pub fn whole_energy(sudoku: &[[u8; 9]; 9]) -> i32 {
    let mut energy = 0i32;
    for i in 0..9 {
        let mut row = [0i32; 10];
        let mut col = [0i32; 10];
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

/// Computes the EXACT change in whole_energy if we swap the two cells.
/// Positive delta = more conflicts (worse), negative delta = fewer conflicts (better).
///
/// The key insight: when digit d appears n times in a row, energy contribution = n-1.
/// Removing one copy: change = -1 if another copy exists (n >= 2), else 0.
/// Adding one copy:   change = +1 if another copy already exists (n >= 1), else 0.
fn energy_delta(sudoku: &[[u8; 9]; 9], p1: (usize, usize), p2: (usize, usize)) -> i32 {
    let (r1, c1) = p1;
    let (r2, c2) = p2;
    let fv = sudoku[r1][c1] as usize;
    let sv = sudoku[r2][c2] as usize;

    if fv == sv { return 0; }

    // Count occurrences of value v in row r excluding column exc
    let row_count = |r: usize, exc: usize, v: usize| -> i32 {
        (0..9).filter(|&c| c != exc && sudoku[r][c] as usize == v).count() as i32
    };
    // Count occurrences of value v in column c excluding row exc
    let col_count = |c: usize, exc: usize, v: usize| -> i32 {
        (0..9).filter(|&r| r != exc && sudoku[r][c] as usize == v).count() as i32
    };

    let mut delta = 0i32;

    // ── Row changes ───────────────────────────────────────────────────────────
    if r1 != r2 {
        // Row r1: fv is removed, sv is added
        let other_fv_r1 = row_count(r1, c1, fv);
        let other_sv_r1 = row_count(r1, c1, sv);
        if other_fv_r1 >= 1 { delta -= 1; } // resolve a conflict
        if other_sv_r1 >= 1 { delta += 1; } // create a conflict

        // Row r2: sv is removed, fv is added
        let other_sv_r2 = row_count(r2, c2, sv);
        let other_fv_r2 = row_count(r2, c2, fv);
        if other_sv_r2 >= 1 { delta -= 1; }
        if other_fv_r2 >= 1 { delta += 1; }
    }

    // ── Column changes ────────────────────────────────────────────────────────
    if c1 != c2 {
        // Col c1: fv is removed, sv is added
        let other_fv_c1 = col_count(c1, r1, fv);
        let other_sv_c1 = col_count(c1, r1, sv);
        if other_fv_c1 >= 1 { delta -= 1; }
        if other_sv_c1 >= 1 { delta += 1; }

        // Col c2: sv is removed, fv is added
        let other_sv_c2 = col_count(c2, r2, sv);
        let other_fv_c2 = col_count(c2, r2, fv);
        if other_sv_c2 >= 1 { delta -= 1; }
        if other_fv_c2 >= 1 { delta += 1; }
    }

    delta
}

// ── Board initialisation ──────────────────────────────────────────────────────

/// Fills each 3×3 box with the missing digits so every box has 1-9 exactly once.
pub fn fill_squares(sudoku: &mut [[u8; 9]; 9]) {
    for box_idx in 0..9 {
        let br = (box_idx / 3) * 3;
        let bc = (box_idx % 3) * 3;

        let mut present = [false; 10];

        for r in br..br + 3 {
            for c in bc..bc + 3 {
                let v = sudoku[r][c] as usize;
                if v != 0 {
                    if present[v] {
                        panic!("Invalid puzzle: digit {v} appears twice in box {box_idx}");
                    }
                    present[v] = true;
                }
            }
        }

        let mut next = 1usize;
        for r in br..br + 3 {
            for c in bc..bc + 3 {
                if sudoku[r][c] == 0 {
                    while present[next] { next += 1; }
                    sudoku[r][c] = next as u8;
                    present[next] = true;
                    next += 1;
                }
            }
        }
    }
}

// ── State transitions ─────────────────────────────────────────────────────────

fn swap_cells(sudoku: &mut [[u8; 9]; 9], a: (usize, usize), b: (usize, usize)) {
    let tmp = sudoku[a.0][a.1];
    sudoku[a.0][a.1] = sudoku[b.0][b.1];
    sudoku[b.0][b.1] = tmp;
}

fn mutable_cells_in_box(fixed: &[[bool; 9]; 9], box_index: usize) -> Vec<(usize, usize)> {
    let br = (box_index / 3) * 3;
    let bc = (box_index % 3) * 3;
    let mut cells = Vec::new();
    for r in br..br + 3 {
        for c in bc..bc + 3 {
            if !fixed[r][c] { cells.push((r, c)); }
        }
    }
    cells
}

// ── Solver ────────────────────────────────────────────────────────────────────

pub fn solve(sudoku: &mut [[u8; 9]; 9], fixed: &[[bool; 9]; 9], show_progress: bool) -> bool {
    let mut rng = rand::thread_rng();
    let start = Instant::now();

    const T_START: f64 = 0.5;
    const COOLING: f64 = 0.999_95;
    const ITER_PER_RUN: usize = 100_000;
    const MAX_RESTARTS: usize = 200;
    const DISPLAY_EVERY: usize = 10_000;
    const RESYNC_EVERY: usize = 50_000; // periodically re-sync energy with ground truth

    let original = *sudoku;
    let mut total_iter: usize = 0;

    if show_progress {
        fill_squares(sudoku);
        display::print_sudoku_initial(sudoku, fixed, whole_energy(sudoku), T_START);
    }

    for restart in 0..MAX_RESTARTS {
        *sudoku = original;
        fill_squares(sudoku);

        let mut temp = T_START;
        let mut energy = whole_energy(sudoku);

        for _ in 0..ITER_PER_RUN {
            total_iter += 1;

            if energy == 0 {
                // Double-check with ground truth before declaring victory
                if whole_energy(sudoku) == 0 {
                    if show_progress {
                        display::print_sudoku_final(sudoku, fixed, total_iter, start.elapsed());
                    }
                    return true;
                }
                // If mismatch (shouldn't happen, but safety net), resync
                energy = whole_energy(sudoku);
            }

            let box_idx = rng.r#gen_range(0..9);
            let candidates = mutable_cells_in_box(fixed, box_idx);
            if candidates.len() < 2 { continue; }

            let i1 = rng.r#gen_range(0..candidates.len());
            let mut i2 = rng.r#gen_range(0..candidates.len());
            while i2 == i1 { i2 = rng.r#gen_range(0..candidates.len()); }

            let p1 = candidates[i1];
            let p2 = candidates[i2];

            // Compute delta BEFORE the swap (positive = worsening, negative = improvement)
            let delta = energy_delta(sudoku, p1, p2);
            swap_cells(sudoku, p1, p2);

            if delta <= 0 {
                energy += delta; // improvement or neutral: always accept
            } else {
                let prob = (-delta as f64 / temp).exp();
                if rng.r#gen::<f64>() < prob {
                    energy += delta; // accept worsening move
                } else {
                    swap_cells(sudoku, p1, p2); // revert
                }
            }

            temp *= COOLING;

            // Periodic energy resync (safety net for any residual drift)
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
