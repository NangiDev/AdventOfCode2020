#[cfg(test)]
mod day_7 {
    use std::{collections::HashMap, vec};

    use adventofcode_2020::day7::{contains_bag, create_bag, extract_bags};

    #[test]
    fn create_bag_from_string() {
        let answers = "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string();

        let mut expected: (String, Vec<String>) = (
            "light red bags contain ".to_string(),
            vec![
                "1 bright white bag, ".to_string(),
                "2 muted yellow bags. ".to_string(),
            ],
        );

        assert_eq!(create_bag(answers), expected);
    }

    #[test]
    fn bag_contains_true() {
        let answers: Vec<String> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];

        let answers = answers.into_iter().map(|f| create_bag(f)).collect();

        assert_eq!(
            contains_bag("light red bags contain ".to_string(), answers),
            true
        );
    }

    // #[test]
    // fn extract_bags_that_can_hold_a_golden_bag_from_string() {
    //     let answers = vec![
    //         "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
    //         "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
    //         "bright white bags contain 1 shiny gold bag.".to_string(),
    //         "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
    //         "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
    //         "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
    //         "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
    //         "faded blue bags contain no other bags.".to_string(),
    //         "dotted black bags contain no other bags.".to_string(),
    //     ];
    //     assert_eq!(extract_bags(answers), 4);
    // }
}
