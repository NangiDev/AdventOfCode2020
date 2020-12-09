#[cfg(test)]
mod day_9 {
    use adventofcode_2020::day9::{find_first_invalid, find_set, is_valid_number};

    #[test]
    fn preamble_of_1_to_25_next_number_26_is_valid() {
        let input: Vec<i64> = (1..26).collect();
        assert_eq!(is_valid_number(&input, 26), true);
    }

    #[test]
    fn preamble_of_1_to_25_next_number_49_is_valid() {
        let input: Vec<i64> = (1..26).collect();
        assert_eq!(is_valid_number(&input, 49), true);
    }

    #[test]
    fn preamble_of_1_to_25_next_number_100_is_invalid() {
        let input: Vec<i64> = (1..26).collect();
        assert_eq!(is_valid_number(&input, 100), false);
    }

    #[test]
    fn preamble_of_1_to_25_next_number_50_is_invalid() {
        let input: Vec<i64> = (1..26).collect();
        assert_eq!(is_valid_number(&input, 50), false);
    }

    #[test]
    fn preamble_of_1_to_19_and_21_to_25_plus_45_next_number_26_is_valid() {
        let mut input: Vec<i64> = (1..26).collect();
        input[19] = 45;
        assert_eq!(is_valid_number(&input, 26), true);
    }

    #[test]
    fn preamble_of_1_to_19_and_21_to_25_plus_45_next_number_65_is_invalid() {
        let mut input: Vec<i64> = (1..26).collect();
        input[19] = 45;
        assert_eq!(is_valid_number(&input, 65), false);
    }

    #[test]
    fn preamble_of_1_to_19_and_21_to_25_plus_45_next_number_64_is_valid() {
        let mut input: Vec<i64> = (1..26).collect();
        input[19] = 45;
        assert_eq!(is_valid_number(&input, 64), true);
    }

    #[test]
    fn preamble_of_1_to_19_and_21_to_25_plus_45_next_number_66_is_valid() {
        let mut input: Vec<i64> = (1..26).collect();
        input[19] = 45;
        assert_eq!(is_valid_number(&input, 66), true);
    }

    #[test]
    fn preamble_of_5_127_is_invalid() {
        let read_input: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let preample = read_input[..5].to_vec();
        let numbers = read_input[5..].to_vec();

        assert_eq!(find_first_invalid(preample, numbers), 127);
    }

    #[test]
    fn find_smallest_15_and_largest_40_that_set_sums_to_127() {
        let read_input: Vec<i64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_set(read_input, 127), 62);
    }
}
