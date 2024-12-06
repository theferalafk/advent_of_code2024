use std::fs::read_to_string;

fn 
is_hit(x: &[char]) -> bool{
    let value1 = ['X','M','A','S'];
    let value2 = ['S','A','M','X'];
    return x.eq(&value1) || x.eq(&value2);
}

fn get_column_n(x: &Vec<Vec<char>>, n: i32, i: usize, j: usize) -> Vec<char> {
    let i_int = i as i32;
    let mut direction = 1;
    if n < 0 {
        direction = -1;
    }
    let index = n.abs();
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for k in 0..index {
        indices.push(((i_int+(direction*k)).try_into().unwrap(), j));
    }
    return indices.iter().map(|&(row, col)| x[row][col]).collect();
}

fn get_diag_n_right(x: &Vec<Vec<char>>, n: i32, i: usize, j: usize) -> Vec<char> {
    let i_int = i as i32;
    let j_int = j as i32;
    let mut direction = 1;
    if n < 0 {
        direction = -1;
    }
    let index = n.abs();
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for k in 0..index {
        indices.push(((i_int+(direction*k)).try_into().unwrap(), (j_int+(1*k)).try_into().unwrap()));
    }
    return indices.iter().map(|&(row, col)| x[row][col]).collect();
}

fn _get_diag_n_left(x: &Vec<Vec<char>>, n: i32, i: usize, j: usize) -> Vec<char> {
    let i_int = i as i32;
    let j_int = j as i32;
    let mut direction = 1;
    if n < 0 {
        direction = -1;
    }
    let index = n.abs();
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for k in 0..index {
        indices.push(((i_int+(direction*k)).try_into().unwrap(), (j_int+(direction*k)).try_into().unwrap()));
    }
    return indices.iter().map(|&(row, col)| x[row][col]).collect();
}

fn main() {

    let riddle = read_to_string("data/day4_data.txt").unwrap();

    /*
    Build Matrix -> 
        [ X M A S ]
        [ M M M M ]
        [ A A A A ]
        [ S S S S ]
    Build String Vertical, Horizontally, Diagonal -> look of xmas or samx
    */
    let mut riddle_matrix: Vec<Vec<char>> = Vec::new();

    for _ in 0..3 {
        riddle_matrix.push(vec!['z'; 141]);
    }

    for line in riddle.split('\n') {
        
        riddle_matrix.push(format!("zzz{}zzz",line).chars().collect());
    }

    for _ in 0..3 {
        riddle_matrix.push(vec!['z'; 141]);
    }

    let mut count = 0;

    for line in &riddle_matrix{
    println!("{:?}",line);
    }
    for i in 3..riddle_matrix.len()-3 {
        for j in 3..riddle_matrix[0].len()-3 {
            /*
            if 2 < i {
                //has three left
            }
            if i < (riddle_matrix.len()-4) {
                //has three right
            }
            if 2 < j {
                //has three up
            }
            if j < (riddle_matrix[0].len()-4) {
                //has three down
            }
            if 2 < i && 2 < j {
                //three diagonal up
            }
            if j < (riddle_matrix.len()-4) && j < (riddle_matrix.len()-4){
                //has diagonal down
            }
            */
            if is_hit(&riddle_matrix[i][j-3..j+1]){
                //left
                //println!("{:?} {} {} 1", &riddle_matrix[i][j-3..j+1], i-3, j-3);
                count += 0;
            }
            if is_hit(&riddle_matrix[i][j..j+4]){
                //right
                println!("{:?} {} {} 2", &riddle_matrix[i][j..j+4], i-3, j-3);
                count += 1;
            }
            if is_hit(&get_column_n(&riddle_matrix, 4, i, j)) {
                println!("{:?} {} {} 3", get_column_n(&riddle_matrix, 4, i, j), i-3, j-3);
                count += 1;
            }
            
            if is_hit(&get_column_n(&riddle_matrix, -4, i, j)) {
                //println!("{:?} {} {} 4", get_column_n(&riddle_matrix, -4, i, j), i-3, j-3);
                count += 0;
            }
            
            if is_hit(&get_diag_n_right(&riddle_matrix, 4, i, j)) {
                println!("{:?} {} {} 5", get_diag_n_right(&riddle_matrix, 4, i, j), i-3, j-3);
                count += 1;
            }
            
            if is_hit(&get_diag_n_right(&riddle_matrix, -4, i, j)) {
                println!("{:?} {} {} 6", get_diag_n_right(&riddle_matrix, -4, i, j), i-3, j-3);
                count += 1;
            }
            /*
            if is_hit(&get_diag_n_left(&riddle_matrix, 4, i, j)) {
                println!("{:?} {} {} 7", get_diag_n_left(&riddle_matrix, 4, i, j), i-3, j-3);
                count += 0;
            }
            
            if is_hit(&get_diag_n_left(&riddle_matrix, -4, i, j)) {
                println!("{:?} {} {} 8", get_diag_n_left(&riddle_matrix, -4, i, j), i-3, j-3);
                count += 0;
            }
            */


        }
    }
    println!("{}", count);
}
