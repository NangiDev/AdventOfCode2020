#[cfg(test)]
mod day_11 {
    use adventofcode_2020::day11::find_sourrounding_seats;
    use adventofcode_2020::day11::has_five_or_more;
    use adventofcode_2020::day11::has_no_sourrounding_adjacent;
    use adventofcode_2020::day11::play_n_round_part_2;
    use adventofcode_2020::day11::{count_seats, has_four_or_more, has_no_adjacent, play_n_round};
    use adventofcode_2020::day11::{find_seat_in_dir, play_one_round_part_2};
    use std::vec;

    #[test]
    fn play_one_round_and_match_next_state() {
        let state: Vec<Vec<char>> = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
        ];

        let expected: Vec<Vec<char>> = vec![
            "#.##.##.##".chars().collect(),
            "#######.##".chars().collect(),
            "#.#.#..#..".chars().collect(),
            "####.##.##".chars().collect(),
            "#.##.##.##".chars().collect(),
            "#.#####.##".chars().collect(),
            "..#.#.....".chars().collect(),
            "##########".chars().collect(),
            "#.######.#".chars().collect(),
            "#.#####.##".chars().collect(),
        ];
        assert_eq!(play_n_round(1, state), expected);
    }

    #[test]
    fn play_one_round_and_match_next_state_1() {
        let state: Vec<Vec<char>> = vec![
            "#.##.##.##".chars().collect(),
            "#######.##".chars().collect(),
            "#.#.#..#..".chars().collect(),
            "####.##.##".chars().collect(),
            "#.##.##.##".chars().collect(),
            "#.#####.##".chars().collect(),
            "..#.#.....".chars().collect(),
            "##########".chars().collect(),
            "#.######.#".chars().collect(),
            "#.#####.##".chars().collect(),
        ];

        let expected: Vec<Vec<char>> = vec![
            "#.LL.L#.##".chars().collect(),
            "#LLLLLL.L#".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "#LLL.LL.L#".chars().collect(),
            "#.LL.LL.LL".chars().collect(),
            "#.LLLL#.##".chars().collect(),
            "..L.L.....".chars().collect(),
            "#LLLLLLLL#".chars().collect(),
            "#.LLLLLL.L".chars().collect(),
            "#.#LLLL.##".chars().collect(),
        ];
        assert_eq!(play_n_round(1, state), expected);
    }

    #[test]
    fn play_two_round_and_match_next_state() {
        let state: Vec<Vec<char>> = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
        ];

        let expected: Vec<Vec<char>> = vec![
            "#.LL.L#.##".chars().collect(),
            "#LLLLLL.L#".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "#LLL.LL.L#".chars().collect(),
            "#.LL.LL.LL".chars().collect(),
            "#.LLLL#.##".chars().collect(),
            "..L.L.....".chars().collect(),
            "#LLLLLLLL#".chars().collect(),
            "#.LLLLLL.L".chars().collect(),
            "#.#LLLL.##".chars().collect(),
        ];
        assert_eq!(play_n_round(2, state), expected);
    }

    #[test]
    fn play_five_round_and_match_next_state() {
        let state: Vec<Vec<char>> = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
        ];

        let expected: Vec<Vec<char>> = vec![
            "#.#L.L#.##".chars().collect(),
            "#LLL#LL.L#".chars().collect(),
            "L.#.L..#..".chars().collect(),
            "#L##.##.L#".chars().collect(),
            "#.#L.LL.LL".chars().collect(),
            "#.#L#L#.##".chars().collect(),
            "..L.L.....".chars().collect(),
            "#L#L##L#L#".chars().collect(),
            "#.LLLLLL.L".chars().collect(),
            "#.#L#L#.##".chars().collect(),
        ];
        assert_eq!(play_n_round(5, state), expected);
    }

    #[test]
    fn play_one_round_and_match_next_state_part_2() {
        let state: Vec<Vec<char>> = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
        ];

        let expected: Vec<Vec<char>> = vec![
            "#.##.##.##".chars().collect(),
            "#######.##".chars().collect(),
            "#.#.#..#..".chars().collect(),
            "####.##.##".chars().collect(),
            "#.##.##.##".chars().collect(),
            "#.#####.##".chars().collect(),
            "..#.#.....".chars().collect(),
            "##########".chars().collect(),
            "#.######.#".chars().collect(),
            "#.#####.##".chars().collect(),
        ];
        assert_eq!(play_n_round_part_2(1, state), expected);
    }

    #[test]
    fn play_one_round_and_match_next_state_1_part_2() {
        let mut state: Vec<Vec<char>> = vec![
            "#.##.##.##".chars().collect(),
            "#######.##".chars().collect(),
            "#.#.#..#..".chars().collect(),
            "####.##.##".chars().collect(),
            "#.##.##.##".chars().collect(),
            "#.#####.##".chars().collect(),
            "..#.#.....".chars().collect(),
            "##########".chars().collect(),
            "#.######.#".chars().collect(),
            "#.#####.##".chars().collect(),
        ];

        let expected: Vec<Vec<char>> = vec![
            "#.LL.LL.L#".chars().collect(),
            "#LLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLL#".chars().collect(),
            "#.LLLLLL.L".chars().collect(),
            "#.LLLLL.L#".chars().collect(),
        ];
        assert_eq!(play_one_round_part_2(&mut state), expected);
    }

    #[test]
    fn play_6_round_and_match_next_state_part_2() {
        let state: Vec<Vec<char>> = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
        ];

        let expected: Vec<Vec<char>> = vec![
            "#.L#.L#.L#".chars().collect(),
            "#LLLLLL.LL".chars().collect(),
            "L.L.L..#..".chars().collect(),
            "##L#.#L.L#".chars().collect(),
            "L.L#.LL.L#".chars().collect(),
            "#.LLLL#.LL".chars().collect(),
            "..#.L.....".chars().collect(),
            "LLL###LLL#".chars().collect(),
            "#.LLLLL#.L".chars().collect(),
            "#.L#LL#.L#".chars().collect(),
        ];
        assert_eq!(play_n_round_part_2(6, state), expected);
    }

    #[test]
    fn has_no_sourrounding_adjacent_seats() {
        let state: Vec<Vec<char>> = vec![
            "L.L".chars().collect(),
            "LLL".chars().collect(),
            "L.L".chars().collect(),
        ];
        assert_eq!(has_no_sourrounding_adjacent(1, 1, &state), true);
    }

    #[test]
    fn has_sourrounding_adjacent_seats() {
        let state: Vec<Vec<char>> = vec![
            "L.L".chars().collect(),
            "LL#".chars().collect(),
            "#.L".chars().collect(),
        ];
        assert_eq!(has_no_sourrounding_adjacent(1, 1, &state), false);
    }

    #[test]
    fn occupy_empty_chair_with_no_adjacent() {
        let state: Vec<Vec<char>> = vec![
            "L.L".chars().collect(),
            "LLL".chars().collect(),
            "L.L".chars().collect(),
        ];
        assert_eq!(has_no_adjacent(1, 1, 3, 3, &state), true);
    }

    #[test]
    fn dont_occupy_empty_chair_with_adjacent() {
        let state: Vec<Vec<char>> = vec![
            "L.L".chars().collect(),
            "LL#".chars().collect(),
            "L.L".chars().collect(),
        ];
        assert_eq!(has_no_adjacent(1, 1, 3, 3, &state), false);
    }

    #[test]
    fn empty_chair_when_4_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "L.#".chars().collect(),
            "##L".chars().collect(),
            "##L".chars().collect(),
        ];
        assert_eq!(has_four_or_more(1, 1, 3, 3, &state), true);
    }

    #[test]
    fn empty_chair_when_6_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "#.#".chars().collect(),
            "###".chars().collect(),
            "##L".chars().collect(),
        ];
        assert_eq!(has_four_or_more(1, 1, 3, 3, &state), true);
    }

    #[test]
    fn empty_chair_when_seat_at_top_edge_4_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec!["###".chars().collect(), "##L".chars().collect()];
        assert_eq!(
            has_four_or_more(1, 0, state[0].len() as i32, state.len() as i32, &state),
            true
        );
    }

    #[test]
    fn empty_chair_when_seat_at_bottom_edge_4_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec!["###".chars().collect(), "##L".chars().collect()];
        assert_eq!(
            has_four_or_more(1, 1, state[0].len() as i32, state.len() as i32, &state),
            true
        );
    }

    #[test]
    fn empty_chair_when_seat_at_left_edge_4_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "##".chars().collect(),
            "##".chars().collect(),
            "#L".chars().collect(),
        ];
        assert_eq!(
            has_four_or_more(0, 1, state[0].len() as i32, state.len() as i32, &state),
            true
        );
    }

    #[test]
    fn empty_chair_when_seat_at_right_edge_4_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "##".chars().collect(),
            "##".chars().collect(),
            "#L".chars().collect(),
        ];
        assert_eq!(
            has_four_or_more(1, 1, state[0].len() as i32, state.len() as i32, &state),
            true
        );
    }

    #[test]
    fn dont_empty_chair_when_seat_at_edge_4_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "#.".chars().collect(),
            "##".chars().collect(),
            ".#".chars().collect(),
        ];
        assert_eq!(has_four_or_more(1, 0, 2, 3, &state), false);
    }

    #[test]
    fn dont_empty_chair_when_4_or_more_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "L.#".chars().collect(),
            "##L".chars().collect(),
            ".#L".chars().collect(),
        ];
        assert_eq!(has_four_or_more(1, 1, 3, 3, &state), false);
    }

    #[test]
    fn count_seats_37() {
        let state: Vec<Vec<char>> = vec![
            "#.#L.L#.##".chars().collect(),
            "#LLL#LL.L#".chars().collect(),
            "L.#.L..#..".chars().collect(),
            "#L##.##.L#".chars().collect(),
            "#.#L.LL.LL".chars().collect(),
            "#.#L#L#.##".chars().collect(),
            "..L.L.....".chars().collect(),
            "#L#L##L#L#".chars().collect(),
            "#.LLLLLL.L".chars().collect(),
            "#.#L#L#.##".chars().collect(),
        ];
        assert_eq!(count_seats(&state), 37);
    }

    #[test]
    fn count_seats_26() {
        let state: Vec<Vec<char>> = vec![
            "#.L#.L#.L#".chars().collect(),
            "#LLLLLL.LL".chars().collect(),
            "L.L.L..#..".chars().collect(),
            "##L#.#L.L#".chars().collect(),
            "L.L#.LL.L#".chars().collect(),
            "#.LLLL#.LL".chars().collect(),
            "..#.L.....".chars().collect(),
            "LLL###LLL#".chars().collect(),
            "#.LLLLL#.L".chars().collect(),
            "#.L#LL#.L#".chars().collect(),
        ];
        assert_eq!(count_seats(&state), 26);
    }

    #[test]
    fn empty_chair_when_5_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "#.#".chars().collect(),
            "##L".chars().collect(),
            "##L".chars().collect(),
        ];
        assert_eq!(has_five_or_more(1, 1, &state), true);
    }

    #[test]
    fn empty_chair_when_seat_at_top_edge_5_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec!["###".chars().collect(), "###".chars().collect()];
        assert_eq!(has_five_or_more(1, 0, &state), true);
    }

    #[test]
    fn empty_chair_when_seat_at_bottom_edge_5_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec!["###".chars().collect(), "###".chars().collect()];
        assert_eq!(has_five_or_more(1, 1, &state), true);
    }

    #[test]
    fn empty_chair_when_seat_at_left_edge_5_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "##".chars().collect(),
            "##".chars().collect(),
            "##".chars().collect(),
        ];
        assert_eq!(has_five_or_more(0, 1, &state), true);
    }

    #[test]
    fn empty_chair_when_seat_at_right_edge_5_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "##".chars().collect(),
            "##".chars().collect(),
            "##".chars().collect(),
        ];
        assert_eq!(has_five_or_more(1, 1, &state), true);
    }

    #[test]
    fn dont_empty_chair_when_seat_at_edge_5_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "#.".chars().collect(),
            "##".chars().collect(),
            ".#".chars().collect(),
        ];
        assert_eq!(has_five_or_more(1, 0, &state), false);
    }

    #[test]
    fn dont_empty_chair_when_5_or_more_adjacent_as_occupied() {
        let state: Vec<Vec<char>> = vec![
            "L.#".chars().collect(),
            "##L".chars().collect(),
            ".#L".chars().collect(),
        ];
        assert_eq!(has_five_or_more(1, 1, &state), false);
    }

    #[test]
    fn find_occupied_seat() {
        let state: Vec<Vec<char>> = vec![
            ".......#.".chars().collect(),
            "...#.....".chars().collect(),
            ".#.......".chars().collect(),
            ".........".chars().collect(),
            "..#L....#".chars().collect(),
            "....#....".chars().collect(),
            ".........".chars().collect(),
            "#........".chars().collect(),
            "...#.....".chars().collect(),
        ];
        assert_eq!(find_seat_in_dir(3, 4, -1, 0, &state), '#');
    }

    #[test]
    fn find_empty_seat() {
        let state: Vec<Vec<char>> = vec![
            ".......#.".chars().collect(),
            "...#.....".chars().collect(),
            ".#.......".chars().collect(),
            ".........".chars().collect(),
            "..#L....L".chars().collect(),
            "....#....".chars().collect(),
            ".........".chars().collect(),
            "#........".chars().collect(),
            "...#.....".chars().collect(),
        ];
        assert_eq!(find_seat_in_dir(3, 4, 1, 0, &state), 'L');
    }

    #[test]
    fn find_eight_occupied_seat() {
        let state: Vec<Vec<char>> = vec![
            ".......#.".chars().collect(),
            "...#.....".chars().collect(),
            ".#.......".chars().collect(),
            ".........".chars().collect(),
            "..#L....#".chars().collect(),
            "....#....".chars().collect(),
            ".........".chars().collect(),
            "#........".chars().collect(),
            "...#.....".chars().collect(),
        ];
        assert_eq!(find_sourrounding_seats(3, 4, &state), 8);
    }

    #[test]
    fn find_zero_occupied_seat() {
        let state: Vec<Vec<char>> = vec![
            ".##.##.".chars().collect(),
            "#.#.#.#".chars().collect(),
            "##...##".chars().collect(),
            "...L...".chars().collect(),
            "##...##".chars().collect(),
            "#.#.#.#".chars().collect(),
            ".##.##.".chars().collect(),
        ];
        assert_eq!(find_sourrounding_seats(3, 3, &state), 0);
    }

    #[test]
    fn empty_chair_when_5_adjacent_as_occupied_part_2() {
        let state: Vec<Vec<char>> = vec![
            "#.#".chars().collect(),
            "##L".chars().collect(),
            "##L".chars().collect(),
        ];
        assert_eq!(find_sourrounding_seats(1, 1, &state), 5);
    }

    #[test]
    fn empty_chair_when_seat_at_top_edge_5_adjacent_as_occupied_part_2() {
        let state: Vec<Vec<char>> = vec!["###".chars().collect(), "###".chars().collect()];
        assert_eq!(find_sourrounding_seats(1, 0, &state), 5);
    }

    #[test]
    fn empty_chair_when_seat_at_bottom_edge_5_adjacent_as_occupied_part_2() {
        let state: Vec<Vec<char>> = vec!["###".chars().collect(), "###".chars().collect()];
        assert_eq!(find_sourrounding_seats(1, 1, &state), 5);
    }

    #[test]
    fn empty_chair_when_seat_at_left_edge_5_adjacent_as_occupied_part_2() {
        let state: Vec<Vec<char>> = vec![
            "##".chars().collect(),
            "##".chars().collect(),
            "##".chars().collect(),
        ];
        assert_eq!(find_sourrounding_seats(0, 1, &state), 5);
    }

    #[test]
    fn empty_chair_when_seat_at_right_edge_5_adjacent_as_occupied_part_2() {
        let state: Vec<Vec<char>> = vec![
            "##".chars().collect(),
            "##".chars().collect(),
            "##".chars().collect(),
        ];
        assert_eq!(find_sourrounding_seats(1, 1, &state), 5);
    }

    #[test]
    fn dont_empty_chair_when_seat_at_edge_5_adjacent_as_occupied_part_2() {
        let state: Vec<Vec<char>> = vec![
            "#.".chars().collect(),
            "##".chars().collect(),
            ".#".chars().collect(),
        ];
        assert_eq!(has_five_or_more(1, 0, &state), false);
    }

    #[test]
    fn dont_empty_chair_when_5_or_more_adjacent_as_occupied_part_2() {
        let state: Vec<Vec<char>> = vec![
            "L.#".chars().collect(),
            "##L".chars().collect(),
            ".#L".chars().collect(),
        ];
        assert_eq!(has_five_or_more(1, 1, &state), false);
    }
}
