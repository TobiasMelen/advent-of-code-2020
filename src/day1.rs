use crate::utils::read_integer_file;

pub fn main() {
    let integers = read_integer_file("day1.txt").unwrap();
    let one_star_result = find_combination_target_sum(&integers, 2020, 2);
    println!("One star result is {:?}", one_star_result.unwrap_or(0));
    let two_start_result = find_combination_target_sum(&integers, 2020, 3);
    println!("Two star result is {:?}", two_start_result.unwrap_or(0));
}

pub fn find_combination_target_sum(
    values: &[isize],
    target_sum: isize,
    combinators: isize,
) -> Option<isize> {
    struct Input<'a> {
        values: &'a [isize],
        target_sum: isize,
        combinators: isize,
    }
    fn recurse(
        input: &Input,
        start_index: usize,
        acc_sum: isize,
        current_combinators: isize,
    ) -> Option<isize> {
        input.values[start_index..]
            .iter()
            .enumerate()
            .find_map(|(index, item)| {
                if current_combinators == input.combinators {
                    if acc_sum + item == input.target_sum {
                        Some(*item)
                    } else {
                        None
                    }
                } else {
                    recurse(
                        input,
                        start_index + index,
                        acc_sum + item,
                        current_combinators + 1,
                    )
                    .map(|result| result * item)
                }
            })
    }
    recurse(
        &Input {
            values,
            target_sum,
            combinators,
        },
        0,
        0,
        1,
    )
}

#[cfg(test)]
mod tests {
    use super::find_combination_target_sum;

    #[test]
    fn example_one_star() {
        let result = find_combination_target_sum(&[1721, 979, 366, 299, 675, 1456], 2020, 2);
        assert_eq!(514579, result.unwrap())
    }

    #[test]
    fn example_two_star() {
        let result = find_combination_target_sum(&[979, 1, 366, 1, 675, 1, 1], 2020, 3);
        assert_eq!(241861950, result.unwrap());
    }
}
