#[cfg(test)]
mod day_10 {
    use adventofcode_2020::day10::{
        get_built_in_joltage, get_multiplied_adapter_diffs, get_total_of_combinations,
    };
    use std::vec;

    #[test]
    fn find_built_in_joltage_22() {
        let result = get_built_in_joltage(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
        assert_eq!(result, 22);
    }

    #[test]
    fn number_of_jolt_diff_is_35() {
        let result = get_multiplied_adapter_diffs(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
        assert_eq!(result, 5 * 7);
    }

    #[test]
    fn number_of_jolt_diff_is_220() {
        let result = get_multiplied_adapter_diffs(vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]);
        assert_eq!(result, 22 * 10);
    }

    #[test]
    fn number_of_total_combination_is_8() {
        let result = get_total_of_combinations(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
        assert_eq!(result, 8);
    }

    #[test]
    fn number_of_total_combination_is_19208() {
        let result = get_total_of_combinations(vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]);
        assert_eq!(result, 19208);
    }
}
