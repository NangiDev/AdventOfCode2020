#![feature(array_value_iter)]

use std::{collections::HashMap, env};

use adventofcode_2020::*;

macro_rules! DAYS {
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            let mut m: HashMap<i32, fn()> = ::std::collections::HashMap::new();
            $(
                m.insert($k,$v);
            )*
            m
        }
    };
}

macro_rules! invalid_day {
    ($x:expr) => {{
        println!("Invalid day number: \"{}\"", $x);
    }};
}

fn main() {
    let list: HashMap<i32, fn()> = DAYS! {
        1 => day01::print,
        2 => day02::print,
        3 => day03::print,
        4 => day04::print,
        5 => day05::print,
        6 => day06::print,
        7 => day07::print,
        8 => day08::print,
        9 => day09::print,
        10 => day10::print,
        11 => day11::print,
        12 => day12::print,
        13 => day13::print,
        14 => day14::print,
        15 => day15::print,
        16 => day16::print,
        18 => day18::print,
    };

    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        match day.trim().parse::<i32>() {
            Ok(day) => match list.get(&day) {
                Some(day) => day(),
                None => invalid_day!(day),
            },
            Err(_) => invalid_day!(day),
        };
    } else {
        println!("Missing argument!");
    }
}
