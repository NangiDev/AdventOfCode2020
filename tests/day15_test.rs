#[cfg(test)]
mod day_15 {
    use adventofcode_2020::day15::add_next_spoken;
    use adventofcode_2020::day15::get_nth_number;
    use std::collections::HashMap;
    use std::vec;

    #[test]
    fn expect_fourth_spoken_to_be_0() {
        let latest_map = &mut HashMap::new();
        latest_map.insert(0, 0);
        latest_map.insert(3, 1);
        latest_map.insert(6, 2);

        let result: Vec<i32> = add_next_spoken(vec![0, 3, 6], latest_map, &mut HashMap::new());
        assert_eq!(result, vec![0, 3, 6, 0]);
    }

    #[test]
    fn expect_fifth_spoken_to_be_3() {
        let latest_map = &mut HashMap::new();
        latest_map.insert(0, 3);
        latest_map.insert(3, 1);
        latest_map.insert(6, 2);

        let previous_map = &mut HashMap::new();
        previous_map.insert(0, 0);

        let result: Vec<i32> = add_next_spoken(vec![0, 3, 6, 0], latest_map, previous_map);
        assert_eq!(result, vec![0, 3, 6, 0, 3]);
    }

    #[test]
    fn expect_sixth_spoken_to_be_3() {
        let latest_map = &mut HashMap::new();
        latest_map.insert(0, 3);
        latest_map.insert(3, 4);
        latest_map.insert(6, 2);

        let previous_map = &mut HashMap::new();
        previous_map.insert(0, 0);
        previous_map.insert(3, 1);

        let result: Vec<i32> = add_next_spoken(vec![0, 3, 6, 0, 3], latest_map, previous_map);
        assert_eq!(result, vec![0, 3, 6, 0, 3, 3]);
    }

    #[test]
    fn expect_seventh_spoken_to_be_1() {
        let latest_map = &mut HashMap::new();
        latest_map.insert(0, 3);
        latest_map.insert(3, 5);
        latest_map.insert(6, 2);

        let previous_map = &mut HashMap::new();
        previous_map.insert(0, 0);
        previous_map.insert(3, 4);

        let result: Vec<i32> = add_next_spoken(vec![0, 3, 6, 0, 3, 3], latest_map, previous_map);
        assert_eq!(result, vec![0, 3, 6, 0, 3, 3, 1]);
    }

    #[test]
    fn expect_eigth_spoken_to_be_0() {
        let latest_map = &mut HashMap::new();
        latest_map.insert(0, 3);
        latest_map.insert(1, 6);
        latest_map.insert(3, 5);
        latest_map.insert(6, 2);

        let previous_map = &mut HashMap::new();
        previous_map.insert(0, 0);
        previous_map.insert(3, 4);

        let result: Vec<i32> = add_next_spoken(vec![0, 3, 6, 0, 3, 3, 1], latest_map, previous_map);
        assert_eq!(result, vec![0, 3, 6, 0, 3, 3, 1, 0]);
    }

    #[test]
    fn expect_ninths_spoken_to_be_4() {
        let latest_map = &mut HashMap::new();
        latest_map.insert(0, 7);
        latest_map.insert(1, 6);
        latest_map.insert(3, 5);
        latest_map.insert(6, 2);

        let previous_map = &mut HashMap::new();
        previous_map.insert(0, 3);
        previous_map.insert(3, 4);

        let result: Vec<i32> =
            add_next_spoken(vec![0, 3, 6, 0, 3, 3, 1, 0], latest_map, previous_map);
        assert_eq!(result, vec![0, 3, 6, 0, 3, 3, 1, 0, 4]);
    }

    #[test]
    fn expect_tenth_spoken_to_be_0() {
        let latest_map = &mut HashMap::new();
        latest_map.insert(0, 7);
        latest_map.insert(1, 6);
        latest_map.insert(3, 5);
        latest_map.insert(4, 8);
        latest_map.insert(6, 2);

        let previous_map = &mut HashMap::new();
        previous_map.insert(0, 3);
        previous_map.insert(3, 4);

        let result: Vec<i32> =
            add_next_spoken(vec![0, 3, 6, 0, 3, 3, 1, 0, 4], latest_map, previous_map);
        assert_eq!(result, vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0]);
    }

    #[test]
    fn expect_fifth_spoken_to_be_3_after_2_rounds() {
        let latest_map = &mut HashMap::new();
        latest_map.insert(0, 0);
        latest_map.insert(3, 1);
        latest_map.insert(6, 2);

        let previous_map = &mut HashMap::new();

        let mut result: Vec<i32> = vec![0, 3, 6];
        result = add_next_spoken(result, latest_map, previous_map);

        result = add_next_spoken(result, latest_map, previous_map);

        assert_eq!(result, vec![0, 3, 6, 0, 3]);
    }

    #[test]
    fn expect_2020th_spoken_to_be_436() {
        let result: i32 = get_nth_number(2020, vec![0, 3, 6]);
        assert_eq!(result, 436);
    }

    #[test]
    fn expect_2020th_spoken_to_be_1() {
        let result: i32 = get_nth_number(2020, vec![1, 3, 2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn expect_2020th_spoken_to_be_10() {
        let result: i32 = get_nth_number(2020, vec![2, 1, 3]);
        assert_eq!(result, 10);
    }

    #[test]
    fn expect_2020th_spoken_to_be_27() {
        let result: i32 = get_nth_number(2020, vec![1, 2, 3]);
        assert_eq!(result, 27);
    }

    #[test]
    fn expect_2020th_spoken_to_be_78() {
        let result: i32 = get_nth_number(2020, vec![2, 3, 1]);
        assert_eq!(result, 78);
    }

    #[test]
    fn expect_2020th_spoken_to_be_438() {
        let result: i32 = get_nth_number(2020, vec![3, 2, 1]);
        assert_eq!(result, 438);
    }

    #[test]
    fn expect_2020th_spoken_to_be_1836() {
        let result: i32 = get_nth_number(2020, vec![3, 1, 2]);
        assert_eq!(result, 1836);
    }

    #[test]
    fn expect_30000000th_spoken_to_be_175594() {
        let result: i32 = get_nth_number(30000000, vec![0, 3, 6]);
        assert_eq!(result, 175594);
    }

    #[test]
    fn expect_30000000th_spoken_to_be_2578() {
        let result: i32 = get_nth_number(30000000, vec![1, 3, 2]);
        assert_eq!(result, 2578);
    }

    #[test]
    fn expect_30000000th_spoken_to_be_3544142() {
        let result: i32 = get_nth_number(30000000, vec![2, 1, 3]);
        assert_eq!(result, 3544142);
    }

    #[test]
    fn expect_30000000th_spoken_to_be_261214() {
        let result: i32 = get_nth_number(30000000, vec![1, 2, 3]);
        assert_eq!(result, 261214);
    }

    #[test]
    fn expect_30000000th_spoken_to_be_6895259() {
        let result: i32 = get_nth_number(30000000, vec![2, 3, 1]);
        assert_eq!(result, 6895259);
    }

    #[test]
    fn expect_30000000th_spoken_to_be_18() {
        let result: i32 = get_nth_number(30000000, vec![3, 2, 1]);
        assert_eq!(result, 18);
    }

    #[test]
    fn expect_30000000th_spoken_to_be_362() {
        let result: i32 = get_nth_number(30000000, vec![3, 1, 2]);
        assert_eq!(result, 362);
    }
}
