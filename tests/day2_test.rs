#[cfg(test)]
mod day_2 {
    use adventofcode_2020::day2::{
        check_official_password_validity, check_unofficial_password_validity,
        count_valid_passwords_according_to_official_toboggan_policy,
        count_valid_passwords_according_to_unofficial_toboggan_policy,
    };
    use std::vec;

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
