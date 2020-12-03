use std::fs;

fn fix_expense_report_2(values: Vec<i32>) -> i32 {
    for v1 in &values {
        for v2 in &values {
            if v1 + v2 == 2020 {
                return v1 * v2;
            }
        }
    }
    return 0;
}
fn fix_expense_report_3(values: Vec<i32>) -> i32 {
    for v1 in &values {
        for v2 in &values {
            for v3 in &values {
                if v1 + v2 + v3 == 2020 {
                    return v1 * v2 * v3;
                }
            }
        }
    }
    return 0;
}

fn read_input() -> Vec<i32> {
    let path = "./src/input_files/day1.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.split("\n").map(|f| f.parse().unwrap()).collect()
}

pub fn _1() -> i32 {
    fix_expense_report_2(read_input())
}
pub fn _2() -> i32 {
    fix_expense_report_3(read_input())
}

#[cfg(test)]
mod tests {
    use super::*;
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
