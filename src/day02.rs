use crate::{input, run_day, utils::Input};

pub fn check_unofficial_password_validity(password: String) -> bool {
    let filtered: String = password.replace(": ", ":");
    let splice: Vec<&str> = filtered.split(&[' ', ':', '-'][..]).collect();

    let count = splice[3].matches(splice[2]).count() as i32;

    count >= splice[0].parse::<i32>().unwrap() && count <= splice[1].parse::<i32>().unwrap()
}

pub fn count_valid_passwords_according_to_unofficial_toboggan_policy(
    passwords: Vec<String>,
) -> i32 {
    let mut count = 0;
    for p in passwords {
        if check_unofficial_password_validity(p) {
            count += 1;
        }
    }

    count
}

pub fn check_official_password_validity(password: String) -> bool {
    let filtered: String = password.replace(": ", ":");
    let splice: Vec<&str> = filtered.split(&[' ', ':', '-'][..]).collect();

    let start_index = splice[0].parse::<i32>().unwrap() as usize;
    let end_index = splice[1].parse::<i32>().unwrap() as usize;

    let first = &splice[3][start_index - 1..start_index];
    let second = &splice[3][end_index - 1..end_index];

    let expected = splice[2];

    (first == expected || second == expected) && !(first == expected && second == expected)
}

pub fn count_valid_passwords_according_to_official_toboggan_policy(passwords: Vec<String>) -> i32 {
    let mut count = 0;
    for p in passwords {
        if check_official_password_validity(p) {
            count += 1;
        }
    }

    count
}

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day2.txt");
    count_valid_passwords_according_to_unofficial_toboggan_policy(input.as_string())
}
pub fn _2() -> i32 {
    let input = input!("./src/input_files/day2.txt");
    count_valid_passwords_according_to_official_toboggan_policy(input.as_string())
}

pub fn print() {
    run_day!(crate::day02);
}
