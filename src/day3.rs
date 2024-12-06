use std::fs::read_to_string;
use regex::Regex;
fn main() {

    let mut memory = read_to_string("data/day3_data.txt").unwrap();

    memory.insert_str(0, "do()");

    memory.insert_str(memory.len()-1, "don't()");

    memory = memory.replace("don't()", "don't()don't()");

    let remover1 = Regex::new(r"(?s)do\(\).*?don't\(\)").unwrap();

    let mut res = 0;

    for group in remover1.find_iter(&memory).map(|m| m.as_str()) {

        let re = Regex::new(r"mul\((?<a>[\d]{1,3}),(?<b>[\d]{1,3})\)").unwrap();

        let a_b: Vec<(i32, i32)> = re.captures_iter(group).map(|caps| {
            let (_, [a, b]) = caps.extract();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        }).collect();


        for (a, b) in a_b {
            res += a*b;
        }
    }

    println!("{res}");
    

}
