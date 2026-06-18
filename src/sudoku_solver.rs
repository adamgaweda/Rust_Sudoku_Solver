
pub struct Point(pub u8,pub u8);
use rand::Rng;

fn energy_count(sudoku: &[[u8;9];9], first: &Point, second: &Point) -> i32{
    let mut energy_new = 0;
    let first_x = first.0 as usize;
    let first_y = first.1 as usize;
    let second_x = first.0 as usize;
    let second_y = first.1 as usize;
    let first_val = sudoku[first_x][first_y];
    let second_val = sudoku[second_x][second_y];
    for i in 0..9 {
        if(sudoku[i][first.1 as usize] == first_val && i!=first_x) {
            energy_new += 1;
        }
        if(sudoku[i][second.1 as usize] == second_val && i!=second_x) {
            energy_new += 1;
        }
        if(sudoku[first.0 as usize][i] == first_val && i!=first_y) {
            energy_new += 1;
        }
        if(sudoku[second.0 as usize][i] == second_val && i!=second_y) {
            energy_new += 1;
        }
    }
    energy_new
}

fn fill_sudoku(sudoku: &mut [[u8;9];9]) {
    let mut mapa: [bool;10];
    let mut number: usize;
    let mut row : usize;
    let mut col :usize;
    for i in 0..3 {
        for j in 0..3 {
            mapa = [false;10];
            for k in 0..3 {
                for l in 0..3 {
                    row = 3*i+k as usize;
                    col = 3*j+l as usize;
                    number = sudoku[row][col] as usize;
                    
                }
            }
        }
    }
}

fn whole_energy(sudoku: &[[u8;9];9]) -> i32 {
    let mut energy = 0;
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j] == sudoku[i][i] && i!=j {
                energy += 1;
            }
        }
    }
    energy
}

fn change_state(sudoku: &mut [[u8;9];9], first: &Point, second: &Point){
    let first_row = first.1 as usize;
    let second_row = second.1 as usize;
    let first_col = first.0 as usize;
    let second_col = second.0 as usize;

    let change = sudoku[first_row][first_col];
    sudoku[first_row][second_col] = sudoku[second_row][second_col];
    sudoku[second_row][second_col] = change;
}

fn solve(sudoku: &mut [[u8;9];9]){
    let mut map = [[true;9];9];
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j]==0{
                map[i][j]=false;
            }
        }
    }
    let mut temperature = 100.0;
    let temp_par = 0.99;
    let mut energy = whole_energy(sudoku);
    let mut energy_diff = 0;
    for i in 0..100000 {
        if energy == 0{
            println!("Sudoku solved!");
            return;
        }


    }
    println!("Sudoku couldn't be solved!");

}
