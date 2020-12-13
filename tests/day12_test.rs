#[cfg(test)]
mod day_12 {
    use adventofcode_2020::day12::{
        move_ferry_with_instructions, move_ferry_with_waypoint_navigation, Ferry, Waypoint,
    };

    #[test]
    fn move_ferry_forward_10_steps_east() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.move_forward(10);
        assert_eq!(ferry.x, 10);
        assert_eq!(ferry.y, 0);
    }
    #[test]
    fn move_ferry_forward_10_steps_west() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_right(180);
        ferry.move_forward(10);
        assert_eq!(ferry.x, -10);
        assert_eq!(ferry.y, 0);
    }
    #[test]
    fn move_ferry_forward_10_steps_north() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_right(270);
        ferry.move_forward(10);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, -10);
    }
    #[test]
    fn move_ferry_forward_10_steps_south() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_right(90);
        ferry.move_forward(10);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, 10);
    }

    #[test]
    fn move_ferry_north_3_steps() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.move_north(3);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, -3);
    }

    #[test]
    fn move_ferry_south_3_steps() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.move_south(3);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, 3);
    }

    #[test]
    fn move_ferry_east_3_steps() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.move_east(3);
        assert_eq!(ferry.x, 3);
        assert_eq!(ferry.y, 0);
    }

    #[test]
    fn move_ferry_west_3_steps() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.move_west(3);
        assert_eq!(ferry.x, -3);
        assert_eq!(ferry.y, 0);
    }

    #[test]
    fn rotate_90_degrees_right_and_move_forward() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_right(90);
        ferry.move_forward(1);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, 1);
    }

    #[test]
    fn rotate_90_degrees_left_and_move_forward() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_left(90);
        ferry.move_forward(1);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, -1);
    }

    #[test]
    fn rotate_180_degrees_right_and_move_forward() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_right(180);
        ferry.move_forward(1);
        assert_eq!(ferry.x, -1);
        assert_eq!(ferry.y, 0);
    }

    #[test]
    fn rotate_180_degrees_left_and_move_forward() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_left(180);
        ferry.move_forward(1);
        assert_eq!(ferry.x, -1);
        assert_eq!(ferry.y, 0);
    }

    #[test]
    fn rotate_270_degrees_right_and_move_forward() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_right(270);
        ferry.move_forward(1);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, -1);
    }

    #[test]
    fn rotate_270_degrees_left_and_move_forward() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        ferry.rotate_left(270);
        ferry.move_forward(1);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, 1);
    }

    #[test]
    fn calculate_manhattan_dist_to_25() {
        let ferry = move_ferry_with_instructions(vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ]);
        assert_eq!(ferry.x, 17);
        assert_eq!(ferry.y, 8);
    }

    #[test]
    fn move_waypoint_north_3_steps() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.move_north(3);
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, -4);
    }

    #[test]
    fn move_waypoint_south_3_steps() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.move_south(3);
        assert_eq!(waypoint.x, 10);
        assert_eq!(waypoint.y, 2);
    }

    #[test]
    fn move_waypoint_east_3_steps() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.move_east(3);
        assert_eq!(waypoint.x, 13);
        assert_eq!(waypoint.y, -1);
    }

    #[test]
    fn move_waypoint_west_3_steps() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.move_west(3);
        assert_eq!(waypoint.x, 7);
        assert_eq!(waypoint.y, -1);
    }

    #[test]
    fn rotate_waypoint_right_90_degrees() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.rotate_right(90);
        assert_eq!(waypoint.x, 1);
        assert_eq!(waypoint.y, 10);
    }

    #[test]
    fn rotate_waypoint_right_180_degrees() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.rotate_right(180);
        assert_eq!(waypoint.x, -10);
        assert_eq!(waypoint.y, 1);
    }

    #[test]
    fn rotate_waypoint_right_270_degrees() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.rotate_right(270);
        assert_eq!(waypoint.x, -1);
        assert_eq!(waypoint.y, -10);
    }

    #[test]
    fn rotate_waypoint_left_90_degrees() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.rotate_left(90);
        assert_eq!(waypoint.x, -1);
        assert_eq!(waypoint.y, -10);
    }

    #[test]
    fn rotate_waypoint_left_180_degrees() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.rotate_left(180);
        assert_eq!(waypoint.x, -10);
        assert_eq!(waypoint.y, 1);
    }

    #[test]
    fn rotate_waypoint_left_270_degrees() {
        let mut waypoint: Waypoint = Waypoint::new();
        waypoint.rotate_left(270);
        assert_eq!(waypoint.x, 1);
        assert_eq!(waypoint.y, 10);
    }

    #[test]
    fn move_waypoint_when_ferry_moving() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, 0);
        assert_eq!(ferry.waypoint.x, 10);
        assert_eq!(ferry.waypoint.y, -1);

        ferry.move_to_waypoint(10);
        assert_eq!(ferry.x, 100);
        assert_eq!(ferry.y, -10);
        assert_eq!(ferry.waypoint.x, 10);
        assert_eq!(ferry.waypoint.y, -1);
    }

    #[test]
    fn move_waypoint_when_ferry_moving_f10_n3_f7() {
        let mut ferry: Ferry = Ferry::new(0, 0);
        assert_eq!(ferry.x, 0);
        assert_eq!(ferry.y, 0);
        assert_eq!(ferry.waypoint.x, 10);
        assert_eq!(ferry.waypoint.y, -1);

        ferry.move_to_waypoint(10);
        assert_eq!(ferry.x, 100);
        assert_eq!(ferry.y, -10);
        assert_eq!(ferry.waypoint.x, 10);
        assert_eq!(ferry.waypoint.y, -1);

        ferry.waypoint.move_north(3);
        assert_eq!(ferry.x, 100);
        assert_eq!(ferry.y, -10);
        assert_eq!(ferry.waypoint.x, 10);
        assert_eq!(ferry.waypoint.y, -4);

        ferry.move_to_waypoint(7);
        assert_eq!(ferry.x, 170);
        assert_eq!(ferry.y, -38);
        assert_eq!(ferry.waypoint.x, 10);
        assert_eq!(ferry.waypoint.y, -4);
    }

    #[test]
    fn calculate_manhattan_dist_with_waypoint_navigation_to_286() {
        let ferry = move_ferry_with_waypoint_navigation(vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ]);
        assert_eq!(ferry.x, 214);
        assert_eq!(ferry.y, 72);
    }
}
