use std::fs;

use crate::run_day;

fn read_input() -> Vec<String> {
    let path = "./src/input_files/day6.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let batches: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();
    batches
}

pub fn sum_unique_letters(answers: Vec<String>) -> i32 {
    answers
        .into_iter()
        .map(|mut a| {
            a = a.replace("\n", "");
            let mut chars: Vec<char> = a.chars().collect();
            chars.sort();
            chars.dedup();
            chars.len() as i32
        })
        .sum()
}

pub fn sum_letters(answers: Vec<String>) -> i32 {
    let letters: &str = "abcdefghijklmnopqrstuvwxyz";
    answers
        .into_iter()
        .map(|mut a| {
            let mut count = 0;
            let members = a.split("\n").count();
            let mut chars: Vec<char> = a.chars().collect();
            chars.sort();
            a = chars.into_iter().map(|i| i.to_string()).collect::<String>();

            for l in letters.chars() {
                if a.matches(l).count() == members {
                    count += 1;
                }
            }

            count
        })
        .sum()
}

pub fn _1() -> i32 {
    let answers = read_input();
    sum_unique_letters(answers)
}

pub fn _2() -> i32 {
    let answers = read_input();
    sum_letters(answers)
}

pub fn print() {
    run_day!(crate::day06);
}
