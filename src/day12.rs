use std::ops::{AddAssign, Mul};

use crate::utils::read_file_lines;

pub fn main() {
    let input = read_file_lines("day12.txt").unwrap();
    let one_star = move_ship(&input);
    println!("Manhattan distance for one star is {}", one_star);
    let two_star = move_ship_waypoint(&input);
    println!("Manhattan distance for two star is {}", two_star);
}

#[derive(Clone, Copy)]
struct Point(isize, isize);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Point(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl Mul<isize> for Point {
    type Output = Point;
    fn mul(self, rhs: isize) -> Self::Output {
        return Point(self.0 * rhs, self.1 * rhs);
    }
}

fn move_ship<S: AsRef<str>>(sequence: &[S]) -> isize {
    //N/E/S/W
    let directions = &[Point(-1, 0), Point(0, 1), Point(1, 0), Point(0, -1)];
    let mut direction_index = 1;
    let mut position = Point(0, 0);
    for instruction in sequence {
        let mut iter = instruction.as_ref().chars();
        let descriptor = iter.nth(0).unwrap_or(' ');
        let amount = iter.collect::<String>().parse().unwrap_or(0);
        match descriptor {
            'N' => position += directions[0] * amount,
            'E' => position += directions[1] * amount,
            'S' => position += directions[2] * amount,
            'W' => position += directions[3] * amount,
            'R' => direction_index = (direction_index + (amount / 90)) % 4,
            'L' => direction_index = (direction_index + 4 - (amount / 90)) % 4,
            'F' => {
                position += directions[direction_index as usize] * amount;
            }
            _ => {}
        }
    }
    position.0.abs() + position.1.abs()
}

fn move_ship_waypoint<S: AsRef<str>>(sequence: &[S]) -> isize {
    //N/E/S/W
    let directions = &[Point(-1, 0), Point(0, 1), Point(1, 0), Point(0, -1)];
    let mut ship_pos = Point(0, 0);
    let mut waypoint_pos = Point(-1, 10);
    for instruction in sequence {
        let mut iter = instruction.as_ref().chars();
        let descriptor = iter.nth(0).unwrap_or(' ');
        let amount = iter.collect::<String>().parse().unwrap_or(0);
        let mut rotate = |amount: isize| {
            let radians = (amount as f64).to_radians();
            let (y, x) = (waypoint_pos.0 as f64, waypoint_pos.1 as f64);
            waypoint_pos = Point(
                (radians.sin() * x + radians.cos() * y).round() as isize,
                (radians.cos() * x - radians.sin() * y).round() as isize,
            )
        };
        match descriptor {
            'N' => waypoint_pos += directions[0] * amount,
            'E' => waypoint_pos += directions[1] * amount,
            'S' => waypoint_pos += directions[2] * amount,
            'W' => waypoint_pos += directions[3] * amount,
            'R' => rotate(amount),
            'L' => rotate(-amount),
            'F' => {
                ship_pos += waypoint_pos * amount;
                //waypoint_pos += waypoint_pos * amount;
            }
            _ => {}
        }
    }
    ship_pos.0.abs() + ship_pos.1.abs()
}

#[cfg(test)]
mod tests {
    use super::{move_ship, move_ship_waypoint};
    static INPUT: &[&str] = &["F10", "N3", "F7", "R90", "F11"];

    #[test]
    fn test_for_one_star() {
        let one_star = move_ship(INPUT);
        assert_eq!(25, one_star);
    }

    #[test]
    fn test_for_two_star() {
        let two_star = move_ship_waypoint(INPUT);
        assert_eq!(286, two_star);
    }
}
