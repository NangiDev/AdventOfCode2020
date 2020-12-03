use std::fs;

struct Step {
    x: i32,
    y: i32,
}

struct Map {
    width: i32,
    height: i32,
    grid: Vec<String>,
}

fn count_trees_in_path(step: Step, map: &Map) -> i32 {
    let mut tree_count = 0;

    let mut position = Step {
        x: step.x,
        y: step.y,
    };

    while position.y < map.height {
        if position.x >= map.width {
            position.x = position.x - map.width;
        }

        let index_x: usize = position.x as usize;
        let index_y: usize = position.y as usize;
        if map.grid[index_y].chars().collect::<Vec<char>>()[index_x] == '#' {
            tree_count += 1;
        }

        position.x += step.x;
        position.y += step.y;
    }

    tree_count
}

fn read_input() -> Map {
    let path = "./src/input_files/day3.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let grid: Vec<String> = contents.split_whitespace().map(|s| s.to_string()).collect();

    Map {
        width: grid[0].len() as i32,
        height: grid.len() as i32,
        grid: grid,
    }
}

pub fn _1() -> i32 {
    let step = Step { x: 3, y: 1 };
    count_trees_in_path(step, &read_input())
}

pub fn _2() -> i32 {
    let map = read_input();
    let step = Step { x: 1, y: 1 };
    let mut answer = count_trees_in_path(step, &map);

    let step = Step { x: 3, y: 1 };
    answer *= count_trees_in_path(step, &map);

    let step = Step { x: 5, y: 1 };
    answer *= count_trees_in_path(step, &map);

    let step = Step { x: 7, y: 1 };
    answer *= count_trees_in_path(step, &map);

    let step = Step { x: 1, y: 2 };
    answer *= count_trees_in_path(step, &map);

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

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
