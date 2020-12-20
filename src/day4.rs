use regex::Regex;

use crate::utils::read_file;

static REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn main() {
    let file_contents = read_file("day4.txt").unwrap();
    let one_star_result = count_valid_passwords(&file_contents, false);
    println!("One star result has {} valid passports", one_star_result);
    let two_star_result = count_valid_passwords(&file_contents, true);
    println!("Two star result has {} valid passports", two_star_result);
}

pub fn year_min_max(value: &str, min: i32, max: i32) -> bool {
    value
        .parse::<i32>()
        .map_or(false, |number| number >= min && number <= max)
}

fn count_valid_passwords(value: &str, run_validators: bool) -> usize {
    let hex_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let eye_color_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let passport_number_regex = Regex::new(r"^[\d]{9}$").unwrap();
    value
        .split("\n\n")
        .filter(|passport| {
            let valid_field_count = passport
                .split([' ', '\n'].as_ref())
                .filter(|field| {
                    let mut field_iterator = field.split(':').take(2);
                    field_iterator.next().map_or(false, |name| {
                        REQUIRED_FIELDS.contains(&name)
                            && (!run_validators
                                || field_iterator.next().map_or(false, |value| match name {
                                    "byr" => year_min_max(value, 1920, 2002),
                                    "iyr" => year_min_max(value, 2010, 2020),
                                    "eyr" => year_min_max(value, 2020, 2030),
                                    "hgt" => {
                                        let (unit_value, unit) = value.split_at(value.len() - 2);
                                        unit_value.parse::<i32>().map_or(false, |value| {
                                            match unit {
                                                "cm" => value >= 150 && value <= 193,
                                                "in" => value >= 59 && value <= 76,
                                                _ => false,
                                            }
                                        })
                                    }
                                    "hcl" => hex_regex.is_match(value),
                                    "ecl" => eye_color_regex.is_match(value),
                                    "pid" => passport_number_regex.is_match(value),
                                    _ => true,
                                }))
                    })
                })
                .count();
            //println!("{}", field_count);
            valid_field_count >= REQUIRED_FIELDS.len()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::count_valid_passwords;

    #[test]
    fn test_for_one_star() {
        let test_passports = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let valid_password_count = count_valid_passwords(test_passports, false);
        assert_eq!(2, valid_password_count);
    }

    #[test]
    fn test_for_two_star() {
        let invalid_passports = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let valid_passports = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let invalid_count = count_valid_passwords(invalid_passports, true);
        assert_eq!(0, invalid_count);
        let valid_count = count_valid_passwords(valid_passports, true);
        assert_eq!(4, valid_count);
    }
}
