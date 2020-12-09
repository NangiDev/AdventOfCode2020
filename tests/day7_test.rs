#[cfg(test)]
mod day_7 {
    use std::{collections::HashMap, vec};

    use adventofcode_2020::day7::{
        add_bag, contains_gold_bag, convert_to_map, convert_to_map_with_count_of_bag,
        count_content_of_bag, count_golden_bags,
    };

    #[test]
    fn add_light_read_bag() {
        let bags = "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string();

        let mut expected: HashMap<String, Vec<String>> = HashMap::new();
        expected.insert(
            "light red".to_string(),
            vec!["bright white".to_string(), "muted yellow".to_string()],
        );

        assert_eq!(add_bag(HashMap::new(), &bags, false), expected);
    }

    #[test]
    fn add_dark_orange_bag() {
        let bags = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string();

        let mut expected: HashMap<String, Vec<String>> = HashMap::new();
        expected.insert(
            "dark orange".to_string(),
            vec!["bright white".to_string(), "muted yellow".to_string()],
        );

        assert_eq!(add_bag(HashMap::new(), &bags, false), expected);
    }

    #[test]
    fn add_light_read_bag_with_count() {
        let bags = "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string();

        let mut expected: HashMap<String, Vec<String>> = HashMap::new();
        expected.insert(
            "light red".to_string(),
            vec![
                "bright white".to_string(),
                "muted yellow".to_string(),
                "muted yellow".to_string(),
            ],
        );

        assert_eq!(add_bag(HashMap::new(), &bags, true), expected);
    }

    #[test]
    fn add_dark_orange_bag_with_count() {
        let bags = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string();

        let mut expected: HashMap<String, Vec<String>> = HashMap::new();
        expected.insert(
            "dark orange".to_string(),
            vec![
                "bright white".to_string(),
                "bright white".to_string(),
                "bright white".to_string(),
                "muted yellow".to_string(),
                "muted yellow".to_string(),
                "muted yellow".to_string(),
                "muted yellow".to_string(),
            ],
        );

        assert_eq!(add_bag(HashMap::new(), &bags, true), expected);
    }

    #[test]
    fn contains_gold_bag_true() {
        let bags = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string();
        let map = add_bag(HashMap::new(), &bags, false);
        let bag = map.get("muted yellow");
        assert_eq!(contains_gold_bag(bag, &map) > 0, true);
    }

    #[test]
    fn contains_gold_bag_false() {
        let bags = "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string();
        let map = add_bag(HashMap::new(), &bags, false);
        let bag = map.get("light red");
        assert_eq!(contains_gold_bag(bag, &map) > 0, false);
    }

    #[test]
    fn convert_input_to_map() {
        let bags: Vec<String> = vec![
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

        let mut expected: HashMap<String, Vec<String>> = HashMap::new();
        for b in &bags {
            expected = add_bag(expected, b, false);
        }

        assert_eq!(convert_to_map(bags), expected);
    }

    #[test]
    fn count_golden_bags_to_sum_4() {
        let bags: Vec<String> = vec![
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

        let bags = convert_to_map(bags);

        assert_eq!(count_golden_bags(bags), 4);
    }

    #[test]
    fn count_golden_bags_to_sum_5() {
        let bags: Vec<String> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 dark olive bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 shiny gold bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];

        let bags = convert_to_map(bags);

        assert_eq!(count_golden_bags(bags), 5);
    }

    #[test]
    fn shiny_gold_bag_contains_32_bags() {
        let bags: Vec<String> = vec![
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

        let bags = convert_to_map_with_count_of_bag(bags);

        assert_eq!(count_content_of_bag(&bags, &"shiny gold".to_string()), 32);
    }
}
