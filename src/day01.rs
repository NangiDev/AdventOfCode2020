use crate::{input, run_day, utils::Input};

pub fn fix_expense_report_2(values: Vec<i32>) -> i32 {
    for v1 in &values {
        for v2 in &values {
            if v1 + v2 == 2020 {
                return v1 * v2;
            }
        }
    }
    return 0;
}
pub fn fix_expense_report_3(values: Vec<i32>) -> i32 {
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

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day1.txt");
    fix_expense_report_2(input.as_int())
}

pub fn _2() -> i32 {
    let input = input!("./src/input_files/day1.txt");
    fix_expense_report_3(input.as_int())
}

pub fn print() {
    run_day!(crate::day01);
}
