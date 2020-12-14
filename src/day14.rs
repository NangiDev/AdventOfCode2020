use crate::{input, utils::Input};

#[derive(Debug)]
pub struct Parameter {
    pub mask: i64,
    pub mems: Vec<(i64, i64)>,
}

impl Parameter {
    pub fn new(mask: i64) -> Self {
        Self { mask, mems: vec![] }
    }

    pub fn add_mem(&mut self, mem: (i64, i64)) {
        self.mems.push(mem);
    }
}

pub fn convert_params_to_objects(parameters: Vec<String>) -> Vec<Parameter> {
    let mut params: Vec<Parameter> = vec![];
    for p in parameters {
        if p.starts_with("mask") {
            let mask = p.split(' ').collect::<Vec<&str>>()[2]
                .to_string()
                .replace('X', "0");
            params.push(Parameter::new(
                i64::from_str_radix(mask.as_str(), 2).unwrap(),
            ));
        } else if p.starts_with("mem") {
            let start_bytes = p.find('[').unwrap_or(0) + 1;
            let end_bytes = p.find(']').unwrap_or(p.len() - 1);
            let mem: i64 = p[start_bytes..end_bytes].parse::<i64>().unwrap();
            let value: i64 = p.split(' ').collect::<Vec<&str>>()[2]
                .parse::<i64>()
                .unwrap();
            format!("{:#036b}", value);
            params.last_mut().unwrap().add_mem((mem, value));
        }
    }
    params
}

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day14.txt").as_string();
    println!("{:?}", convert_params_to_objects(input));
    0
}
pub fn _2() -> i32 {
    let input = input!("./src/input_files/day14.txt");
    0
}
