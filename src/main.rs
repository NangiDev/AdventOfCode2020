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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }

    day = day.trim().to_string();
    let day_num: u8 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };

    match day_num {
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
        _ => println!("Invalid day number: {}", day_num),
    }
}
