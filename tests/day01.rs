#[cfg(test)]
mod day01_test {
    use adventofcode_2020::day01::{fix_expense_report_2, fix_expense_report_3};
    use std::vec;

    #[test]
    fn find_two_values_that_sum_is_2020_in_expense_report() {
        let result = fix_expense_report_2(vec![1721, 979, 366, 299, 675, 1456]);
        assert_eq!(result, 514579);
    }

    #[test]
    fn find_three_values_that_sum_is_2020_in_expense_report() {
        let result = fix_expense_report_3(vec![1721, 979, 366, 299, 675, 1456]);
        assert_eq!(result, 241861950);
    }
}
