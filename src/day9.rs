use std::fs;

fn read_input() -> Vec<String> {
    let path = "./src/input_files/day9.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let batches: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    batches
}

pub fn is_valid_number(seq: &Vec<i64>, number: i64) -> bool {
    for n1 in seq {
        for n2 in seq {
            if n1 != n2 && n1 + n2 == number {
                return true;
            }
        }
    }

    false
}

pub fn find_first_invalid(mut preamble: Vec<i64>, numbers: Vec<i64>) -> i64 {
    let mut offset: usize = 0;
    for n in numbers {
        if !is_valid_number(&preamble, n) {
            return n;
        }
        preamble[offset] = n;
        offset += 1;
        if offset >= preamble.len() {
            offset = 0
        };
    }

    0
}

pub fn find_set(input: Vec<i64>, number: i64) -> i64 {
    for i in 0..input.len() {
        let mut largest = 0;
        let mut sum = input[i];
        for y in (i + 1)..input.len() {
            sum += input[y];

            if input[y] > largest {
                largest = input[y];
            }

            if sum == number {
                return input[i] + largest;
            }
        }
    }

    -1
}

pub fn _1() -> i64 {
    let input = read_input();
    let input: Vec<i64> = input
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    find_first_invalid(input[..25].to_vec(), input[25..].to_vec())
}

pub fn _2() -> i64 {
    let input = read_input();
    let input: Vec<i64> = input
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    find_set(input, 1212510616)
}
