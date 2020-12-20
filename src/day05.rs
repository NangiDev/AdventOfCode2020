use std::fs;

use crate::run_day;

fn read_input() -> Vec<String> {
    let path = "./src/input_files/day5.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let batches: Vec<String> = contents.split_whitespace().map(|s| s.to_string()).collect();
    batches
}

pub fn find_row(seat: &str) -> i32 {
    let mut row_max = 127;
    let mut row_min = 0;
    let mut chars = seat.chars();
    let mut nx = chars.next().unwrap();

    while nx == 'F' || nx == 'B' {
        if nx == 'F' {
            row_max = row_min / 2 + row_max / 2;
        } else {
            row_min += 1 + (row_max - row_min) / 2;
        }

        nx = chars.next().unwrap();
    }
    row_min
}

pub fn find_column(seat: &str) -> i32 {
    let mut col_max = 7;
    let mut col_min = 0;
    for c in seat.chars() {
        if c == 'L' {
            col_max = col_min / 2 + col_max / 2;
        } else if c == 'R' {
            col_min += 1 + (col_max - col_min) / 2;
        }
    }
    col_min
}

pub fn calculate_row_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

pub fn find_highest_seat_id(boarding_passes: Vec<String>) -> i32 {
    let mut highest = 0;
    for seat in &boarding_passes {
        let seat_id = calculate_row_id(find_row(seat), find_column(seat));
        if seat_id > highest {
            highest = seat_id;
        }
    }
    highest
}

pub fn find_missing_seat_in_list(boarding_passes: Vec<i32>) -> i32 {
    let mut my_pass: i32 = 0;

    for i in 0..boarding_passes.len() - 2 {
        let cur = boarding_passes[i];
        let next = boarding_passes[i + 1];

        if next - cur > 1 {
            my_pass = cur + 1;
            break;
        }
    }
    my_pass
}

pub fn _1() -> i32 {
    let boarding_passes = read_input();
    find_highest_seat_id(boarding_passes)
}

pub fn _2() -> i32 {
    let mut boarding_passes: Vec<i32> = read_input()
        .into_iter()
        .map(|s| calculate_row_id(find_row(&s), find_column(&s)))
        .collect();
    boarding_passes.sort_unstable();
    find_missing_seat_in_list(boarding_passes)
}

pub fn print() {
    run_day!(crate::day05);
}
