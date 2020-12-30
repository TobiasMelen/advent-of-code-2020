use std::collections::HashMap;

use crate::utils::read_integer_file;

pub fn main() {
    let mut input = read_integer_file("day10.txt").unwrap();
    input.sort();
    let (one, _, three) = get_complete_chain_jump_count(&input);
    let one_star = one * three;
    println!("Adapters in chain for one star is {}", one_star);
    let two_star = count_all_possible_adapter_combinations_dynamic(&input);
    println!("There are {} possible combinations for two stars", two_star);
}

type JoltJumps = (isize, isize, isize);

fn get_complete_chain_jump_count(adapters: &[isize]) -> JoltJumps {
    adapters
        .iter()
        .fold(
            (
                0, //Chain starts with a zero value
                (
                    0, 0, 1, //There will be a last jump of three after adapters
                ),
            ),
            |(prev_value, (one, two, three)), value| {
                (
                    *value,
                    match value - prev_value {
                        1 => (one + 1, two, three),
                        2 => (one, two + 1, three),
                        3 => (one, two, three + 1),
                        value => {
                            panic!(format!("Invalid jump length of {} in adapter chain", value))
                        }
                    },
                )
            },
        )
        .1
}

//Recursive tracking of every branch is way to slow. Calculate jump possibilites to last with dynamic programming.
//Idea stolen from great explanation here https://dev.to/sleeplessbyte/comment/194fe;
fn count_all_possible_adapter_combinations_dynamic(adapters: &[isize]) -> isize {
    let back_indexes: &[isize] = &[1, 2, 3];
    let initial_seed = [(0, 1)].iter().cloned().collect::<HashMap<_, _>>();
    let combinations = adapters.iter().fold(initial_seed, |mut map, value| {
        let possible_combinations_to_index: isize = back_indexes
            .iter()
            .filter_map(|behind| map.get(&(value - behind)))
            .sum();
        map.insert(*value, possible_combinations_to_index);
        map
    });
    *combinations.values().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::{count_all_possible_adapter_combinations_dynamic, get_complete_chain_jump_count};

    fn get_input() -> [isize; 31] {
        let mut input = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        input.sort();
        input
    }

    #[test]
    fn test_for_one_star() {
        let (one, _, three) = get_complete_chain_jump_count(&get_input());
        assert_eq!(220, one * three);
    }

    #[test]
    fn test_for_two_star() {
        let result = count_all_possible_adapter_combinations_dynamic(&get_input());
        assert_eq!(19208, result);
    }
}
