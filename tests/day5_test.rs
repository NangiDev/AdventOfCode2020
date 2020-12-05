#[cfg(test)]
mod day_5 {
    use adventofcode_2020::day5::{
        calculate_row_id, find_column, find_highest_seat_id, find_missing_seat_in_list, find_row,
    };

    #[test]
    fn find_seat_row_44() {
        assert_eq!(find_row(&"FBFBBFFRLR".to_string()), 44);
    }
    #[test]
    fn find_seat_row_70() {
        assert_eq!(find_row(&"BFFFBBFRRR".to_string()), 70);
    }
    #[test]
    fn find_seat_row_14() {
        assert_eq!(find_row(&"FFFBBBFRRR".to_string()), 14);
    }
    #[test]
    fn find_seat_row_102() {
        assert_eq!(find_row(&"BBFFBBFRLL".to_string()), 102);
    }

    #[test]
    fn find_seat_column_5_row_44() {
        assert_eq!(find_column(&"FBFBBFFRLR".to_string()), 5);
    }
    #[test]
    fn find_seat_column_7_row_70() {
        assert_eq!(find_column(&"BFFFBBFRRR".to_string()), 7);
    }
    #[test]
    fn find_seat_column_5_row_14() {
        assert_eq!(find_column(&"FFFBBBFRRR".to_string()), 7);
    }
    #[test]
    fn find_seat_column_4_row_102() {
        assert_eq!(find_column(&"BBFFBBFRLL".to_string()), 4);
    }

    #[test]
    fn calculate_row_id_row_44_col_5() {
        assert_eq!(calculate_row_id(44, 5), 357);
    }
    #[test]
    fn calculate_row_id_row_70_col_7() {
        assert_eq!(calculate_row_id(70, 7), 567);
    }
    #[test]
    fn calculate_row_id_row_14_col_7() {
        assert_eq!(calculate_row_id(14, 7), 119);
    }
    #[test]
    fn calculate_row_id_row_102_col_4() {
        assert_eq!(calculate_row_id(102, 4), 820);
    }

    #[test]
    fn find_highest_seat_id_567() {
        let boarding_passes: Vec<String> = vec![
            "FBFBBFFRLR".to_string(),
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];
        assert_eq!(find_highest_seat_id(boarding_passes), 820);
    }

    #[test]
    fn find_missing_seat_6_in_list() {
        let boarding_passes: Vec<i32> = vec![3, 4, 5, 7, 8, 9];
        assert_eq!(find_missing_seat_in_list(boarding_passes), 6);
    }
}
