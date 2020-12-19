use crate::utils::read_integer_file;

pub fn main() {
    let integers = read_integer_file("day1.txt").unwrap();
    let one_star_result = solve(&integers, 2020, 2);
    println!("One star result is {:?}", one_star_result.unwrap_or(0));
    let two_start_result = solve(&integers, 2020, 3);
    println!("Two star result is {:?}", two_start_result.unwrap_or(0));
}

fn find_result_recursive(
    inputs: &[i32],
    sum_for: i32,
    addends: usize,
    start_index: usize,
    adds: &[i32],
) -> Option<i32> {
    inputs
        .iter()
        .skip(start_index)
        .enumerate()
        .find_map(|(index, item)| {
            let mut result_set = adds.to_vec();
            result_set.push(*item);
            if adds.len() < addends - 1 {
                find_result_recursive(
                    inputs,
                    sum_for,
                    addends,
                    start_index + index,
                    &result_set,
                )
            } else {
                let sum = result_set.iter().sum::<i32>();
                if sum == sum_for {
                    Some(result_set.iter().fold(1, |acc, item| acc * item))
                } else {
                    None
                }
            }
        })
}

fn solve(inputs: &[i32], sum_for: i32, addends: usize) -> Option<i32> {
    find_result_recursive(inputs, sum_for, addends, 0, &[])
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
        let result = solve(&[979, 366, 675], 2020, 3);
        assert_eq!(241861950, result.unwrap());
    }
}
