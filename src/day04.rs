use regex::Regex;
use std::collections::HashMap;

use crate::{input, run_day, utils::Input};

pub fn is_valid_passport(map: &HashMap<String, String>) -> bool {
    map.contains_key("ecl")
        && map.contains_key("pid")
        && map.contains_key("eyr")
        && map.contains_key("hcl")
        && map.contains_key("byr")
        && map.contains_key("iyr")
        && map.contains_key("hgt")
}

pub fn is_fields_valid(map: &HashMap<String, String>) -> bool {
    for (key, value) in &*map {
        match key.as_str() {
            "byr" => {
                let val = value.parse::<i32>().unwrap();
                if !(1920..=2002).contains(&val) {
                    return false;
                }
            }
            "iyr" => {
                let val = value.parse::<i32>().unwrap();
                if !(2010..=2020).contains(&val) {
                    return false;
                }
            }
            "eyr" => {
                let val = value.parse::<i32>().unwrap();
                if !(2020..=2030).contains(&val) {
                    return false;
                }
            }
            "hgt" => {
                let unit: String = value.chars().filter(|c| c.is_alphabetic()).collect();
                if unit == "in" {
                    let t: String = value.chars().filter(|c| c.is_digit(10)).collect();
                    if t.parse::<i32>().unwrap() < 59 || t.parse::<i32>().unwrap() > 76 {
                        return false;
                    }
                } else if unit == "cm" {
                    let t: String = value.chars().filter(|c| c.is_digit(10)).collect();
                    if t.parse::<i32>().unwrap() < 150 || t.parse::<i32>().unwrap() > 193 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                let re = Regex::new(r"#[0-9,a-f]{6}").unwrap();
                if !re.is_match(value) {
                    return false;
                }
            }
            "ecl" => {
                if value != "amb"
                    && value != "blu"
                    && value != "brn"
                    && value != "gry"
                    && value != "grn"
                    && value != "hzl"
                    && value != "oth"
                {
                    return false;
                }
            }
            "pid" => {
                if value.len() != 9 {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

pub fn filter_complete_passports(
    passports: Vec<HashMap<String, String>>,
) -> Vec<HashMap<String, String>> {
    passports
        .into_iter()
        .filter(|p| is_valid_passport(p))
        .collect()
}
pub fn filter_valid_passports(
    read_input: Vec<HashMap<String, String>>,
) -> Vec<HashMap<String, String>> {
    read_input
        .into_iter()
        .filter(|p| is_fields_valid(p))
        .collect()
}

fn input_to_hashmap(batches: Vec<String>) -> Vec<HashMap<String, String>> {
    let mut passports: Vec<HashMap<String, String>> = vec![];
    for b in batches {
        let mut map: HashMap<String, String> = HashMap::new();
        let pairs: Vec<String> = b.split_whitespace().map(|s| s.to_string()).collect();
        for p in pairs {
            let pair: Vec<String> = p.split(':').map(|s| s.to_string()).collect();
            map.insert(pair[0].clone(), pair[1].clone());
        }
        passports.push(map);
    }
    passports
}

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day4.txt");
    let hash_map = input_to_hashmap(input.as_group());
    filter_complete_passports(hash_map).len() as i32
}

pub fn _2() -> i32 {
    let input = input!("./src/input_files/day4.txt");
    let hash_map = input_to_hashmap(input.as_group());
    let valid_input = filter_complete_passports(hash_map);
    filter_valid_passports(valid_input).len() as i32
}

pub fn print() {
    run_day!(crate::day04);
}
