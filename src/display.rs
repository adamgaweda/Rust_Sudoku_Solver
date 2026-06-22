// ANSI terminal display module — no external dependencies, pure Rust
// Uses clear-screen approach for live animation (robust, no line-count issues)

// ── Color codes ───────────────────────────────────────────────────────────────
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

const FG_WHITE: &str = "\x1b[97m";
const FG_CYAN: &str = "\x1b[96m";
const FG_RED: &str = "\x1b[91m";
const FG_YELLOW: &str = "\x1b[93m";
const FG_GREEN: &str = "\x1b[92m";
const FG_BLUE: &str = "\x1b[94m";
const FG_MAGENTA: &str = "\x1b[95m";

const BG_DARK: &str = "\x1b[48;5;235m";
const BG_BOX: &str = "\x1b[48;5;237m";

const CURSOR_HIDE: &str = "\x1b[?25l";
const CURSOR_SHOW: &str = "\x1b[?25h";
/// Clear screen and move cursor to top-left
const HOME: &str = "\x1b[2J\x1b[H";

// ── Helpers ───────────────────────────────────────────────────────────────────

fn has_conflict(sudoku: &[[u8; 9]; 9], row: usize, col: usize) -> bool {
    let val = sudoku[row][col];
    if val == 0 {
        return false;
    }
    for i in 0..9 {
        if i != col && sudoku[row][i] == val {
            return true;
        }
        if i != row && sudoku[i][col] == val {
            return true;
        }
    }
    false
}

fn box_of(row: usize, col: usize) -> usize {
    (row / 3) * 3 + col / 3
}

fn cell_str(sudoku: &[[u8; 9]; 9], map: &[[bool; 9]; 9], row: usize, col: usize) -> String {
    let val = sudoku[row][col];
    if val == 0 {
        format!("{DIM}·{RESET}")
    } else if map[row][col] {
        format!("{BOLD}{FG_WHITE}{val}{RESET}")
    } else if has_conflict(sudoku, row, col) {
        format!("{BOLD}{FG_RED}{val}{RESET}")
    } else {
        format!("{FG_CYAN}{val}{RESET}")
    }
}

// ── Board renderer ────────────────────────────────────────────────────────────

fn render_board(sudoku: &[[u8; 9]; 9], map: &[[bool; 9]; 9]) -> String {
    let box_bg = |row: usize, col: usize| -> &'static str {
        if (box_of(row, col) % 2) == 0 { BG_DARK } else { BG_BOX }
    };

    let h_thick = format!("{FG_YELLOW}╠═══════╬═══════╬═══════╣{RESET}");
    let h_thin  = format!("{FG_YELLOW}╟───────╫───────╫───────╢{RESET}");
    let top     = format!("{FG_YELLOW}╔═══════╦═══════╦═══════╗{RESET}");
    let bot     = format!("{FG_YELLOW}╚═══════╩═══════╩═══════╝{RESET}");

    let mut out = String::new();
    out.push_str(&top);
    out.push('\n');

    for row in 0..9 {
        if row > 0 {
            if row % 3 == 0 {
                out.push_str(&h_thick);
            } else {
                out.push_str(&h_thin);
            }
            out.push('\n');
        }

        out.push_str(&format!("{FG_YELLOW}║{RESET}"));
        for col in 0..9 {
            if col > 0 && col % 3 == 0 {
                out.push_str(&format!("{FG_YELLOW}║{RESET}"));
            } else if col > 0 {
                out.push_str(&format!("{DIM}│{RESET}"));
            }
            let bg = box_bg(row, col);
            let cell = cell_str(sudoku, map, row, col);
            out.push_str(&format!("{bg} {cell} {RESET}"));
        }
        out.push_str(&format!("{FG_YELLOW}║{RESET}"));
        out.push('\n');
    }

    out.push_str(&bot);
    out
}

