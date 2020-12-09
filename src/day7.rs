use std::{collections::HashMap, fs};

fn read_input() -> Vec<String> {
    let path = "./src/input_files/day7.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let batches: Vec<String> = contents.split_whitespace().map(|s| s.to_string()).collect();
    batches
}

pub fn create_bag(bag: String) -> (String, Vec<String>) {
    let split: Vec<String> = bag.split_whitespace().map(|s| s.to_string()).collect();

    let split: Vec<&[String]> = split.chunks(4).collect();
    let split: Vec<String> = split
        .into_iter()
        .map(|g| {
            let mut string: String = "".to_string();
            for s in g {
                string.push_str(s);
                string.push(' ');
            }
            string
        })
        .collect();

    let mut value: Vec<String> = vec![];
    for i in 1..split.len() {
        value.push(split[i].clone());
    }

    (split[0].clone(), value)
}

pub fn contains_bag(bag_key: String, bags: HashMap<String, Vec<String>>) -> bool {
    let contained = bags.get(&bag_key).unwrap();
    println!("{:?}", contained);

    

    false
}

pub fn extract_bags(answers: Vec<String>) -> i32 {
    // let bags: Vec<HashMap<String, Vec<String>>> =
    //     answers.into_iter().map(|b| create_bag(b)).collect();

    let mut count = 0;

    count
}

pub fn _1() -> i32 {
    let answers = read_input();
    extract_bags(answers);
    0
}

pub fn _2() -> i32 {
    0
}
