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

pub fn find_next_departing_bus_i64(time_stamp: i32, busses: Vec<(i32, i32)>) -> (i32, i32) {
    let mut next_bus = (0, time_stamp + 10000);
    for b in busses {
        let next_dep = get_next_depart_of_bus(time_stamp, b.0);
        if next_bus.1 >= next_dep {
            next_bus.0 = b.0;
            next_bus.1 = next_dep;
        }
    }
    next_bus
}

// Stole this solution from https://github.com/maxdavidson/advent-of-code-2020/blob/main/src/day13/mod.rs
// The stride *= id was the big different from my solution. It has to do with the common denominator to optimize the solution
pub fn find_earliest_sequece_of(mut timestamp: i64, busses: Vec<&str>) -> i64 {
    let mut stride = 1;

    for (maybe_bus_id, offset) in busses.iter().copied().zip(0i64..) {
        if let Ok(bus_id) = maybe_bus_id.parse::<i64>() {
            while (timestamp + offset) % bus_id != 0 {
                timestamp += stride
            }
            stride *= bus_id
        }
    }

    timestamp
}

// Stole this solution from https://github.com/maxdavidson/advent-of-code-2020/blob/main/src/day13/mod.rs
// The stride *= id was the big different from my solution. It has to do with the common denominator to optimize the solution
pub fn find_earliest_sequece_of_i64(mut timestamp: i64, busses: Vec<(i64, i64)>) -> i64 {
    let mut stride = 1;

    for (id, offset) in busses {
        while (timestamp + offset) % id != 0 {
            timestamp += stride
        }
        stride *= id
    }

    timestamp
}

// pub fn find_earliest_sequece_of(mut timestamp: i64, busses: Vec<&str>) -> i64 {
//     let first_id = busses.first().unwrap().parse::<i64>().unwrap();
//     let last_id = busses.last().unwrap().parse::<i64>().unwrap();
//     let id_len = busses.len() as i64;

//     while timestamp + first_id < i64::MAX {
//         timestamp += first_id;

//         if !timestamp % first_id == 0 && (timestamp + id_len - 1) % last_id == 0 {
//             continue;
//         }

//         let mut stride = 1;
//         let mut valid = true;

//         while stride < id_len && valid {
//             if let Ok(id) = busses.get(stride as usize).unwrap().parse::<i64>() {
//                 if (timestamp + stride) % id != 0 {
//                     valid = false;
//                 }
//             }

//             stride += 1;
//         }

//         if valid {
//             return timestamp;
//         }
//     }

//     0
// }

// pub fn find_earliest_sequece_of_i64(mut timestamp: i64, busses: Vec<(i64, i64)>) -> i64 {
//     let first_id = busses.first().unwrap().0;
//     let last_id = busses.last().unwrap().0;
//     let id_list_len = busses.last().unwrap().1;

//     // while timestamp <= 1202161486 {
//     while timestamp + first_id < i64::MAX {
//         timestamp += first_id;

//         if timestamp % first_id != 0 && (timestamp + id_list_len - 1) % last_id != 0 {
//             continue;
//         }

//         // if timestamp % 1000000 == 0 {
//         //     println!("{}", timestamp);
//         // }

//         let mut stride = 1;
//         let mut index = 1;
//         let mut valid = true;

//         while stride <= id_list_len as i64 && valid {
//             if let Some(id) = busses.get(index) {
//                 if stride != id.1 {
//                     stride += 1;
//                     continue;
//                 }

//                 if (timestamp + stride) % id.0 != 0 {
//                     valid = false;
//                 }
//             }

//             index += 1;
//             stride += 1;
//         }

//         if valid {
//             return timestamp;
//         }
//     }

//     0
// }

pub fn _1() -> i32 {
    let input = input!("./src/input_files/day13.txt");
    let input = input.as_string();
    let time_stamp = input[0].parse::<i32>().unwrap();
    let busses = input[1].split(',').collect::<Vec<&str>>();

    let bus = find_next_departing_bus(time_stamp, busses);

    bus.0 * (bus.1 - time_stamp)
}
pub fn _2() -> i64 {
    let input = input!("./src/input_files/day13.txt").as_string();

    let busses = input[1].split(',').collect::<Vec<&str>>();

    // Had an idea optimising with tupels. but I dont think it did any differents

    // let mut tuples: Vec<(i64, i64)> = vec![];
    // for (idx, bus) in busses.iter().enumerate() {
    //     if let Ok(b) = bus.parse::<i64>() {
    //         tuples.push((b, idx as i64));
    //     }
    // }

    find_earliest_sequece_of(100000000000000, busses)
}
