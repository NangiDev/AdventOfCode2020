use std::fs;

pub fn exec_1(instructions: Vec<[String; 2]>) -> i32 {
    exec_single_instruction_on_inf_loop(0, &instructions, &mut vec![], 0)
}

pub fn exec_2(instructions: Vec<[String; 2]>) -> i32 {
    let mut indices: Vec<i32> = vec![];

    for i in 0..instructions.len() - 1 {
        if instructions[i][0] == "nop" || instructions[i][0] == "jmp" {
            indices.push(i as i32);
        }
    }

    let mut result: i32 = 0;
    while !indices.is_empty() {
        result = exec_single_instruction_terminate_correctly(
            0,
            &mut indices,
            &instructions,
            &mut vec![],
            0,
        )
    }

    result
}

fn exec_single_instruction_on_inf_loop(
    index: i32,
    instructions: &Vec<[String; 2]>,
    memo: &mut std::vec::Vec<i32>,
    result: i32,
) -> i32 {
    if memo.contains(&index) || index >= instructions.len() as i32 {
        return result;
    };

    match instructions[index as usize][0].as_str() {
        "nop" => nop(index, instructions, memo, result),
        "acc" => acc(index, instructions, memo, result),
        "jmp" => jmp(index, instructions, memo, result),
        _ => 0,
    }
}

fn exec_single_instruction_terminate_correctly(
    index: i32,
    indices: &mut Vec<i32>,
    instructions: &Vec<[String; 2]>,
    memo: &mut std::vec::Vec<i32>,
    result: i32,
) -> i32 {
    if index >= instructions.len() as i32 {
        indices.clear();
        return result;
    };

    if memo.contains(&index) {
        indices.remove(0);
        return result;
    }

    match instructions[index as usize][0].as_str() {
        "nop" => {
            memo.push(index);
            if indices[0] == index && index > 0 {
                exec_single_instruction_terminate_correctly(
                    index + instructions[index as usize][1].parse::<i32>().unwrap(),
                    indices,
                    instructions,
                    memo,
                    result,
                )
            } else {
                exec_single_instruction_terminate_correctly(
                    index + 1,
                    indices,
                    instructions,
                    memo,
                    result,
                )
            }
        }
        "acc" => {
            memo.push(index);
            exec_single_instruction_terminate_correctly(
                index + 1,
                indices,
                instructions,
                memo,
                result + instructions[index as usize][1].parse::<i32>().unwrap(),
            )
        }
        "jmp" => {
            memo.push(index);
            if indices[0] == index {
                exec_single_instruction_terminate_correctly(
                    index + 1,
                    indices,
                    instructions,
                    memo,
                    result,
                )
            } else {
                exec_single_instruction_terminate_correctly(
                    index + instructions[index as usize][1].parse::<i32>().unwrap(),
                    indices,
                    instructions,
                    memo,
                    result,
                )
            }
        }
        _ => 0,
    }
}

fn nop(
    index: i32,
    instructions: &Vec<[String; 2]>,
    memo: &mut std::vec::Vec<i32>,
    result: i32,
) -> i32 {
    memo.push(index);
    exec_single_instruction_on_inf_loop(index + 1, instructions, memo, result)
}

fn acc(
    index: i32,
    instructions: &Vec<[String; 2]>,
    memo: &mut std::vec::Vec<i32>,
    mut result: i32,
) -> i32 {
    memo.push(index);
    result += instructions[index as usize][1].parse::<i32>().unwrap();
    exec_single_instruction_on_inf_loop(index + 1, instructions, memo, result)
}

fn jmp(
    index: i32,
    instructions: &Vec<[String; 2]>,
    memo: &mut std::vec::Vec<i32>,
    result: i32,
) -> i32 {
    memo.push(index);
    exec_single_instruction_on_inf_loop(
        index + instructions[index as usize][1].parse::<i32>().unwrap(),
        instructions,
        memo,
        result,
    )
}

pub fn convert_exec(instructions: Vec<String>) -> Vec<[String; 2]> {
    let converted: Vec<[String; 2]> = instructions
        .into_iter()
        .map(|i| {
            let i: Vec<&str> = i.split(" ").collect();
            [
                i[0].to_string(),
                i[1].to_string().parse::<i32>().unwrap().to_string(),
            ]
        })
        .collect();

    converted
}

fn read_input() -> Vec<String> {
    let path = "./src/input_files/day8.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.split("\n").map(|f| f.parse().unwrap()).collect()
}

pub fn _1() -> i32 {
    let instructions: Vec<String> = read_input();

    let instructions = convert_exec(instructions);
    exec_1(instructions)
}

pub fn _2() -> i32 {
    let instructions: Vec<String> = read_input();

    let instructions = convert_exec(instructions);
    exec_2(instructions)
}
