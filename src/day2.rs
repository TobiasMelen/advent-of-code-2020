use crate::utils::read_file_lines;

pub fn main() {
    let file_lines = read_file_lines("day2.txt").unwrap();
    let one_star = count_valid_passwords(&file_lines, &get_password_validity_one_star);
    println!("One star result is {}", one_star);
    let two_star = count_valid_passwords(&file_lines, &get_password_validity_two_star);
    println!("Two star result is {}", two_star);
}

struct RuledPassword<'a> {
    password: &'a str,
    char: char,
    min: i32,
    max: i32,
}

fn count_valid_passwords<S: AsRef<str>>(
    passwords: &[S],
    validator: &dyn Fn(&RuledPassword) -> bool,
) -> usize {
    passwords
        .iter()
        .map(|value| extract_variables(value.as_ref()).unwrap())
        .filter(|value| validator(value))
        .count()
}

fn get_password_validity_one_star(ruled: &RuledPassword) -> bool {
    let occurences =
        ruled.password.chars().fold(
            0,
            |count, char| {
                if ruled.char == char {
                    count + 1
                } else {
                    count
                }
            },
        );
    occurences >= ruled.min && occurences <= ruled.max
}

fn get_password_validity_two_star(ruled: &RuledPassword) -> bool {
    let mut chars = ruled.password.chars();
    let first_index = (ruled.min - 1) as usize;
    let second_index = (ruled.max - 1) as usize;
    let min_matches = chars
        .nth(first_index)
        .map(|char| char == ruled.char)
        .unwrap_or(false);
    let max_matches = chars
        .nth(second_index - (first_index + 1))
        .map(|char| char == ruled.char)
        .unwrap_or(false);
    (max_matches && !min_matches) || (min_matches && !max_matches)
}

fn extract_variables(input: &str) -> Option<RuledPassword> {
    let mut result = input.split(" ");
    let mut min_max = result.next().map(|val| {
        val.split("-")
            .map(|digit| digit.parse::<i32>().unwrap_or(0))
    })?;
    Some(RuledPassword {
        char: result.next()?.chars().next()?,
        password: result.next()?,
        min: min_max.next()?,
        max: min_max.next()?,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        count_valid_passwords, get_password_validity_one_star, get_password_validity_two_star,
    };

    #[test]
    fn example_one_star() {
        let input = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let valid_password_count = count_valid_passwords(&input, &get_password_validity_one_star);
        assert_eq!(2, valid_password_count)
    }

    #[test]
    fn example_two_star() {
        let input = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let valid_password_count = count_valid_passwords(&input, &get_password_validity_two_star);
        assert_eq!(1, valid_password_count);
    }
}