fn render_stats(iteration: usize, energy: i32, temperature: f64, restart: usize) -> String {
    let e_color = if energy == 0 { FG_GREEN } else { FG_RED };
    format!(
        "  {BOLD}{FG_BLUE}Iter:{RESET} {FG_WHITE}{:>9}{RESET}  \
         {BOLD}{FG_BLUE}Conflicts:{RESET} {e_color}{BOLD}{:>3}{RESET}  \
         {BOLD}{FG_BLUE}Temp:{RESET} {FG_MAGENTA}{:.6}{RESET}  \
         {BOLD}{FG_BLUE}Restart:{RESET} {FG_YELLOW}{:>2}{RESET}",
        iteration, energy, temperature, restart
    )
}

fn render_legend() -> String {
    format!(
        "  {FG_WHITE}{BOLD}Legend:{RESET}  \
         {BOLD}{FG_WHITE}█{RESET} given   \
         {FG_CYAN}█{RESET} placed   \
         {FG_RED}█{RESET} conflict\n"
    )
}

fn render_header() -> String {
    format!(
        "\n  {BOLD}{FG_YELLOW}╔══════════════════════════════════════╗{RESET}\n\
           {BOLD}  ║{FG_YELLOW}   🧩  SUDOKU — Simulated Annealing   {FG_YELLOW}║{RESET}\n\
           {BOLD}{FG_YELLOW}  ╚══════════════════════════════════════╝{RESET}\n"
    )
}

// ── Public API ────────────────────────────────────────────────────────────────

/// First draw — sets up the screen.
pub fn print_sudoku_initial(sudoku: &[[u8; 9]; 9], map: &[[bool; 9]; 9], energy: i32, t_start: f64) {
    print!("{CURSOR_HIDE}{HOME}");
    print!("{}", render_header());
    print!("{}", render_legend());
    println!("{}", render_board(sudoku, map));
    println!("\n{}", render_stats(0, energy, t_start, 0));
    println!("\n  {DIM}Initialising…{RESET}");
}

/// Live update — clears screen and redraws.
pub fn print_sudoku_step(
    sudoku: &[[u8; 9]; 9],
    map: &[[bool; 9]; 9],
    iteration: usize,
    energy: i32,
    temperature: f64,
    restart: usize,
) {
    print!("{HOME}");
    print!("{}", render_header());
    print!("{}", render_legend());
    println!("{}", render_board(sudoku, map));
    println!("\n{}", render_stats(iteration, energy, temperature, restart));
    println!("\n  {DIM}Solving…{RESET}");
}

/// Restart notification — just a re-draw with a different status line.
pub fn print_sudoku_restart(
    sudoku: &[[u8; 9]; 9],
    map: &[[bool; 9]; 9],
    restart_num: usize,
    total_iter: usize,
    energy: i32,
    temperature: f64,
    elapsed: std::time::Duration,
) {
    print!("{HOME}");
    print!("{}", render_header());
    print!("{}", render_legend());
    println!("{}", render_board(sudoku, map));
    println!("\n{}", render_stats(total_iter, energy, temperature, restart_num));
    println!(
        "\n  {FG_YELLOW}↻ Restart #{restart_num}  \
         — stuck at {FG_RED}{energy}{FG_YELLOW} conflicts \
         — elapsed {:.2?}{RESET}",
        elapsed
    );
}

/// Final state — solved or failed.
pub fn print_sudoku_final(
    sudoku: &[[u8; 9]; 9],
    map: &[[bool; 9]; 9],
    total_iter: usize,
    elapsed: std::time::Duration,
) {
    let energy = crate::sudoku_solver::whole_energy(sudoku);
    print!("{HOME}");
    print!("{}", render_header());
    print!("{}", render_legend());
    println!("{}", render_board(sudoku, map));
    println!("\n{}", render_stats(total_iter, energy, 0.0, 0));

    if energy == 0 {
        println!(
            "\n  {BOLD}{FG_GREEN}✓  Solved in {total_iter} iterations  ({:.2?}){RESET}",
            elapsed
        );
    } else {
        println!(
            "\n  {BOLD}{FG_RED}✗  Could not solve — {energy} conflicts remain  ({:.2?}){RESET}",
            elapsed
        );
    }
    print!("{CURSOR_SHOW}");
}
