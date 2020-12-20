use crate::utils::read_file_lines;

struct Slope {
    right: usize,
    down: usize,
}

static ONE_STAR_SLOPES: &[Slope] = &[Slope { right: 3, down: 1 }];
static TWO_STAR_SLOPES: &[Slope] = &[
    Slope { right: 1, down: 1 },
    Slope { right: 3, down: 1 },
    Slope { right: 5, down: 1 },
    Slope { right: 7, down: 1 },
    Slope { right: 1, down: 2 },
];

pub fn main() {
    let input = read_file_lines("day3.txt").unwrap();
    let one_star_encounters = multiply_tree_slope_encounters(ONE_STAR_SLOPES, &input);
    println!("One star encounters {} trees", one_star_encounters);
    let two_star_encounters = multiply_tree_slope_encounters(TWO_STAR_SLOPES, &input);
    println!("Product of two star encounters is {}", two_star_encounters);
}

fn multiply_tree_slope_encounters<S: AsRef<str>>(
    slopes: &[Slope],
    tree_line_patterns: &[S],
) -> usize {
    slopes.iter().fold(1, |product, slope| {
        let (_, encounters) = tree_line_patterns.iter().step_by(slope.down).fold(
            (0, 0),
            |(right_pos, encounters), tree_line| {
                let encounter = tree_line
                    .as_ref()
                    .chars()
                    .cycle()
                    .nth(right_pos)
                    .map_or(false, |char| char == '#');
                (right_pos + slope.right, encounters + encounter as usize)
            },
        );
        product * encounters
    })
}

#[cfg(test)]
mod tests {
    use super::{multiply_tree_slope_encounters, ONE_STAR_SLOPES, TWO_STAR_SLOPES};
    static TEST_INPUT: &[&str] = &[
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];
    #[test]
    fn example_one_star() {
        let encounters = multiply_tree_slope_encounters(ONE_STAR_SLOPES, TEST_INPUT);
        assert_eq!(7, encounters)
    }
    #[test]
    fn example_two_star() {
        let encounters = multiply_tree_slope_encounters(TWO_STAR_SLOPES, TEST_INPUT);
        assert_eq!(336, encounters)
    }
}
