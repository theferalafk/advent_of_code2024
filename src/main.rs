use std::fs::read_to_string;

fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32>  = Vec::new();

    for line in read_to_string("data/day1_data.txt").unwrap().lines() {
        let tmp = line.split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
        list1.push(tmp[0]);
        list2.push(tmp[1]);
    }
    list1.sort();
    list2.sort();
    

    // part 1
    let mut res1 = 0;

    for values in list1.iter().zip(&list2) {
        res1 += (values.0 - values.1).abs();
    }

    println!("{}", res1);

    // part 2
    let mut res2: i32 = 0;

    for number in &list1 {
        res2 += number*list2.iter().filter(|&x| x == number).count() as i32;
    }

    println!("{}", res2);

}
