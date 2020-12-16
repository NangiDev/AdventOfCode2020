#[cfg(test)]
mod day_1 {
    use adventofcode_2020::day14::{apply_mask, convert_byte, sum_memory};

    #[test]
    fn apply_mask_11_to_73() {
        let mask: String = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();
        let result: i64 = apply_mask(mask, 11);
        assert_eq!(result, 73);
    }

    #[test]
    fn apply_mask_101_to_101() {
        let mask: String = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();
        let result: i64 = apply_mask(mask, 101);
        assert_eq!(result, 101);
    }

    #[test]
    fn apply_mask_0_to_64() {
        let mask: String = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();
        let result: i64 = apply_mask(mask, 0);
        assert_eq!(result, 64);
    }

    #[test]
    fn convert_bytes_to_i64_73() {
        let result: i64 = convert_byte("0b0000000000000000000000000001001001".to_string());
        assert_eq!(result, 73);
    }

    #[test]
    fn sum_remaining_values_in_mem_to_165() {
        let input: Vec<String> = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];
        let result: i64 = sum_memory(input);
        assert_eq!(result, 165);
    }
}
