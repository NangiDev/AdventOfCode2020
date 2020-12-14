#[cfg(test)]
mod day_1 {
    use adventofcode_2020::day14::{convert_params_to_objects, Parameter};

    #[test]
    fn convert_param_to_object() {
        let result: Vec<Parameter> = convert_params_to_objects(vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ]);

        let mut expected: Vec<Parameter> = vec![];
        let mut p1 = Parameter::new(0b0000000000000000000000000001000000);
        p1.add_mem((8, 11));
        p1.add_mem((7, 101));
        p1.add_mem((8, 0));
        expected.push(p1);

        assert_eq!(result[0].mask, expected[0].mask);
        assert_eq!(result[0].mems, expected[0].mems);
    }

    #[test]
    fn convert_multiple_params_to_object() {
        let result: Vec<Parameter> = convert_params_to_objects(vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
            "mask = XX00111XX1010X0110011110X1XX110010X1".to_string(),
            "mem[6] = 34".to_string(),
            "mem[2] = 152".to_string(),
            "mem[4] = 0".to_string(),
        ]);

        let mut expected: Vec<Parameter> = vec![];
        let mut p1 = Parameter::new(0b0000000000000000000000000001000000);
        p1.add_mem((8, 11));
        p1.add_mem((7, 101));
        p1.add_mem((8, 0));
        expected.push(p1);
        let mut p1 = Parameter::new(0b0011100101000110011110010011001001);
        p1.add_mem((6, 34));
        p1.add_mem((2, 152));
        p1.add_mem((4, 0));
        expected.push(p1);

        assert_eq!(result[0].mask, expected[0].mask);
        assert_eq!(result[0].mems, expected[0].mems);
    }
}
