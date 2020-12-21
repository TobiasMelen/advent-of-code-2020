use crate::utils::read_file_lines;

pub fn main() {
    let input = read_file_lines("day5.txt").unwrap();
    let mut seat_ids = input
        .iter()
        .map(|line| translate_to_binary(&line))
        .collect::<Vec<_>>();
    seat_ids.sort();
    println!(
        "Highest seat id for one star is {}",
        seat_ids.last().unwrap_or(&0)
    );
    println!(
        "Your seat for two star is {}",
        seat_ids
            .windows(2)
            .find_map(|window| {
                if window[1] - window[0] == 2 {
                    Some(window[0] + 1)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    )
}

//I had a binary search that kept getting of by a half, so i googled.
//One should never google these, now i just did the same binary-translation as everyone else.
//Cheat mode:
fn translate_to_binary(position_descriptor: &str) -> i32 {
    let binary = position_descriptor
        .chars()
        .map(|char| match char {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => panic!(),
        })
        .collect::<String>();
    i32::from_str_radix(&binary, 2).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::translate_to_binary;

    #[test]
    fn test_for_one_star() {
        assert_eq!(567, translate_to_binary("BFFFBBFRRR"));
        assert_eq!(119, translate_to_binary("FFFBBBFRRR"));
        assert_eq!(820, translate_to_binary("BBFFBBFRLL"));
    }
}
