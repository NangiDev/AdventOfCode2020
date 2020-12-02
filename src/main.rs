use adventofcode_2020::day2;
use std::fs;

fn main() {
    let path = "./src/input_files/day2.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let values = contents.split("\n").collect();

    println!(
        "{}",
        day2::count_valid_passwords_according_to_official_toboggan_policy(values)
    );
}
