use std::collections::HashMap;

use crate::*;

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

pub struct Days {
    pub list: HashMap<i32, fn()>,
}

impl Default for Days {
    fn default() -> Self {
        Days::new()
    }
}

impl Days {
    pub fn new() -> Self {
        Self {
            list: DAYS! {
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
            },
        }
    }
}
