use std::env;

use adventofcode_2020::*;

const DAYS: &[fn()] = &[
    day01::print,
    day02::print,
    day03::print,
    day04::print,
    day05::print,
    day06::print,
    day07::print,
    day08::print,
    day09::print,
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
