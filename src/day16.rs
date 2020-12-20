use std::collections::HashMap;

use crate::{input, run_day, utils::Input};

pub fn get_error_rate(ticket: String, map: HashMap<String, Vec<i32>>) -> i32 {
    for t in ticket
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    {
        let mut valid = false;
        for n in map.values() {
            if t >= n[0] && t <= n[1] || t >= n[2] && t <= n[3] {
                valid = true;
                break;
            }
        }
        if !valid {
            return t;
        }
    }
    0
}

pub fn to_map(input: Vec<String>) -> HashMap<String, Vec<i32>> {
    let mut ticket: HashMap<String, Vec<i32>> = HashMap::new();

    for t in input[0]
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    {
        let t = t.split(':').map(|s| s.to_string()).collect::<Vec<String>>();
        let key = t[0].clone();
        let value = t[1].replace(" or ", "-");
        let value = value
            .split('-')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        ticket.insert(key, value);
    }

    ticket
}

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day16.txt").as_group();
    let ticket = to_map(input.clone());
    let mut nb_t: Vec<String> = input[2]
        .split('\n')
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    nb_t.remove(0);

    let mut error_rate = 0;
    for nearby in nb_t {
        error_rate += get_error_rate(nearby, ticket.clone());
    }
    error_rate
}

pub fn _2() -> String {
    let _input = input!("./src/input_files/day16.txt").as_group();
    "Not implemented".to_string()
}

pub fn print() {
    run_day!(crate::day16);
}
