use std::collections::HashMap;

use crate::utils::read_file;

pub fn main() {
    let input = read_file("day11.txt").unwrap();
    let parsed_start = parse_seating(&input);
    let one_star = swap_seatings_til_settled(&parsed_start, true, 4);
    println!("Seated for one star is {}", one_star.seated);
    let two_star = swap_seatings_til_settled(&parsed_start, false, 5);
    println!("Seated for two star is {}", two_star.seated);
}
#[derive(PartialEq, Clone, Copy)]
enum SeatStatus {
    NonSeat,
    Free,
    Occupied,
}

struct Seating {
    map: HashMap<(isize, isize), SeatStatus>,
    settled: bool,
    seated: isize,
}

fn parse_seating(content: &str) -> Seating {
    let mut seated = 0;
    let map = content
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (row_nr, line)| {
            line.chars().enumerate().for_each(|(seat_nr, char)| {
                if char == '#' {
                    seated += 1
                };
                map.insert(
                    ((row_nr as isize), (seat_nr as isize)),
                    match char {
                        'L' => SeatStatus::Free,
                        '#' => SeatStatus::Occupied,
                        '.' => SeatStatus::NonSeat,
                        val => panic!("Invalid character {} in input", val),
                    },
                );
            });
            map
        });
    Seating {
        map,
        seated,
        settled: false,
    }
}

static DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn direction_has_neighbour(
    seating: &Seating,
    (seat, row): (&isize, &isize),
    direction: &(isize, isize),
    non_seat_neighbours: bool,
) -> bool {
    let (seat, row) = (seat + direction.0, row + direction.1);
    match seating.map.get(&(row, seat)) {
        Some(SeatStatus::Occupied) => true,
        Some(SeatStatus::NonSeat) if non_seat_neighbours == false => {
            direction_has_neighbour(seating, (&seat, &row), direction, non_seat_neighbours)
        }
        _ => false,
    }
}

fn swap_seatings(seating: &Seating, non_seat_neighbours: bool, max_neighbours: usize) -> Seating {
    let mut seated = 0;
    let mut settled = true;
    let map = seating
        .map
        .iter()
        .map(|((row_nr, seat_nr), seat_status)| {
            let count_neighbours = || {
                DIRECTIONS
                    .iter()
                    .filter(|direction| {
                        direction_has_neighbour(
                            &seating,
                            (&seat_nr, &row_nr),
                            direction,
                            non_seat_neighbours,
                        )
                    })
                    .count()
            };
            let new_seat_status = match seat_status {
                SeatStatus::Occupied if count_neighbours() >= max_neighbours => SeatStatus::Free,
                SeatStatus::Free if count_neighbours() == 0 => SeatStatus::Occupied,
                status => *status,
            };
            //Unclean reach out
            if &new_seat_status != seat_status {
                settled = false
            };
            if new_seat_status == SeatStatus::Occupied {
                seated += 1
            }
            ((*row_nr, *seat_nr), new_seat_status)
        })
        .collect();
    Seating {
        map,
        seated,
        settled,
    }
}

fn swap_seatings_til_settled<'a>(
    seating: &'a Seating,
    non_seat_neighbours: bool,
    max_neighbours: usize,
) -> Seating {
    let result = swap_seatings(seating, non_seat_neighbours, max_neighbours);
    match result.settled {
        true => result,
        false => swap_seatings_til_settled(&result, non_seat_neighbours, max_neighbours),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_seating, swap_seatings_til_settled};

    static INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_one_star_result() {
        let seating = parse_seating(INPUT);
        assert_eq!(37, swap_seatings_til_settled(&seating, true, 4).seated);
    }

    #[test]
    fn test_two_star_result() {
        let seating = parse_seating(INPUT);
        assert_eq!(26, swap_seatings_til_settled(&seating, false, 5).seated);
    }
}
