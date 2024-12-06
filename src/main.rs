use std::fs::read_to_string;
fn main() {

    let raw_data = read_to_string("data/day5_data.txt").unwrap();

    let tmp = raw_data.split("\n\n").collect::<Vec<&str>>();

    let rules = tmp[0].split("\n").collect::<Vec<&str>>();

    let mut res = 0;

    for prints in tmp[1].split("\n") {
        //create opposing rule

        let mut numbers = prints.split(",").collect::<Vec<&str>>();

        let mut valid = true;

        let mut i = 1;
        let mut j = 0;

        while i < numbers.len() {
            'inner: while j < i {
                if rules.contains(&format!("{}|{}", numbers[i], numbers[j]).as_str()) {
                    valid = false;
                    // element i muss dann vor j gepackt werden
                    let tmp = numbers.remove(i);
                    numbers.insert(j, tmp);
                    i = 0;
                    j = 0;
                    break 'inner
                }
                j += 1;
            }
            i += 1;
        }
        if !valid {
            res += numbers[numbers.len()/2].parse::<i32>().unwrap();
        }
        /*
        println!("opposing rules {:?}", opposing_rules);

        println!("rules {:?}", rules);

        println!(" thing tried {:?}", prints);
        */
        //check for exisiting rules
    }

    println!("{res}");
}
