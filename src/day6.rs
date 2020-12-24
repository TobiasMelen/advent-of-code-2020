use std::collections::{HashMap, HashSet};

use crate::utils::read_file;

pub fn main() {
    let input = read_file("day6.txt").unwrap_or(String::from(""));
    let one_star_result = sum_all_group_answers(&input, &count_scattered_yes_answers);
    println!("Result for one star is {} answers", one_star_result);
    let two_star_result: i32 = sum_all_group_answers(&input, &count_collective_yes_answers);
    println!("Result for two star is {} answers", two_star_result);
}

fn count_scattered_yes_answers(answers: &str) -> i32 {
    answers
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect::<HashSet<_>>()
        .len() as i32
}

fn count_collective_yes_answers(answers: &str) -> i32 {
    let grouped_answers = answers
        .chars()
        .filter(|char| char.is_alphabetic())
        .fold(HashMap::new(), |mut set, char| {
            set.insert(char, set.get(&char).unwrap_or(&0) + 1);
            set
        });
    let group_participant_count = answers
        .lines()
        //filter empty lines
        .filter(|line| !line.is_empty())
        .count();
    grouped_answers
        .iter()
        .filter(|(_, answer_count)| *answer_count == &(group_participant_count as i32))
        .count() as i32
}

fn sum_all_group_answers(answers: &str, group_counter: &dyn Fn(&str) -> i32) -> i32 {
    answers.split("\n\n").map(group_counter).sum()
}

#[cfg(test)]
mod test {
    use super::{count_collective_yes_answers, count_scattered_yes_answers, sum_all_group_answers};

    static INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b
";

    #[test]
    fn test_for_one_star() {
        assert_eq!(
            11,
            sum_all_group_answers(INPUT, &count_scattered_yes_answers)
        );
    }

    #[test]
    fn test_for_two_star() {
        assert_eq!(
            6,
            sum_all_group_answers(INPUT, &count_collective_yes_answers)
        );
    }
}
