#[cfg(test)]
mod day_18 {
    use adventofcode_2020::day18::{
        calculate, calculate_ordered, calculate_with_nested_expressions,
        calculate_with_nested_expressions_ordered,
    };

    #[test]
    fn calc_ordered_expression_to_be_231() {
        let expression = "1 + 2 * 3 + 4 * 5 + 6".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_ordered(exp);
        assert_eq!(result, 231);
    }

    #[test]
    fn calc_ordered_expression_to_be_51() {
        let expression = "1 + (2 * 3) + (4 * (5 + 6))".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions_ordered(exp);
        assert_eq!(result, 51);
    }

    #[test]
    fn calc_ordered_expression_to_be_46() {
        let expression = "2 * 3 + (4 * 5)".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions_ordered(exp);
        assert_eq!(result, 46);
    }

    #[test]
    fn calc_ordered_expression_to_be_1445() {
        let expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions_ordered(exp);
        assert_eq!(result, 1445);
    }

    #[test]
    fn calc_ordered_expression_to_be_669060() {
        let expression = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions_ordered(exp);
        assert_eq!(result, 669060);
    }

    #[test]
    fn calc_ordered_expression_to_be_23340() {
        let expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions_ordered(exp);
        assert_eq!(result, 23340);
    }

    #[test]
    fn calc_expression_to_be_71() {
        let expression = "1 + 2 * 3 + 4 * 5 + 6".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate(exp);
        assert_eq!(result, 71);
    }

    #[test]
    fn calc_expression_to_be_51() {
        let expression = "1 + (2 * 3) + (4 * (5 + 6))".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions(exp);
        assert_eq!(result, 51);
    }

    #[test]
    fn calc_expression_to_be_26() {
        let expression = "2 * 3 + (4 * 5)".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions(exp);
        assert_eq!(result, 26);
    }

    #[test]
    fn calc_expression_to_be_437() {
        let expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions(exp);
        assert_eq!(result, 437);
    }

    #[test]
    fn calc_expression_to_be_12240() {
        let expression = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions(exp);
        assert_eq!(result, 12240);
    }

    #[test]
    fn calc_expression_to_be_13632() {
        let expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions(exp);
        assert_eq!(result, 13632);
    }

    #[test]
    fn calc_expression_to_be_81() {
        let expression = "1 + (2 * (3 + 4)) * 5 + 6".to_string();
        let exp: Vec<String> = expression.split(' ').map(|s| s.to_string()).collect();
        let result = calculate_with_nested_expressions(exp);
        assert_eq!(result, 81);
    }
}
