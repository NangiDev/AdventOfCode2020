use std::fs;

fn check_unofficial_password_validity(password: &str) -> bool {
    let filtered: String = str::replace(password, ": ", ":");
    let splice: Vec<&str> = filtered.split(&[' ', ':', '-'][..]).collect();

    let count = splice[3].matches(splice[2]).count() as i32;

    count >= splice[0].parse::<i32>().unwrap() && count <= splice[1].parse::<i32>().unwrap()
}

fn count_valid_passwords_according_to_unofficial_toboggan_policy(passwords: Vec<&str>) -> i32 {
    let mut count = 0;
    for p in passwords {
        if check_unofficial_password_validity(p) {
            count += 1;
        }
    }

    count
}

fn check_official_password_validity(password: &str) -> bool {
    let filtered: String = str::replace(password, ": ", ":");
    let splice: Vec<&str> = filtered.split(&[' ', ':', '-'][..]).collect();

    let start_index = splice[0].parse::<i32>().unwrap() as usize;
    let end_index = splice[1].parse::<i32>().unwrap() as usize;

    let first = &splice[3][start_index - 1..start_index];
    let second = &splice[3][end_index - 1..end_index];

    let expected = splice[2];

    (first == expected || second == expected) && !(first == expected && second == expected)
}

fn count_valid_passwords_according_to_official_toboggan_policy(passwords: Vec<&str>) -> i32 {
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn check_if_password_is_valid_according_to_unofficial_policy() {
        assert_eq!(check_unofficial_password_validity("1-3 a: abcde"), true);
        assert_eq!(check_unofficial_password_validity("1-3 b: cdefg"), false);
        assert_eq!(check_unofficial_password_validity("2-9 c: ccccccccc"), true);
    }

    #[test]
    fn find_two_valid_passwords_according_to_unofficial_policy() {
        let result = count_valid_passwords_according_to_unofficial_toboggan_policy(vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn check_if_password_is_valid_according_to_official_policy() {
        assert_eq!(check_official_password_validity("1-3 a: abcde"), true);
        assert_eq!(check_official_password_validity("1-3 b: cdefg"), false);
        assert_eq!(check_official_password_validity("2-9 c: ccccccccc"), false);
    }

    #[test]
    fn find_one_valid_passwords_according_to_official_policy() {
        let result = count_valid_passwords_according_to_official_toboggan_policy(vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ]);
        assert_eq!(result, 1);
    }
}
