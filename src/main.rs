use std::fs::read_to_string;
use regex::Regex;
fn main() {

    let mut memory = read_to_string("data/day3_data.txt").unwrap();

    let remover1 = Regex::new(r"don't\(\).+?don't\(\)").unwrap();

    let mut test = memory.clone();

    for _i in 0..3 {
    for res in remover1.find_iter(&test.clone()).map(|m| m.as_str()) {
        if !res.contains("do()"){
            test = test.replace(res, "don't()");
        }
    }

    }

    println!("{} {}",memory.len(),test.len());

    let remover2 = Regex::new(r"don't\(\).*?do\(\)").unwrap();

    memory = remover2.replace_all(&memory, "").to_string();

    test = remover2.replace_all(&test, "").to_string();


    println!("{} {} {} {}",memory.len(),test.len(), memory.contains(  "don't()"), test.contains("don't()"));

    let re = Regex::new(r"mul\((?<a>[\d]{1,3}),(?<b>[\d]{1,3})\)").unwrap();

    let a_b: Vec<(i32, i32)> = re.captures_iter(&memory).map(|caps| {
        let (_, [a, b]) = caps.extract();
        (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
    }).collect();

    //println!("{:?}",a_b);

    let mut res = 0;

    for (a, b) in a_b {
        res += a*b;
    }

    println!("{res}");
    

}
