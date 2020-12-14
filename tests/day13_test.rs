#[cfg(test)]
mod day_13 {
    use adventofcode_2020::day13::{find_next_departing_bus, get_next_depart_of_bus};

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
}
