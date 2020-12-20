#[cfg(test)]
mod day_19 {
    use std::collections::HashMap;

    use adventofcode_2020::day19::rules_to_map;




    
    #[test]
    fn map_simple_example() {
        let input: Vec<String> = vec!["0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\""
            .to_string()];

        let rules_map = rules_to_map(input);

        let mut expected: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        expected.insert(
            "0".to_string(),
            vec![vec!["1".to_string(), "2".to_string()]],
        );
        expected.insert("1".to_string(), vec![vec!["a".to_string()]]);
        expected.insert(
            "2".to_string(),
            vec![
                vec!["1".to_string(), "3".to_string()],
                vec!["3".to_string(), "1".to_string()],
            ],
        );
        expected.insert("3".to_string(), vec![vec!["b".to_string()]]);

        assert_eq!(rules_map, expected);
    }

    #[test]
    fn map_a_bit_more_advanced_example() {
        let input: Vec<String> = vec!["0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\""
            .to_string()];

        let rules_map = rules_to_map(input);

        let mut expected: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        expected.insert(
            "0".to_string(),
            vec![vec!["4".to_string(), "1".to_string(), "5".to_string()]],
        );
        expected.insert(
            "1".to_string(),
            vec![
                vec!["2".to_string(), "3".to_string()],
                vec!["3".to_string(), "2".to_string()],
            ],
        );
        expected.insert(
            "2".to_string(),
            vec![
                vec!["4".to_string(), "4".to_string()],
                vec!["5".to_string(), "5".to_string()],
            ],
        );
        expected.insert(
            "3".to_string(),
            vec![
                vec!["4".to_string(), "5".to_string()],
                vec!["5".to_string(), "4".to_string()],
            ],
        );
        expected.insert("4".to_string(), vec![vec!["a".to_string()]]);
        expected.insert("5".to_string(), vec![vec!["b".to_string()]]);

        assert_eq!(rules_map, expected);
    }
}
