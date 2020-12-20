#[cfg(test)]
mod day_8 {
    use adventofcode_2020::day08::{convert_exec, exec_1, exec_2};
    use std::vec;

    #[test]
    fn convert_instructoins_to_list() {
        let instructions: Vec<String> = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];
        let expected: Vec<[String; 2]> = vec![
            ["nop".to_string(), "0".to_string()],
            ["acc".to_string(), "1".to_string()],
            ["jmp".to_string(), "4".to_string()],
            ["acc".to_string(), "3".to_string()],
            ["jmp".to_string(), "-3".to_string()],
            ["acc".to_string(), "-99".to_string()],
            ["acc".to_string(), "1".to_string()],
            ["jmp".to_string(), "-4".to_string()],
            ["acc".to_string(), "6".to_string()],
        ];

        assert_eq!(convert_exec(instructions), expected);
    }

    #[test]
    fn sequence_should_stop_with_acc_5() {
        let instructions: Vec<String> = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        let instructions = convert_exec(instructions);

        assert_eq!(exec_1(instructions), 5);
    }

    #[test]
    fn sequence_should_stop_with_acc_8() {
        let instructions: Vec<String> = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        let instructions = convert_exec(instructions);

        assert_eq!(exec_2(instructions), 8);
    }
}
