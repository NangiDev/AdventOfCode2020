pub fn fix_expense_report_2(values: Vec<&str>) -> i32 {
    for v1 in &values {
        for v2 in &values {
            if v1.parse::<i32>().unwrap() + v2.parse::<i32>().unwrap() == 2020 {
                return v1.parse::<i32>().unwrap() * v2.parse::<i32>().unwrap();
            }
        }
    }
    return 0;
}
pub fn fix_expense_report_3(values: Vec<&str>) -> i32 {
    for v1 in &values {
        for v2 in &values {
            for v3 in &values {
                if v1.parse::<i32>().unwrap()
                    + v2.parse::<i32>().unwrap()
                    + v3.parse::<i32>().unwrap()
                    == 2020
                {
                    return v1.parse::<i32>().unwrap()
                        * v2.parse::<i32>().unwrap()
                        * v3.parse::<i32>().unwrap();
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn find_two_values_that_sum_is_2020_in_expense_report() {
        let result = fix_expense_report_2(vec!["1721", "979", "366", "299", "675", "1456"]);
        assert_eq!(result, 514579);
    }

    #[test]
    fn find_three_values_that_sum_is_2020_in_expense_report() {
        let result = fix_expense_report_3(vec!["1721", "979", "366", "299", "675", "1456"]);
        assert_eq!(result, 241861950);
    }
}
