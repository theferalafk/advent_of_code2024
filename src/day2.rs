use std::fs::read_to_string;
use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut report_list: Vec<Vec<i32>> = Vec::new();

    for line in read_to_string("data/day2_data.txt").unwrap().lines() {
        let tmp = line.split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
        report_list.push(tmp);
    }

    let mut safe = 0;

    for _report in report_list {
        '_outer: for j in 0.._report.len() {
            let mut report = _report.clone();
            report.remove(j);
            let mut biggest_diff = 0;
            let mut last_element = report[0];
            let mut safe_up = true;
            let mut safe_down = true;
            for i in 1..report.len() {
                if last_element > report[i] {
                    safe_up = false;
                } else if last_element < report[i] {
                    safe_down = false;
                }
                if last_element == report[i] {
                    safe_down = false;
                    safe_up = false;
                }
                if biggest_diff < (last_element - report[i]).abs() {
                    biggest_diff = (last_element - report[i]).abs();
                }
                last_element = report[i];
            }
            if (safe_down | safe_up ) & (biggest_diff < 4) {
                safe += 1;
                break '_outer;
            }
        }
    }

    println!("{}", safe);

}
