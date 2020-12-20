use std::collections::HashMap;

use crate::{input, run_day, utils::Input};

pub fn add_next_spoken(
    mut values: Vec<i32>,
    latest: &mut HashMap<i32, usize>,
    previous: &mut HashMap<i32, usize>,
) -> Vec<i32> {
    let latest_val = *values.last().unwrap();
    let last_idx = values.len();
    match latest.get(&latest_val) {
        Some(last_spoken) => match previous.get(&latest_val) {
            Some(prev_spoken) => {
                let new_val = (last_spoken - prev_spoken) as i32;

                if let Some(p) = latest.get(&new_val) {
                    previous.insert(new_val, *p);
                }

                latest.insert(new_val, last_idx);
                values.push(new_val);
            }
            None => {
                let new_val = 0;

                if let Some(p) = latest.get(&new_val) {
                    previous.insert(new_val, *p);
                }

                latest.insert(new_val, last_idx);
                values.push(new_val);
            }
        },
        None => {
            latest.insert(0, last_idx);
            values.push(0);
        }
    }
    values
}

pub fn get_nth_number(n: usize, mut values: Vec<i32>) -> i32 {
    let mut count = values.len();

    let mut latest: HashMap<i32, usize> = HashMap::new();
    for (idx, v) in values.iter().enumerate() {
        latest.insert(*v, idx);
    }
    let mut previous: HashMap<i32, usize> = HashMap::new();

    while count < n {
        values = add_next_spoken(values.clone(), &mut latest, &mut previous);
        count += 1;
    }

    *values.last().unwrap()
}

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day15.txt").as_int();
    get_nth_number(2020, input)
}

pub fn _2() -> i32 {
    let input = input!("./src/input_files/day15.txt").as_int();
    get_nth_number(30000000, input)
}

pub fn print() {
    run_day!(crate::day15);
}
