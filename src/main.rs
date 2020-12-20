mod day1;
mod day2;
mod day3;
mod utils;
use day1::main as day1;
use day2::main as day2;
use day3::main as day3;

use regex::Regex;
use std::env;

fn main() {
    let number_extract = Regex::new(r"\d{1,2}").unwrap();
    let day = env::args()
        .skip(1)
        .find_map(|arg| {
            number_extract.captures(&arg).and_then(|captures| {
                captures
                    .get(0)
                    .and_then(|capture| capture.as_str().parse::<i32>().ok())
            })
        })
        .unwrap_or(1);
    match day {
        1 => day1(),
        2 => day2(),
        3 => day3(),
        _ => println!("ERROR: Day {} does not have a solution!", day),
    }
}