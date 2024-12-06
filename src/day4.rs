use std::fs::read_to_string;

fn is_hit(x: &[char]) -> bool{
    let value1 = ['X','M','A','S'];
    let value2 = ['S','A','M','X'];
    return x.eq(&value1) || x.eq(&value2);
}

fn is_hit_2(x: &[char]) -> bool {
    let value1 = ['M','A','S'];
    let value2 = ['S','A','M'];
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
        indices.push(((i_int+k).try_into().unwrap(), (j_int+(direction*k)).try_into().unwrap()));
    }
    return indices.iter().map(|&(row, col)| x[row][col]).collect();
}

fn get_cross(x: &Vec<Vec<char>>, i: usize, j: usize) -> (Vec<char>, Vec<char>) {
    let i_int = i as i32;
    let j_int = j as i32;

    //first diag
    let mut indices_1: Vec<(usize, usize)> = Vec::new();
    for k in -1..2 {
        indices_1.push(((i_int+k).try_into().unwrap(), (j_int+k).try_into().unwrap()));
    }

    //second diag
    let mut indices_2: Vec<(usize, usize)> = Vec::new();
    for k in -1..2 {
        indices_2.push(((i_int-k).try_into().unwrap(), (j_int+k).try_into().unwrap()));
    }

    return (indices_1.iter().map(|&(row, col)| x[row][col]).collect(), indices_2.iter().map(|&(row, col)| x[row][col]).collect())
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
        riddle_matrix.push(vec!['z'; 146]);
    }
    for line in riddle.split('\n') {
        riddle_matrix.push(format!("zzz{}zzz",line).chars().collect());
    }

    for _ in 0..3 {
        riddle_matrix.push(vec!['z'; 146]);
    }


    let mut count = 0;


    //part 1
    for i in 3..riddle_matrix.len()-3 {
        for j in 3..riddle_matrix[0].len()-3 {
            if is_hit(&riddle_matrix[i][j..j+4]){
                count += 1;
            }
            if is_hit(&get_column_n(&riddle_matrix, 4, i, j)) {
                count += 1;
            }
            
            if is_hit(&get_diag_n_right(&riddle_matrix, 4, i, j)) {
                count += 1;
            }
            
            if is_hit(&get_diag_n_right(&riddle_matrix, -4, i, j)) {
                count += 1;
            }
        }
    }
    println!("{}", count);


    //part 2
    count = 0;

    for i in 3..riddle_matrix.len()-3 {
        for j in 3..riddle_matrix[0].len()-3 { 
            let diags = get_cross(&riddle_matrix, i, j);
            if is_hit_2(&diags.0) && is_hit_2(&diags.1){
                count += 1;
            }
        }
    }
    println!("{}", count);
}
