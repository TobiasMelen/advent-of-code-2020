use crate::{day1::find_combination_target_sum, utils::read_integer_file};

pub fn main() {
    let input = read_integer_file("day9.txt").unwrap();
    let one_star = find_first_invalid(&input, 25);
    println!("First invalid number for one star is {:?}", one_star);
    let two_star = find_continuous_range_minmax_for_sum(&input, one_star.unwrap());
    println!(
        "Max and min of continuous series for sum and two star result is {:?}",
        two_star
    );
}

fn find_first_invalid(data: &[isize], preamble: usize) -> Option<isize> {
    data.iter()
        .enumerate()
        .skip(preamble as usize)
        .find_map(|(i, value)| {
            //the preamble number of numbers before current;
            let control_numbers = &data[i - preamble..i];
            match find_combination_target_sum(control_numbers, *value, 2) {
                Some(_) => None,
                None => Some(*value),
            }
        })
}

fn find_continuous_range_minmax_for_sum(data: &[isize], target_sum: isize) -> Option<isize> {
    data.iter().enumerate().find_map(|(index, _)| {
        let mut sequence_sum: isize = 0;
        let mut sequence_min = None;
        let mut sequence_max = None;
        for value in data[index..].iter() {
            sequence_sum += value;
            sequence_min = sequence_min
                .map(|min| if value < min { min } else { value })
                .or(Some(value));
            sequence_max = sequence_max
                .map(|max| if value > max { max } else { value })
                .or(Some(value));
            match sequence_sum - target_sum {
                0 => return sequence_min.zip(sequence_max).map(|(min, max)| min + max),
                d if d > 0 => break,
                _ => continue,
            }
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::{find_continuous_range_minmax_for_sum, find_first_invalid};

    static INPUT: &[isize] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_for_one_star() {
        assert_eq!(Some(127), find_first_invalid(INPUT, 5));
    }

    #[test]
    fn test_for_two_star() {
        assert_eq!(Some(62), find_continuous_range_minmax_for_sum(INPUT, 127));
    }
}
