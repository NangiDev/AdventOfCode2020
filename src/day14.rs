use std::collections::HashMap;

use crate::{input, utils::Input};

pub fn apply_mask(mask: String, number: i64) -> i64 {
    let mut bytes = format!("{:#036b}", number).chars().collect::<Vec<char>>();
    for (idx, m) in mask.chars().enumerate() {
        match m {
            '1' | '0' => {
                if m != bytes[idx] {
                    bytes[idx] = m;
                }
            }
            _ => {}
        }
    }
    convert_byte(bytes.iter().collect::<String>())
}

fn get_binary_byte(idx: usize) -> u64 {
    let mut res: u64 = 1;
    for _i in 0..idx {
        res *= 2;
    }
    res
}

pub fn convert_byte(byte: String) -> i64 {
    let mut number: i64 = 0;
    for (idx, c) in byte.chars().enumerate() {
        let idx = byte.len() - idx - 1;
        if let Some(b) = c.to_digit(10) {
            number += (b as u64 * get_binary_byte(idx)) as i64;
        }
    }
    number
}

pub fn sum_memory(input: Vec<String>) -> i64 {
    let mut memory: HashMap<String, i64> = HashMap::new();
    let mut mask = "".to_string();
    for i in &input {
        if i.starts_with("mask") {
            mask = i[7..].to_string();
        } else {
            let start_bytes = i.find('[').unwrap_or(0) + 1;
            let end_bytes = i.find(']').unwrap_or(i.len() - 1);
            let mem = i[start_bytes..end_bytes].to_string();
            let number = &i[end_bytes + 4..].parse::<i64>().unwrap();
            let number = apply_mask(mask.clone(), *number);

            memory.insert(mem, number);
        }
    }
    memory.values().sum()
}

pub fn _1() -> i64 {
    let input = input!("./src/input_files/day14.txt").as_string();
    sum_memory(input)
}

pub fn _2() -> i64 {
    let _input = input!("./src/input_files/day14.txt").as_string();
    println!("Not implemented yet");
    0
}
