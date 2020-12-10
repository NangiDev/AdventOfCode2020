use std::{collections::HashMap, fs, vec};

pub fn get_built_in_joltage(joltages: Vec<i64>) -> i64 {
    joltages.into_iter().fold(0, i64::max) + 3
}

pub fn get_multiplied_adapter_diffs(mut joltages: Vec<i64>) -> i64 {
    let mut threes = 0;
    let mut oncies = 0;
    joltages.push(get_built_in_joltage(joltages.clone()));
    joltages.sort();
    joltages.into_iter().fold(0, |i, n| {
        if n - i >= 3 {
            threes += 1;
        } else {
            oncies += 1;
        }
        n
    });
    oncies * threes
}

pub fn get_total_of_combinations(mut joltages: Vec<i64>) -> i64 {
    joltages.push(0);
    joltages.push(get_built_in_joltage(joltages.clone()));
    joltages.sort();
    run_depth(0, &joltages, &mut HashMap::new())
}

fn run_depth(index: usize, joltages: &Vec<i64>, memo: &mut HashMap<usize, i64>) -> i64 {
    if memo.contains_key(&index) {
        return *memo.get(&index).unwrap();
    }

    if index >= joltages.len() - 1 {
        memo.insert(index, 1);
        return 1;
    }

    let mut stack: Vec<usize> = vec![];
    for i in (index + 1)..joltages.len() {
        match joltages[i] - joltages[index] {
            1 | 2 | 3 => stack.push(i),
            _ => {}
        }
    }

    let mut count: i64 = 0;
    while !stack.is_empty() {
        let next_index = stack.remove(stack.len() - 1);
        count += run_depth(next_index, joltages, memo);

        memo.insert(index, count);
    }

    count
}

fn read_input() -> Vec<i64> {
    let path = "./src/input_files/day10.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.split("\n").map(|f| f.parse().unwrap()).collect()
}

pub fn _1() -> i64 {
    let joltages = read_input();
    get_multiplied_adapter_diffs(joltages)
}
pub fn _2() -> i64 {
    let joltages = read_input();
    get_total_of_combinations(joltages)
}
