#[cfg(test)]
mod day_16 {
    use std::collections::HashMap;

    use adventofcode_2020::day16::{get_error_rate, to_map};

    fn get_test_map_part_1() -> HashMap<String, Vec<i32>> {
        to_map(
            "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
                .split("\n\n")
                .map(From::from)
                .collect(),
        )
    }

    fn _get_test_map_part_2() -> HashMap<String, Vec<i32>> {
        to_map(
            "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
                .split("\n\n")
                .map(From::from)
                .collect(),
        )
    }

    #[test]
    fn error_rate_is_0() {
        let ticket = get_test_map_part_1();
        let result = get_error_rate("7,3,47".to_string(), ticket);
        assert_eq!(result, 0);
    }

    #[test]
    fn error_rate_is_4() {
        let ticket = get_test_map_part_1();
        let result = get_error_rate("40,4,50".to_string(), ticket);
        assert_eq!(result, 4);
    }

    #[test]
    fn error_rate_is_55() {
        let ticket = get_test_map_part_1();
        let result = get_error_rate("55,2,20".to_string(), ticket);
        assert_eq!(result, 55);
    }

    #[test]
    fn error_rate_is_12() {
        let ticket = get_test_map_part_1();
        let result = get_error_rate("38,6,12".to_string(), ticket);
        assert_eq!(result, 12);
    }

    #[test]
    fn convert_ticket_small() {
        let ticket = get_test_map_part_1();

        let mut expected = HashMap::new();
        expected.insert("class".to_string(), vec![1, 3, 5, 7]);
        expected.insert("row".to_string(), vec![6, 11, 33, 44]);
        expected.insert("seat".to_string(), vec![13, 40, 45, 50]);
        assert_eq!(ticket, expected);
    }

    #[test]
    fn convert_ticket_large() {
        let ticket = to_map(
            "departure location: 40-261 or 279-955
departure station: 33-375 or 394-963
departure platform: 39-863 or 877-970
departure track: 30-237 or 256-955
departure date: 47-731 or 741-950
departure time: 38-301 or 317-954
arrival location: 26-598 or 623-969
arrival station: 50-835 or 854-971
arrival platform: 44-535 or 549-958
arrival track: 36-672 or 685-967
class: 34-217 or 236-974
duration: 29-469 or 483-970
price: 45-111 or 120-965
route: 32-751 or 760-954
row: 25-321 or 339-954
seat: 38-423 or 438-958
train: 45-798 or 813-954
type: 40-487 or 503-954
wagon: 46-916 or 938-949
zone: 25-160 or 184-957

your ticket:
73,59,83,127,137,151,71,139,67,53,89,79,61,109,131,103,149,97,107,101

nearby tickets:
782,297,512,592,171,360,774,483,653,294,299,519,448,916,939,293,535,63,54,648
650,284,299,520,890,792,654,774,639,887,655,654,313,662,83,650,137,570,700,505"
                .split("\n\n")
                .map(From::from)
                .collect(),
        );

        let mut expected = HashMap::new();
        expected.insert("departure location".to_string(), vec![40, 261, 279, 955]);
        expected.insert("departure station".to_string(), vec![33, 375, 394, 963]);
        expected.insert("departure platform".to_string(), vec![39, 863, 877, 970]);
        expected.insert("departure track".to_string(), vec![30, 237, 256, 955]);
        expected.insert("departure date".to_string(), vec![47, 731, 741, 950]);
        expected.insert("departure time".to_string(), vec![38, 301, 317, 954]);
        expected.insert("arrival location".to_string(), vec![26, 598, 623, 969]);
        expected.insert("arrival station".to_string(), vec![50, 835, 854, 971]);
        expected.insert("arrival platform".to_string(), vec![44, 535, 549, 958]);
        expected.insert("arrival track".to_string(), vec![36, 672, 685, 967]);
        expected.insert("class".to_string(), vec![34, 217, 236, 974]);
        expected.insert("duration".to_string(), vec![29, 469, 483, 970]);
        expected.insert("price".to_string(), vec![45, 111, 120, 965]);
        expected.insert("route".to_string(), vec![32, 751, 760, 954]);
        expected.insert("row".to_string(), vec![25, 321, 339, 954]);
        expected.insert("seat".to_string(), vec![38, 423, 438, 958]);
        expected.insert("train".to_string(), vec![45, 798, 813, 954]);
        expected.insert("type".to_string(), vec![40, 487, 503, 954]);
        expected.insert("wagon".to_string(), vec![46, 916, 938, 949]);
        expected.insert("zone".to_string(), vec![25, 160, 184, 957]);

        assert_eq!(ticket, expected);
    }
}
