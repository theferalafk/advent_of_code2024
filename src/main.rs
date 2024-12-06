use std::fs::read_to_string;


fn main() {

    let raw_data = read_to_string("data/day6_data.txt").unwrap();

    let mut direction = 0; //0 up, 1 right, 2 down, 3 left mod 4

    let move_direction = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut position = (1, 4);

    let mut visited = 0;

    

    //was ist mein workflow: -> finde den guard, starting direction ist up -> setze x auf aktueller position visited +1 (falls x schon gesetzt pass)
    // -> addiere entsprechend der richtung

    
}
