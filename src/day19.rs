use std::collections::HashMap;

use crate::{debug, input, utils::Input};

pub fn rules_to_map(input: Vec<String>) -> HashMap<String, Vec<Vec<String>>> {
    let rules: Vec<String> = input[0].split('\n').map(From::from).collect();
    let mut rules_map: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for r in rules {
        let values: Vec<String> = r.split(':').map(From::from).collect();
        let key = values[0].clone();

        let references: Vec<String> = values[1].split('|').map(|s| s.trim().to_string()).collect();

        let mut val_list: Vec<Vec<String>> = vec![];

        for r in &references {
            let r: Vec<String> = r.replace('"', "").split(' ').map(From::from).collect();
            val_list.push(r);
        }

        rules_map.insert(key, val_list);
    }

    rules_map
}

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day19.txt").as_group();

    let rules_map = rules_to_map(input);
    println!("{:?}", rules_map);
    0
}
pub fn _2() -> i32 {
    let _input = input!("./src/input_files/day19.txt").as_string();
    0
}
