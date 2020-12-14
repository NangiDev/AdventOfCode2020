use crate::{input, utils::Input};

pub fn get_next_depart_of_bus(time_stamp: i32, id: i32) -> i32 {
    let mut beginning = 0;
    while beginning <= time_stamp {
        beginning += id;
    }

    beginning
}

pub fn find_next_departing_bus(time_stamp: i32, busses: Vec<&str>) -> (i32, i32) {
    let mut next_bus = (0, time_stamp + 10000);
    for b in busses {
        if let Ok(id) = b.parse::<i32>() {
            let next_dep = get_next_depart_of_bus(time_stamp, id);
            if next_bus.1 >= next_dep {
                next_bus.0 = id;
                next_bus.1 = next_dep;
            }
        }
    }
    next_bus
}

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day13.txt");
    let input = input.as_string();
    let time_stamp = input[0].parse::<i32>().unwrap();
    let busses = input[1].split(",").collect::<Vec<&str>>();

    let bus = find_next_departing_bus(time_stamp, busses);

    bus.0 * (bus.1 - time_stamp)
}
pub fn _2() -> i32 {
    let input = input!("./src/input_files/day13.txt");
    input.as_string();
    0
}
