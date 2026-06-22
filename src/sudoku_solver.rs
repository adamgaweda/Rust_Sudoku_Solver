

use rand::Rng;


fn energy_count(sudoku: &[[u8;9];9], first: &(usize,usize), second: &(usize,usize)) -> i32{
    let mut energy_new = 0;
    let first_x = first.0 as usize;
    let first_y = first.1 as usize;
    let second_x = second.0 as usize;
    let second_y = second.1 as usize;
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

fn whole_energy(sudoku: &[[u8;9];9]) -> i32 {
    let mut energy = 0;
    let mut row_counts = [0; 10];
    let mut col_counts = [0; 10];
    let mut row_val : usize;
    let mut col_val : usize;
    for i in 0..9 {
        row_counts = [0; 10];
        col_counts = [0; 10];
        for j in 0..9 {

            row_val = sudoku[i][j] as usize;
            col_val = sudoku[j][i] as usize;

            row_counts[row_val] += 1;
            col_counts[col_val] += 1;

            for k in 1..10 {
                if row_counts[k] > 1 {
                    energy += row_counts[k] - 1;
                }
                if col_counts[k] > 1 {
                    energy += col_counts[k] - 1;
                }
            }
        }

    }
    energy
}

fn fill_squares(sudoku: &mut [[u8;9];9]){
    let mut numbers = [false;10];
    let mut number : usize;
    let mut incr : usize = 1;
    for i in 0..9 {
        for j in i%3 *3..i%3 * 3+3 {
            for k in i/3 * 3..i/3 * 3+3 {
                number = sudoku[j][k] as usize;
                if numbers[number] && number != 0 {
                    panic!()
                }
                numbers[number] = true;
            }
        }
        for j in i/3*3..i/3*3+3 {
            for k in i%3*3..i%3*3+3 {
                while (numbers[incr]) {
                    incr+=1
                }
                if(sudoku[j][k]==0){
                    sudoku[j][k] = incr as u8;
                    numbers[incr] = true;
                }
            }
        }
        incr =0;
        numbers = [false;10];
    }
}

fn change_state(sudoku: &mut [[u8;9];9], first: &(usize,usize), second: &(usize,usize)){
    let first_row = first.1 as usize;
    let second_row = second.1 as usize;
    let first_col = first.0 as usize;
    let second_col = second.0 as usize;

    let change = sudoku[first_row][first_col];
    sudoku[first_row][first_col] = sudoku[second_row][second_col];
    sudoku[second_row][second_col] = change;
}

fn grid_indexes(mapa: &[[bool;9];9],box_index: usize) -> Vec<(usize, usize)> {
    let mut cells = Vec::new();
    let row = box_index/3*3 as usize;
    let col = box_index % 3*3 as usize;
    for i in row..row+3 {
        for j in col..col+3 {
            if(!mapa[i][j]){
                cells.push((i,j));
            }
        }
    }
    cells
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
    let mut rng = rand::thread_rng();
    let mut temperature = 100.0;
    let temp_par = 0.99;
    let mut energy = whole_energy(sudoku);
    let mut energy_diff = 0;
    let mut to_change: Vec<(usize, usize)>;
    for i in 0..100000 {
        if energy == 0{
            println!("Sudoku solved!");
            return;
        }
        to_change = grid_indexes(&map, rng.gen_range(0..9));
        if to_change.len() <2 {
            continue;
        }

        let mut idx1 = rng.gen_range(0..to_change.len());
        let mut idx2 = rng.gen_range(0..to_change.len());
        while idx1 == idx2 {
            idx2 = rng.gen_range(0..to_change.len());
        }
        let p1 = to_change[idx1];
        let p2 = to_change[idx2];

        energy_diff = energy_count(&sudoku,&p1,&p2);
        change_state(sudoku,&p1,&p2);
        energy_diff -= energy_count(&sudoku,&p1,&p2);
        if energy_diff <= 0 || rng.gen::<f64>() < (-(energy_diff as f64) / temperature).exp()  {
            energy += energy_diff;
        } else{
            change_state(sudoku,&p1,&p2);
        }
        temperature = temperature*temp_par;

    }
    println!("Sudoku couldn't be solved!");
    panic!();
}
