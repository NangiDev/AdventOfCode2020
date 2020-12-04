use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn read_input() -> Vec<HashMap<String, String>> {
    let path = "./src/input_files/day4.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let batches: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();

    let mut passports: Vec<HashMap<String, String>> = vec![];
    for b in batches {
        let mut map: HashMap<String, String> = HashMap::new();
        let pairs: Vec<String> = b.split_whitespace().map(|s| s.to_string()).collect();
        for p in pairs {
            let pair: Vec<String> = p.split(":").map(|s| s.to_string()).collect();
            map.insert(pair[0].clone(), pair[1].clone());
        }
        passports.push(map);
    }
    passports
}

fn is_valid_passport(map: &HashMap<String, String>) -> bool {
    map.contains_key("ecl")
        && map.contains_key("pid")
        && map.contains_key("eyr")
        && map.contains_key("hcl")
        && map.contains_key("byr")
        && map.contains_key("iyr")
        && map.contains_key("hgt")
}

fn is_fields_valid(map: &HashMap<String, String>) -> bool {
    for (key, value) in &*map {
        match key.as_str() {
            "byr" => {
                let val = value.parse::<i32>().unwrap();
                if val < 1920 || val > 2002 {
                    return false;
                }
            }
            "iyr" => {
                let val = value.parse::<i32>().unwrap();
                if val < 2010 || val > 2020 {
                    return false;
                }
            }
            "eyr" => {
                let val = value.parse::<i32>().unwrap();
                if val < 2020 || val > 2030 {
                    return false;
                }
            }
            "hgt" => {
                let unit: String = value.chars().filter(|c| c.is_alphabetic()).collect();
                if unit == "in" {
                    let t: String = value.chars().filter(|c| c.is_digit(10)).collect();
                    if t.parse::<i32>().unwrap() < 59 || t.parse::<i32>().unwrap() > 76 {
                        return false;
                    }
                } else if unit == "cm" {
                    let t: String = value.chars().filter(|c| c.is_digit(10)).collect();
                    if t.parse::<i32>().unwrap() < 150 || t.parse::<i32>().unwrap() > 193 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                let re = Regex::new(r"#[0-9,a-f]{6}").unwrap();
                if !re.is_match(value) {
                    return false;
                }
            }
            "ecl" => {
                if value != "amb"
                    && value != "blu"
                    && value != "brn"
                    && value != "gry"
                    && value != "grn"
                    && value != "hzl"
                    && value != "oth"
                {
                    return false;
                }
            }
            "pid" => {
                if value.len() != 9 {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

fn filter_complete_passports(
    passports: Vec<HashMap<String, String>>,
) -> Vec<HashMap<String, String>> {
    passports
        .into_iter()
        .filter(|p| is_valid_passport(p))
        .collect()
}
fn filter_valid_passports(
    read_input: Vec<HashMap<String, String>>,
) -> Vec<HashMap<String, String>> {
    let filter = read_input
        .into_iter()
        .filter(|p| is_fields_valid(p))
        .collect();

    filter
}

pub fn _1() -> i32 {
    filter_complete_passports(read_input()).len() as i32
}

pub fn _2() -> i32 {
    let valid_input = filter_complete_passports(read_input());
    filter_valid_passports(valid_input).len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_two_valid_passports() {
        let mut passports: Vec<HashMap<String, String>> = vec![];

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "gry".to_string());
        map.insert("pid".to_string(), "860033327".to_string());
        map.insert("eyr".to_string(), "2020".to_string());
        map.insert("hcl".to_string(), "#fffffd".to_string());
        map.insert("byr".to_string(), "1937".to_string());
        map.insert("iyr".to_string(), "2017".to_string());
        map.insert("cid".to_string(), "147".to_string());
        map.insert("hgt".to_string(), "183cm".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "amb".to_string());
        map.insert("pid".to_string(), "028048884".to_string());
        map.insert("eyr".to_string(), "2023".to_string());
        map.insert("hcl".to_string(), "#cfa07d".to_string());
        map.insert("byr".to_string(), "1929".to_string());
        map.insert("iyr".to_string(), "2013".to_string());
        map.insert("cid".to_string(), "350".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "brn".to_string());
        map.insert("pid".to_string(), "760753108".to_string());
        map.insert("eyr".to_string(), "2024".to_string());
        map.insert("hcl".to_string(), "#ae17e1".to_string());
        map.insert("byr".to_string(), "1931".to_string());
        map.insert("iyr".to_string(), "2013".to_string());
        map.insert("hgt".to_string(), "179cm".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "brn".to_string());
        map.insert("pid".to_string(), "166559648".to_string());
        map.insert("eyr".to_string(), "2025".to_string());
        map.insert("hcl".to_string(), "#cfa07d".to_string());
        map.insert("iyr".to_string(), "2011".to_string());
        map.insert("hgt".to_string(), "59in".to_string());
        passports.push(map);

        assert_eq!(filter_complete_passports(passports).len() as i32, 2);
    }

    #[test]
    fn find_four_valid_passports() {
        let mut passports: Vec<HashMap<String, String>> = vec![];

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "grn".to_string());
        map.insert("pid".to_string(), "087499704".to_string());
        map.insert("eyr".to_string(), "2030".to_string());
        map.insert("hcl".to_string(), "#623a2f".to_string());
        map.insert("byr".to_string(), "1980".to_string());
        map.insert("iyr".to_string(), "2012".to_string());
        map.insert("hgt".to_string(), "74in".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "blu".to_string());
        map.insert("pid".to_string(), "896056539".to_string());
        map.insert("eyr".to_string(), "2029".to_string());
        map.insert("hcl".to_string(), "#a97842".to_string());
        map.insert("byr".to_string(), "1989".to_string());
        map.insert("iyr".to_string(), "2014".to_string());
        map.insert("cid".to_string(), "129".to_string());
        map.insert("hgt".to_string(), "165cm".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "hzl".to_string());
        map.insert("pid".to_string(), "545766238".to_string());
        map.insert("eyr".to_string(), "2022".to_string());
        map.insert("hcl".to_string(), "#888785".to_string());
        map.insert("byr".to_string(), "2001".to_string());
        map.insert("iyr".to_string(), "2015".to_string());
        map.insert("cid".to_string(), "88".to_string());
        map.insert("hgt".to_string(), "164cm".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "blu".to_string());
        map.insert("pid".to_string(), "093154719".to_string());
        map.insert("eyr".to_string(), "2021".to_string());
        map.insert("hcl".to_string(), "#b6652a".to_string());
        map.insert("byr".to_string(), "1944".to_string());
        map.insert("iyr".to_string(), "2010".to_string());
        map.insert("hgt".to_string(), "158cm".to_string());
        passports.push(map);

        assert_eq!(filter_valid_passports(passports).len() as i32, 4);
    }

    #[test]
    fn find_zero_valid_passports() {
        let mut passports: Vec<HashMap<String, String>> = vec![];

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "amb".to_string());
        map.insert("pid".to_string(), "186cm".to_string());
        map.insert("eyr".to_string(), "1972".to_string());
        map.insert("hcl".to_string(), "#18171d".to_string());
        map.insert("byr".to_string(), "1926".to_string());
        map.insert("iyr".to_string(), "2018".to_string());
        map.insert("cid".to_string(), "100".to_string());
        map.insert("hgt".to_string(), "170".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "grn".to_string());
        map.insert("pid".to_string(), "012533040".to_string());
        map.insert("eyr".to_string(), "1967".to_string());
        map.insert("hcl".to_string(), "#602927".to_string());
        map.insert("byr".to_string(), "1946".to_string());
        map.insert("iyr".to_string(), "2019".to_string());
        map.insert("hgt".to_string(), "170cm".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "brn".to_string());
        map.insert("pid".to_string(), "021572410".to_string());
        map.insert("eyr".to_string(), "2020".to_string());
        map.insert("hcl".to_string(), "dab227".to_string());
        map.insert("byr".to_string(), "1992".to_string());
        map.insert("iyr".to_string(), "2012".to_string());
        map.insert("cid".to_string(), "277".to_string());
        map.insert("hgt".to_string(), "182cm".to_string());
        passports.push(map);

        map = HashMap::new();
        map.insert("ecl".to_string(), "zzz".to_string());
        map.insert("pid".to_string(), "3556412378".to_string());
        map.insert("eyr".to_string(), "2038".to_string());
        map.insert("hcl".to_string(), "74454a".to_string());
        map.insert("byr".to_string(), "2007".to_string());
        map.insert("iyr".to_string(), "2023".to_string());
        map.insert("hgt".to_string(), "59cm".to_string());
        passports.push(map);

        assert_eq!(filter_valid_passports(passports).len() as i32, 0);
    }

    #[test]
    fn passport_with_all_fields_present_is_valid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "gry".to_string());
        map.insert("pid".to_string(), "860033327".to_string());
        map.insert("eyr".to_string(), "2020".to_string());
        map.insert("hcl".to_string(), "#fffffd".to_string());
        map.insert("byr".to_string(), "1937".to_string());
        map.insert("iyr".to_string(), "2017".to_string());
        map.insert("cid".to_string(), "147".to_string());
        map.insert("hgt".to_string(), "183cm".to_string());
        assert_eq!(is_valid_passport(&map), true);
    }

    #[test]
    fn passport_with_with_missing_hgt_is_invalid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "amb".to_string());
        map.insert("pid".to_string(), "028048884".to_string());
        map.insert("eyr".to_string(), "2023".to_string());
        map.insert("hcl".to_string(), "#cfa07d".to_string());
        map.insert("byr".to_string(), "1929".to_string());
        map.insert("iyr".to_string(), "2013".to_string());
        map.insert("cid".to_string(), "350".to_string());
        assert_eq!(is_valid_passport(&map), false);
    }

    #[test]
    fn passport_with_with_missing_cid_is_valid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "brn".to_string());
        map.insert("pid".to_string(), "760753108".to_string());
        map.insert("eyr".to_string(), "2024".to_string());
        map.insert("hcl".to_string(), "#ae17e1".to_string());
        map.insert("byr".to_string(), "1931".to_string());
        map.insert("iyr".to_string(), "2013".to_string());
        map.insert("hgt".to_string(), "179cm".to_string());
        assert_eq!(is_valid_passport(&map), true);
    }

    #[test]
    fn passport_with_missing_cid_and_byr_is_invalid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "brn".to_string());
        map.insert("pid".to_string(), "166559648".to_string());
        map.insert("eyr".to_string(), "2025".to_string());
        map.insert("hcl".to_string(), "#cfa07d".to_string());
        map.insert("iyr".to_string(), "2011".to_string());
        map.insert("hgt".to_string(), "59in".to_string());
        assert_eq!(is_valid_passport(&map), false);
    }

    #[test]
    fn validate_byr_is_valid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("byr".to_string(), "2002".to_string());
        assert_eq!(is_fields_valid(&map), true);
    }
    #[test]
    fn validate_byr_is_invalid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("byr".to_string(), "2003".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }

    #[test]
    fn validate_iyr_is_valid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("iyr".to_string(), "2010".to_string());
        assert_eq!(is_fields_valid(&map), true);
    }
    #[test]
    fn validate_iyr_is_invalid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("iyr".to_string(), "2009".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }
    #[test]
    fn validate_eyr_is_valid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("eyr".to_string(), "2030".to_string());
        assert_eq!(is_fields_valid(&map), true);
    }
    #[test]
    fn validate_eyr_is_invalid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("eyr".to_string(), "2019".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }
    #[test]
    fn validate_hgt_is_valid_unit_inch() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("hgt".to_string(), "60in".to_string());
        assert_eq!(is_fields_valid(&map), true);
    }
    #[test]
    fn validate_hgt_is_valid_unit_cm() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("hgt".to_string(), "190cm".to_string());
        assert_eq!(is_fields_valid(&map), true);
    }
    #[test]
    fn validate_hgt_is_invalid_unit_inch() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("hgt".to_string(), "190in".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }
    #[test]
    fn validate_hgt_is_invalid_unit_none() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("hgt".to_string(), "190".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }
    #[test]
    fn validate_hcl_is_valid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("hcl".to_string(), "#123abc".to_string());
        assert_eq!(is_fields_valid(&map), true);
    }
    #[test]
    fn validate_hcl_is_invalid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("hcl".to_string(), "#123abz".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }
    #[test]
    fn validate_hcl_is_invalid_length() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("hcl".to_string(), "123abc".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }
    #[test]
    fn validate_ecl_is_valid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "brn".to_string());
        assert_eq!(is_fields_valid(&map), true);
    }
    #[test]
    fn validate_ecl_is_invalid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("ecl".to_string(), "wat".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }
    #[test]
    fn validate_pid_is_valid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("pid".to_string(), "000000001".to_string());
        assert_eq!(is_fields_valid(&map), true);
    }
    #[test]
    fn validate_pid_is_invalid() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("pid".to_string(), "0123456789".to_string());
        assert_eq!(is_fields_valid(&map), false);
    }
}
