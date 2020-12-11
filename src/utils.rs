use std::fs::read_to_string;

#[macro_export]
macro_rules! input {
    ( $x:expr ) => {{
        Input { path: $x }
    }};
}

#[derive(Debug)]
pub struct Input {
    pub path: &'static str,
}

impl Input {
    pub fn as_string(self: &Self) -> Vec<String> {
        let content = read_to_string(self.path).expect("Something went wrong reading the file");
        content.split("\n").map(From::from).collect()
    }

    pub fn as_int(self: &Self) -> Vec<i32> {
        let content = read_to_string(self.path).expect("Something went wrong reading the file");
        content
            .split("\n")
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}
