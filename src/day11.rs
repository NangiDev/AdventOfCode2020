use std::{cmp::Ordering, fs};

use crate::run_day;

pub fn count_seats(state: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for s1 in state {
        for s2 in s1 {
            if *s2 == '#' {
                count += 1;
            }
        }
    }
    count
}

pub fn play_one_round(state: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = state[0].len() as i32;
    let height = state.len() as i32;

    let mut new_state = state.clone();

    for (i, y) in state.clone().iter().enumerate() {
        for (e, _x) in y.iter().enumerate() {
            match state[i][e] {
                'L' => {
                    if has_no_adjacent(e as i32, i as i32, width, height, state) {
                        new_state[i][e] = '#';
                    }
                }
                '#' => {
                    if has_four_or_more(e as i32, i as i32, width, height, state) {
                        new_state[i][e] = 'L';
                    }
                }
                _ => {}
            }
        }
    }

    new_state
}

pub fn play_one_round_part_2(state: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_state = state.clone();

    for (i, y) in state.clone().iter().enumerate() {
        for (e, _x) in y.iter().enumerate() {
            match state[i][e] {
                'L' => {
                    if has_no_sourrounding_adjacent(e as i32, i as i32, state) {
                        new_state[i][e] = '#';
                    }
                }
                '#' => {
                    if has_five_or_more(e as i32, i as i32, state) {
                        new_state[i][e] = 'L';
                    }
                }
                _ => {}
            }
        }
    }

    new_state
}

pub fn has_no_sourrounding_adjacent(x: i32, y: i32, state: &Vec<Vec<char>>) -> bool {
    find_sourrounding_seats(x, y, state) <= 0
}

pub fn play_n_round(n: i32, state: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_state = state;

    for _i in 0..n {
        new_state = play_one_round(&mut new_state.clone());
    }

    new_state
}

pub fn play_n_round_part_2(n: i32, state: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_state = state;

    for _i in 0..n {
        new_state = play_one_round_part_2(&mut new_state.clone());
    }

    new_state
}

pub fn find_seat_in_dir(x: i32, y: i32, dx: i32, dy: i32, state: &Vec<Vec<char>>) -> char {
    if dx == 0 && dy == 0 {
        return 'L';
    }

    let y = y + dy;
    let x = x + dx;

    if y < 0 || y >= state.len() as i32 {
        return 'L';
    }

    if x < 0 || x >= state[0].len() as i32 {
        return 'L';
    }

    match state[y as usize][x as usize] {
        '#' => '#',
        'L' => 'L',
        _ => find_seat_in_dir(x, y, dx, dy, state),
    }
}

pub fn find_sourrounding_seats(x: i32, y: i32, state: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if find_seat_in_dir(x, y, dx, dy, state) == '#' {
                count += 1;
            }
        }
    }
    count
}

pub fn has_no_adjacent(x: i32, y: i32, w: i32, h: i32, state: &Vec<Vec<char>>) -> bool {
    let r = (y + 1).min(h - 1) as usize;
    let l = (y - 1).max(0) as usize;
    let u = (x - 1).max(0) as usize;
    let d = (x + 1).min(w - 1) as usize;

    state[y as usize][x as usize] != '#'
        && state[l][x as usize] != '#'
        && state[r][x as usize] != '#'
        && state[y as usize][d] != '#'
        && state[y as usize][u] != '#'
        && state[r][d] != '#'
        && state[l][u] != '#'
        && state[r][u] != '#'
        && state[l][d] != '#'
}

pub fn has_four_or_more(x: i32, y: i32, w: i32, h: i32, state: &Vec<Vec<char>>) -> bool {
    let r = x + 1;
    let l = x - 1;
    let u = y - 1;
    let d = y + 1;

    let mut count = 0;
    if l >= 0 && state[y as usize][l as usize] == '#' {
        count += 1
    };
    if r < w && state[y as usize][r as usize] == '#' {
        count += 1
    };
    if d < h && state[d as usize][x as usize] == '#' {
        count += 1
    };
    if u >= 0 && state[u as usize][x as usize] == '#' {
        count += 1
    };
    if r < w && d < h && state[d as usize][r as usize] == '#' {
        count += 1
    };
    if l >= 0 && u >= 0 && state[u as usize][l as usize] == '#' {
        count += 1
    };
    if r < w && u >= 0 && state[u as usize][r as usize] == '#' {
        count += 1
    };
    if l >= 0 && d < h && state[d as usize][l as usize] == '#' {
        count += 1
    };

    count >= 4
}

pub fn has_five_or_more(x: i32, y: i32, state: &Vec<Vec<char>>) -> bool {
    find_sourrounding_seats(x as i32, y as i32, state) >= 5
}
fn vec_compare(va: &Vec<Vec<char>>, vb: &Vec<Vec<char>>) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| a.cmp(&b) == Ordering::Equal)
}

fn read_input() -> Vec<Vec<char>> {
    let path = "./src/input_files/day11.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.split('\n').map(|f| f.chars().collect()).collect()
}

pub fn _1() -> i32 {
    let mut state: Vec<Vec<char>> = read_input();

    let mut new_state = play_one_round(&mut state.clone());

    while !vec_compare(&new_state, &state) {
        state = new_state.clone();
        new_state = play_one_round(&mut new_state.clone());
    }
    count_seats(&new_state)
}
pub fn _2() -> i32 {
    let mut state: Vec<Vec<char>> = read_input();

    let mut new_state = play_one_round_part_2(&mut state.clone());

    while !vec_compare(&new_state, &state) {
        state = new_state.clone();
        new_state = play_one_round_part_2(&mut new_state.clone());
    }

    count_seats(&new_state)
}

pub fn print() {
    run_day!(crate::day11);
}
