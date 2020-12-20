#[cfg(test)]
mod day_2 {
    use adventofcode_2020::day02::{
        check_official_password_validity, check_unofficial_password_validity,
        count_valid_passwords_according_to_official_toboggan_policy,
        count_valid_passwords_according_to_unofficial_toboggan_policy,
    };
    use std::vec;

    #[test]
    fn check_if_password_is_valid_according_to_unofficial_policy() {
        assert_eq!(
            check_unofficial_password_validity("1-3 a: abcde".to_string()),
            true
        );
        assert_eq!(
            check_unofficial_password_validity("1-3 b: cdefg".to_string()),
            false
        );
        assert_eq!(
            check_unofficial_password_validity("2-9 c: ccccccccc".to_string()),
            true
        );
    }

    #[test]
    fn find_two_valid_passwords_according_to_unofficial_policy() {
        let result = count_valid_passwords_according_to_unofficial_toboggan_policy(vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn check_if_password_is_valid_according_to_official_policy() {
        assert_eq!(
            check_official_password_validity("1-3 a: abcde".to_string()),
            true
        );
        assert_eq!(
            check_official_password_validity("1-3 b: cdefg".to_string()),
            false
        );
        assert_eq!(
            check_official_password_validity("2-9 c: ccccccccc".to_string()),
            false
        );
    }

    #[test]
    fn find_one_valid_passwords_according_to_official_policy() {
        let result = count_valid_passwords_according_to_official_toboggan_policy(vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ]);
        assert_eq!(result, 1);
    }
}
