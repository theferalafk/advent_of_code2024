use std::fs::read_to_string;


fn main() {

    let raw_data = read_to_string("data/day6_data.txt").unwrap();

    let tmp = raw_data.split('\n').collect::<Vec<&str>>();

    let mut maze: Vec<Vec<char>> = Vec::new();

    for i in tmp {
        maze.push(i.chars().collect::<Vec<char>>());
    }

    let move_direction: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut position = (0, 0);

    let mut not_visited = 0;

    'pos: for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j].eq(&'^'){
                position = (i as i32, j as i32);
                break 'pos;
            }
        }
    }

    let mut visited = 0;

    let mut part2 = 0;

    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            //println!("{}",maze.len());
            //println!("{}",maze[0].len());
            let mut tmp_maze = maze.clone();
            tmp_maze[i][j]='#';
            let mut tmp_position = position.clone();
            let mut direction = 0; //0 up, 1 right, 2 down, 3 left mod 4

            'walk: loop {

                let next_field = (tmp_position.0+move_direction[direction].0, tmp_position.1+move_direction[direction].1);

                if next_field.0 < 0 || next_field.0 > tmp_maze.len() as i32 -1 || next_field.1 < 0 || next_field.1 > tmp_maze[0].len() as i32 - 1{

                    if tmp_maze[tmp_position.0 as usize][tmp_position.1 as usize].ne(&'X') {
                        tmp_maze[tmp_position.0 as usize][tmp_position.1 as usize] = 'X';
                        visited += 1;
                    }

                    break 'walk;
                }

                let next_object = tmp_maze[next_field.0 as usize][next_field.1 as usize];

                if next_object.eq(&'#') {
                    direction = (direction+1)%4;
                } else {
                    if tmp_maze[tmp_position.0 as usize][tmp_position.1 as usize].ne(&'X') {
                        tmp_maze[tmp_position.0 as usize][tmp_position.1 as usize] = 'X';
                        visited += 1;
                        not_visited = 0;

                    } else {
                        not_visited += 1;
                    }
                    tmp_position = next_field;

                }

                if not_visited > 130*4 {
                    part2 += 1;
                    break 'walk;
                }
            }
        }
    }
    //for i in maze {
    //    println!("{:?}", i);
    //}
    println!("{visited}");
    println!("{part2}");


    
}
