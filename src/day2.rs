use std::fs;

pub fn check_unofficial_password_validity(password: &str) -> bool {
    let filtered: String = str::replace(password, ": ", ":");
    let splice: Vec<&str> = filtered.split(&[' ', ':', '-'][..]).collect();

    let count = splice[3].matches(splice[2]).count() as i32;

    count >= splice[0].parse::<i32>().unwrap() && count <= splice[1].parse::<i32>().unwrap()
}

pub fn count_valid_passwords_according_to_unofficial_toboggan_policy(passwords: Vec<&str>) -> i32 {
    let mut count = 0;
    for p in passwords {
        if check_unofficial_password_validity(p) {
            count += 1;
        }
    }

    count
}

pub fn check_official_password_validity(password: &str) -> bool {
    let filtered: String = str::replace(password, ": ", ":");
    let splice: Vec<&str> = filtered.split(&[' ', ':', '-'][..]).collect();

    let start_index = splice[0].parse::<i32>().unwrap() as usize;
    let end_index = splice[1].parse::<i32>().unwrap() as usize;

    let first = &splice[3][start_index - 1..start_index];
    let second = &splice[3][end_index - 1..end_index];

    let expected = splice[2];

    (first == expected || second == expected) && !(first == expected && second == expected)
}

pub fn count_valid_passwords_according_to_official_toboggan_policy(passwords: Vec<&str>) -> i32 {
    let mut count = 0;
    for p in passwords {
        if check_official_password_validity(p) {
            count += 1;
        }
    }

    count
}

fn read_input() -> Vec<String> {
    let path = "./src/input_files/day2.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.split("\n").map(|s| s.to_string()).collect()
}

pub fn _1() -> i32 {
    let passwords = read_input();
    count_valid_passwords_according_to_unofficial_toboggan_policy(
        passwords.iter().map(AsRef::as_ref).collect(),
    )
}
pub fn _2() -> i32 {
    let passwords = read_input();
    count_valid_passwords_according_to_official_toboggan_policy(
        passwords.iter().map(AsRef::as_ref).collect(),
    )
}
