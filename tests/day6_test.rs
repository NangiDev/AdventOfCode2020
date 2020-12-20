#[cfg(test)]
mod day_6 {
    use adventofcode_2020::day06::{sum_letters, sum_unique_letters};

    #[test]
    fn sum_of_unique_answers_is_11() {
        let answers = vec![
            "abc".to_string(),
            "a\nb\nc".to_string(),
            "ab\nac".to_string(),
            "a\na\na\na".to_string(),
            "b".to_string(),
        ];
        assert_eq!(sum_unique_letters(answers), 11);
    }

    #[test]
    fn sum_of_common_answers_is_6() {
        let answers = vec![
            "abc".to_string(),
            "a\nb\nc".to_string(),
            "ab\nac".to_string(),
            "a\na\na\na".to_string(),
            "b".to_string(),
        ];
        assert_eq!(sum_letters(answers), 6);
    }
}
