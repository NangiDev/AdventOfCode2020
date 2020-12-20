use std::{collections::HashMap, fs};

use crate::run_day;

fn read_input() -> Vec<String> {
    let path = "./src/input_files/day7.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let batches: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    batches
}

pub fn add_bag(
    mut map: HashMap<String, Vec<String>>,
    bag: &String,
    keep_count: bool,
) -> HashMap<String, Vec<String>> {
    if bag.contains("no other bags") {
        return map;
    }

    let split: Vec<String> = bag.split_whitespace().map(|s| s.to_string()).collect();

    let split: Vec<Vec<String>> = split
        .chunks(4)
        .map(|c| {
            let mut v: Vec<String> = vec![];
            for s in c {
                v.push(s.clone());
            }
            v
        })
        .collect();

    let mut children: Vec<String> = vec![];
    if keep_count {
        for i in 1..split.len() {
            for _y in 0..split[i][0].parse::<i32>().unwrap() {
                let mut value: String = split[i][1].clone();
                value.push_str(" ");
                value.push_str(&split[i][2].clone());
                children.push(value);
            }
        }
    } else {
        for i in 1..split.len() {
            let mut value: String = split[i][1].clone();
            value.push_str(" ");
            value.push_str(&split[i][2].clone());
            children.push(value);
        }
    }

    let mut key: String = split[0][0].clone();
    key.push_str(" ");
    key.push_str(&split[0][1].clone());
    map.insert(key, children);
    map
}

pub fn convert_to_map(bags: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for b in bags {
        map = add_bag(map, &b, false);
    }
    map
}

pub fn convert_to_map_with_count_of_bag(bags: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for b in bags {
        map = add_bag(map, &b, true);
    }
    map
}

pub fn contains_gold_bag(bag: Option<&Vec<String>>, map: &HashMap<String, Vec<String>>) -> i32 {
    if bag.is_none() {
        return 0;
    }
    let bag = bag.unwrap();

    let mut result = 0;
    for b in bag {
        if "shiny gold".to_string() == *b || result > 0 {
            return 1;
        }
        result += contains_gold_bag(map.get(b), &map);
    }

    return result;
}

pub fn count_golden_bags(map: HashMap<String, Vec<String>>) -> i32 {
    let mut count = 0;

    for m in &map {
        if "shiny gold".to_string() != *m.0 && contains_gold_bag(Some(&m.1), &map) > 0 {
            count += 1;
        }
    }
    count
}

pub fn count_content_of_bag(map: &HashMap<String, Vec<String>>, bag: &String) -> i32 {
    let bag = map.get(bag);
    if bag.is_none() {
        return 0;
    }

    let bag = bag.unwrap();
    let mut count = bag.len() as i32;

    for b in bag {
        count += count_content_of_bag(map, b);
    }

    count
}

pub fn _1() -> i32 {
    let input = read_input();
    let input = convert_to_map(input);
    count_golden_bags(input)
}

pub fn _2() -> i32 {
    let input = read_input();
    let input = convert_to_map_with_count_of_bag(input);
    count_content_of_bag(&input, &"shiny gold".to_string())
}

pub fn print() {
    run_day!(crate::day7);
}
