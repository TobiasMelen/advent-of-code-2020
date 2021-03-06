mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;
use day1::main as day1;
use day10::main as day10;
use day11::main as day11;
use day12::main as day12;
use day2::main as day2;
use day3::main as day3;
use day4::main as day4;
use day5::main as day5;
use day6::main as day6;
use day7::main as day7;
use day8::main as day8;
use day9::main as day9;

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
        4 => day4(),
        5 => day5(),
        6 => day6(),
        7 => day7(),
        8 => day8(),
        9 => day9(),
        10 => day10(),
        11 => day11(),
        12 => day12(),
        _ => println!("ERROR: Day {} does not have a solution!", day),
    }
}
