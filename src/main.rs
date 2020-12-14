use std::{env, io};

use adventofcode_2020::*;

macro_rules! run_day {
    ($day:path) => {{
        use $day::*;
        println!(
            "{}\n    part1:    {:?}\n    part2:    {:?}",
            stringify!($day),
            _1(),
            _2()
        );
    }};
}

macro_rules! invalid_day {
    ($x:expr) => {{
        println!("Invalid day number: {}", $x);
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();

    match args.get(1) {
        Some(option) => {
            day = option.to_string();
        }
        None => {
            println!("Enter day: ");
            io::stdin()
                .read_line(&mut day)
                .expect("Failed to read line");
        }
    }

    match day.trim().parse::<i32>() {
        Ok(day_num) => match day_num {
            1 => run_day!(day1),
            2 => run_day!(day2),
            3 => run_day!(day3),
            4 => run_day!(day4),
            5 => run_day!(day5),
            6 => run_day!(day6),
            7 => run_day!(day7),
            8 => run_day!(day8),
            9 => run_day!(day9),
            10 => run_day!(day10),
            11 => run_day!(day11),
            12 => run_day!(day12),
            14 => run_day!(day14),
            _ => invalid_day!(day),
        },
        Err(_) => {
            invalid_day!(day);
        }
    };
}
