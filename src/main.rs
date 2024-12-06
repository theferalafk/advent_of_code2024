use std::fs::read_to_string;
fn main() {

    let raw_data = read_to_string("data/day5_data.txt").unwrap();

    let tmp = raw_data.split("\n\n").collect::<Vec<&str>>();

    let rules = tmp[0];

    for prints in tmp[1].split("\n") {
        //create opposing rule

        for pages in prints.split(",") {
            
        }

        println!("{:?}", rules);

        println!("{:?}", prints);
        //check for exisiting rules
    }
}
