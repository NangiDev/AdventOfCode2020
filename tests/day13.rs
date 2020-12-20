#[cfg(test)]
mod day_13 {
    use adventofcode_2020::day13::find_earliest_sequece_of_i64;
    use adventofcode_2020::day13::find_next_departing_bus_i64;
    use adventofcode_2020::day13::{
        find_earliest_sequece_of, find_next_departing_bus, get_next_depart_of_bus,
    };

    #[test]
    fn next_depart_of_id_7_is_945() {
        let result = get_next_depart_of_bus(939, 7);
        assert_eq!(result, 945);
    }

    #[test]
    fn next_depart_of_id_13_is_949() {
        let result = get_next_depart_of_bus(939, 13);
        assert_eq!(result, 949);
    }

    #[test]
    fn next_depart_of_id_59_is_944() {
        let result = get_next_depart_of_bus(939, 59);
        assert_eq!(result, 944);
    }

    #[test]
    fn find_59_as_next_departing_bus() {
        let result = find_next_departing_bus(939, vec!["7", "13", "x", "x", "59", "x", "31", "19"]);
        assert_eq!(result, (59, 944));
    }

    #[test]
    fn find_59_as_next_departing_bus_i64() {
        let result =
            find_next_departing_bus_i64(939, vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)]);
        assert_eq!(result, (59, 944));
    }

    #[test]
    fn find_1068781_as_earliest_timestamp_with_list_sequence() {
        let busses: Vec<&str> = vec!["7", "13", "x", "x", "59", "x", "31", "19"];

        let result = find_earliest_sequece_of(0, busses);

        assert_eq!(result, 1068781);
    }

    #[test]
    fn find_3417_as_earliest_timestamp_with_list_sequence() {
        let busses: Vec<&str> = vec!["17", "x", "13", "19"];

        let result = find_earliest_sequece_of(0, busses);

        assert_eq!(result, 3417);
    }

    #[test]
    fn find_754018_as_earliest_timestamp_with_list_sequence() {
        let busses: Vec<&str> = vec!["67", "7", "59", "61"];

        let result = find_earliest_sequece_of(0, busses);

        assert_eq!(result, 754018);
    }

    #[test]
    fn find_779210_as_earliest_timestamp_with_list_sequence() {
        let busses: Vec<&str> = vec!["67", "x", "7", "59", "61"];

        let result = find_earliest_sequece_of(0, busses);

        assert_eq!(result, 779210);
    }

    #[test]
    fn find_1261476_as_earliest_timestamp_with_list_sequence() {
        let busses: Vec<&str> = vec!["67", "7", "x", "59", "61"];

        let result = find_earliest_sequece_of(0, busses);

        assert_eq!(result, 1261476);
    }

    #[test]
    fn find_1202161486_as_earliest_timestamp_with_list_sequence() {
        let busses: Vec<&str> = vec!["1789", "37", "47", "1889"];

        let result = find_earliest_sequece_of(0, busses);

        assert_eq!(result, 1202161486);
    }

    // #[test]
    // fn find_1068781_as_earliest_timestamp_with_list_sequence_i64() {
    //     let busses: Vec<(i64, i64)> = vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)];

    //     let result = find_earliest_sequece_of_i64(0, busses);

    //     assert_eq!(result, 1068781);
    // }

    // #[test]
    // fn find_3417_as_earliest_timestamp_with_list_sequence_i64() {
    //     let busses: Vec<(i64, i64)> = vec![(17, 0), (13, 2), (19, 3)];

    //     let result = find_earliest_sequece_of_i64(0, busses);

    //     assert_eq!(result, 3417);
    // }

    #[test]
    fn find_754018_as_earliest_timestamp_with_list_sequence_i64() {
        let busses: Vec<(i64, i64)> = vec![(67, 0), (7, 1), (59, 2), (61, 3)];

        let result = find_earliest_sequece_of_i64(0, busses);

        assert_eq!(result, 754018);
    }

    // #[test]
    // fn find_779210_as_earliest_timestamp_with_list_sequence_i64() {
    //     let busses: Vec<(i64, i64)> = vec![(67, 0), (7, 2), (59, 3), (61, 4)];

    //     let result = find_earliest_sequece_of_i64(0, busses);

    //     assert_eq!(result, 779210);
    // }

    // #[test]
    // fn find_1261476_as_earliest_timestamp_with_list_sequence_i64() {
    //     let busses: Vec<(i64, i64)> = vec![(67, 0), (7, 1), (59, 3), (61, 4)];

    //     let result = find_earliest_sequece_of_i64(0, busses);

    //     assert_eq!(result, 1261476);
    // }

    // #[test]
    // fn find_1202161486_as_earliest_timestamp_with_list_sequence_i64() {
    //     let busses: Vec<(i64, i64)> = vec![(1789, 0), (37, 1), (47, 2), (1889, 3)];

    //     let result = find_earliest_sequece_of_i64(0, busses);

    //     assert_eq!(result, 1202161486);
    // }
}
