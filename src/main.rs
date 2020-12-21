use std::env;

use adventofcode_2020::*;

macro_rules! invalid_day {
    ($x:expr) => {{
        println!("Invalid day number: \"{}\"", $x);
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let days = days::Days::new();

    if let Some(day) = args.get(1) {
        match day.trim().parse::<i32>() {
            Ok(day) => match days.list.get(&day) {
                Some(day) => day(),
                None => invalid_day!(day),
            },
            Err(_) => invalid_day!(day),
        };
    } else {
        println!("Missing argument!");
    }
}
