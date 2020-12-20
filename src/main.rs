use std::env;

use adventofcode_2020::*;

const DAYS: &[fn()] = &[
    day1::print,
    day2::print,
    day3::print,
    day4::print,
    day5::print,
    day6::print,
    day7::print,
    day8::print,
    day9::print,
    day10::print,
    day11::print,
    day12::print,
    day13::print,
    day14::print,
    day15::print,
    day16::print,
    day16::print,
    day18::print,
    day18::print,
    day18::print,
];

macro_rules! invalid_day {
    ($x:expr) => {{
        println!("Invalid day number: \"{}\"", $x);
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        match day.trim().parse::<usize>() {
            Ok(day) => match DAYS.get(day.wrapping_sub(1)) {
                Some(day) => day(),
                None => invalid_day!(day),
            },
            Err(_) => invalid_day!(day),
        };
    } else {
        println!("Missing argument!");
    }
}
