use std::fs;

pub struct Step {
    pub x: i32,
    pub y: i32,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub grid: Vec<String>,
}

pub fn count_trees_in_path(step: Step, map: &Map) -> i32 {
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
