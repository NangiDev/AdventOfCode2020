use crate::{input, utils::Input};

pub fn calculate(mut expr: Vec<String>) -> i64 {
    expr = expr
        .iter_mut()
        .map(|s| s.replace("(", "").replace(")", ""))
        .collect();
    expr.reverse();
    while expr.len() >= 3 {
        let lv = expr.pop().unwrap().parse::<i64>().unwrap();
        let op = expr.pop().unwrap();
        let rv = expr.pop().unwrap().parse::<i64>().unwrap();

        let mut val = "".to_string();
        match op.as_str() {
            "+" => val = (lv + rv).to_string(),
            "*" => val = (lv * rv).to_string(),
            _ => {}
        }

        expr.push(val)
    }

    let exp: Vec<i64> = expr
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    exp.iter().sum()
}

pub fn calculate_with_nested_expressions(mut expr: Vec<String>) -> i64 {
    let mut left_p: Vec<(usize, String)> = expr
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_idx, f)| f.contains('('))
        .collect();

    while let Some(lp) = left_p.last_mut() {
        let mut postfix = "";
        let mut tmp: Vec<String> = vec![];

        for _i in lp.0..expr.len() {
            let val = &expr.remove(lp.0);
            tmp.push(val.clone());
            if val.contains(')') {
                if val.contains("))") {
                    postfix = ")";
                }
                break;
            }
        }
        let val = calculate(tmp);

        if lp.1.contains("((") {
            lp.1 = format!("({}", val);
            expr.insert(lp.0, format!("({}{}", val, postfix));
        } else {
            expr.insert(lp.0, format!("{}{}", val.to_string(), postfix));
            left_p.remove(left_p.len() - 1);
        }
    }

    calculate(expr)
}

pub fn calculate_ordered(mut expr: Vec<String>) -> i64 {
    expr = expr
        .iter_mut()
        .map(|s| s.replace("(", "").replace(")", ""))
        .collect();
    expr.reverse();

    let mut plus: Vec<(usize, String)> = expr
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_idx, f)| f.contains('+'))
        .collect();

    while let Some(lp) = plus.last_mut() {
        let rv = expr.remove(lp.0 - 1).parse::<i64>().unwrap();
        let _op = expr.remove(lp.0 - 1);
        let lv = expr.remove(lp.0 - 1).parse::<i64>().unwrap();

        expr.insert(lp.0 - 1, (lv + rv).to_string());
        plus.remove(plus.len() - 1);
    }

    calculate(expr)
}

pub fn calculate_with_nested_expressions_ordered(mut expr: Vec<String>) -> i64 {
    let mut left_p: Vec<(usize, String)> = expr
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_idx, f)| f.contains('('))
        .collect();

    while let Some(lp) = left_p.last_mut() {
        let mut postfix = "";
        let mut tmp: Vec<String> = vec![];

        for _i in lp.0..expr.len() {
            let val = &expr.remove(lp.0);
            tmp.push(val.clone());
            if val.contains(')') {
                if val.contains("))") {
                    postfix = ")";
                }
                break;
            }
        }
        let val = calculate_ordered(tmp);

        if lp.1.contains("((") {
            lp.1 = format!("({}", val);
            expr.insert(lp.0, format!("({}{}", val, postfix));
        } else {
            expr.insert(lp.0, format!("{}{}", val.to_string(), postfix));
            left_p.remove(left_p.len() - 1);
        }
    }

    calculate_ordered(expr)
}

pub fn _1() -> i64 {
    let input = input!("./src/input_files/day18.txt").as_string();

    let mut count: i64 = 0;
    for i in input {
        let expr: Vec<String> = i.split(' ').map(|s| s.to_string()).collect();
        count += calculate_with_nested_expressions(expr);
    }
    count
}
pub fn _2() -> i64 {
    let input = input!("./src/input_files/day18.txt").as_string();

    let mut count: i64 = 0;
    for i in input {
        let expr: Vec<String> = i.split(' ').map(|s| s.to_string()).collect();
        count += calculate_with_nested_expressions_ordered(expr);
    }
    count
}
