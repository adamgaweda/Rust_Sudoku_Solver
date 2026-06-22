mod sudoku_solver;
mod display;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let show_progress = !args.contains(&"--no-display".to_string());

    // Arto Inkala's "World's Hardest Sudoku" (2010) — 0 = empty cell
    let mut sudoku: [[u8; 9]; 9] = [
        [8, 0, 0,   0, 0, 0,   0, 0, 0],
        [0, 0, 3,   6, 0, 0,   0, 0, 0],
        [0, 7, 0,   0, 9, 0,   2, 0, 0],

        [0, 5, 0,   0, 0, 7,   0, 0, 0],
        [0, 0, 0,   0, 4, 5,   7, 0, 0],
        [0, 0, 0,   1, 0, 0,   0, 3, 0],

        [0, 0, 1,   0, 0, 0,   0, 6, 8],
        [0, 0, 8,   5, 0, 0,   0, 1, 0],
        [0, 9, 0,   0, 0, 0,   4, 0, 0],
    ];

    // Build the fixed-cell mask
    let mut fixed = [[false; 9]; 9];
    for r in 0..9 {
        for c in 0..9 {
            if sudoku[r][c] != 0 {
                fixed[r][c] = true;
            }
        }
    }

    if !show_progress {
        // Headless mode: print plain text progress to stdout
        println!("Running solver in headless mode…");
        let start = std::time::Instant::now();

        // Clone before solve (fill_squares modifies the board internally)
        let solved = sudoku_solver::solve(&mut sudoku, &fixed, false);

        let elapsed = start.elapsed();
        let energy = sudoku_solver::whole_energy(&sudoku);
        if solved {
            println!("✓ Solved!  Energy={energy}  Time={elapsed:.2?}");
            print_plain(&sudoku);
        } else {
            println!("✗ Not solved.  Remaining conflicts={energy}  Time={elapsed:.2?}");
            print_plain(&sudoku);
        }
    } else {
        // Animated terminal display
        let solved = sudoku_solver::solve(&mut sudoku, &fixed, true);
        if !solved {
            std::process::exit(1);
        }
    }
}

fn print_plain(sudoku: &[[u8; 9]; 9]) {
    for (r, row) in sudoku.iter().enumerate() {
        if r > 0 && r % 3 == 0 { println!("------+-------+------"); }
        for (c, &v) in row.iter().enumerate() {
            if c > 0 && c % 3 == 0 { print!(" | "); }
            else if c > 0 { print!(" "); }
            print!("{v}");
        }
        println!();
    }
}
