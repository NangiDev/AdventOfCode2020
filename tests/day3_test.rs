#[cfg(test)]
mod day_3 {
    use adventofcode_2020::day3::{count_trees_in_path, Map, Step};

    fn map() -> Map {
        Map {
            width: 11,
            height: 11,
            grid: vec![
                "..##.......".to_string(),
                "#...#...#..".to_string(),
                ".#....#..#.".to_string(),
                "..#.#...#.#".to_string(),
                ".#...##..#.".to_string(),
                "..#.##.....".to_string(),
                ".#.#.#....#".to_string(),
                ".#........#".to_string(),
                "#.##...#...".to_string(),
                "#...##....#".to_string(),
                ".#..#...#.#".to_string(),
            ],
        }
    }

    #[test]
    fn count_trees_with_step_1_1() {
        let step = Step { x: 1, y: 1 };
        let map = map();
        assert_eq!(count_trees_in_path(step, &map), 2);
    }

    #[test]
    fn count_trees_with_step_3_1() {
        let step = Step { x: 3, y: 1 };
        let map = map();
        assert_eq!(count_trees_in_path(step, &map), 7);
    }

    #[test]
    fn count_trees_with_step_5_1() {
        let step = Step { x: 5, y: 1 };
        let map = map();
        assert_eq!(count_trees_in_path(step, &map), 3);
    }

    #[test]
    fn count_trees_with_step_7_1() {
        let step = Step { x: 7, y: 1 };
        let map = map();
        assert_eq!(count_trees_in_path(step, &map), 4);
    }

    #[test]
    fn count_trees_with_step_1_2() {
        let step = Step { x: 1, y: 2 };
        let map = map();
        assert_eq!(count_trees_in_path(step, &map), 2);
    }

    #[test]
    fn count_multiple_trees() {
        let step = Step { x: 1, y: 1 };
        let map = Map {
            width: 4,
            height: 4,
            grid: vec![
                "....".to_string(),
                ".#..".to_string(),
                "..#.".to_string(),
                "....".to_string(),
            ],
        };
        assert_eq!(count_trees_in_path(step, &map), 2);
    }

    #[test]
    fn count_no_trees() {
        let step = Step { x: 2, y: 1 };
        let map = Map {
            width: 4,
            height: 4,
            grid: vec![
                "....".to_string(),
                ".#..".to_string(),
                "..#.".to_string(),
                "....".to_string(),
            ],
        };
        assert_eq!(count_trees_in_path(step, &map), 0);
    }

    #[test]
    fn count_multiple_trees_wrapping() {
        let step = Step { x: 3, y: 1 };
        let map = Map {
            width: 4,
            height: 4,
            grid: vec![
                "....".to_string(),
                "...#".to_string(),
                "..#.".to_string(),
                ".#..".to_string(),
            ],
        };
        assert_eq!(count_trees_in_path(step, &map), 3);
    }

    #[test]
    fn count_no_trees_wrapping() {
        let step = Step { x: 2, y: 1 };
        let map = Map {
            width: 4,
            height: 4,
            grid: vec![
                "####".to_string(),
                "##.#".to_string(),
                ".###".to_string(),
                "##.#".to_string(),
            ],
        };
        assert_eq!(count_trees_in_path(step, &map), 0);
    }
}
