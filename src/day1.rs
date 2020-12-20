use crate::utils::read_integer_file;

pub fn main() {
    let integers = read_integer_file("day1.txt").unwrap();
    let one_star_result = solve(&integers, 2020, 2);
    println!("One star result is {:?}", one_star_result.unwrap_or(0));
    let two_start_result = solve(&integers, 2020, 3);
    println!("Two star result is {:?}", two_start_result.unwrap_or(0));
}

fn solve(values: &[i32], target_sum: i32, parts_to_sum: i32) -> Option<i32> {
    struct Input<'a> {
        values: &'a [i32],
        target_sum: i32,
        parts_to_sum: i32,
    }
    fn recurse(input: &Input, start_index: usize, acc_sum: i32, depth: i32) -> Option<i32> {
        input
            .values[start_index..]
            .iter()
            .enumerate()
            .find_map(|(index, item)| {
                if depth == input.parts_to_sum {
                    if acc_sum + item == input.target_sum {
                        Some(*item)
                    } else {
                        None
                    }
                } else {
                    recurse(input, start_index + index, acc_sum + item, depth + 1)
                        .map(|result| result * item)
                }
            })
    }
    recurse(
        &Input {
            values,
            target_sum,
            parts_to_sum,
        },
        0,
        0,
        1,
    )
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_one_star() {
        let result = solve(&[1721, 979, 366, 299, 675, 1456], 2020, 2);
        assert_eq!(514579, result.unwrap())
    }

    #[test]
    fn example_two_star() {
        let result = solve(&[979, 1, 366, 1, 675, 1 , 1], 2020, 3);
        assert_eq!(241861950, result.unwrap());
    }
}
