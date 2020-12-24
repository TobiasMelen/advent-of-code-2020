use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::utils::read_file;

type BagData<'a> = HashMap<&'a str, HashMap<&'a str, i32>>;
pub fn main() {
    let input = read_file("day7.txt").unwrap();
    let one_star = get_possible_wrappers_for_color("shiny gold", &extract_bag_data(&input, true));
    println!(
        "Shiny gold bags can be contained in {} other bags for one star.",
        one_star.len()
    );
    let two_star = count_color_bag_capacity("shiny gold", &extract_bag_data(&input, false));
    println!(
        "Shiny gold bags can contain a total of {} other bags for two stars.",
        two_star
    );
}

fn count_color_bag_capacity(color: &str, bag_data: &BagData) -> i32 {
    bag_data.get(color).map_or(1, |contains| {
        contains
            .iter()
            .map(|(color, quantity)| {
                quantity + quantity * count_color_bag_capacity(color, bag_data)
            })
            .sum()
    })
}

fn get_possible_wrappers_for_color(color: &str, bag_data: &BagData) -> HashSet<String> {
    //bruh, getting an recursive iterator to work in rust.
    //mutable recursion here we go
    fn populate_color_containers<'a>(
        color: &str,
        bag_data: &'a BagData,
        target: &mut HashSet<String>,
    ) {
        bag_data.get(color).map(|v| {
            v.keys().for_each(|color| {
                target.insert(String::from(*color));
                populate_color_containers(color, bag_data, target);
            });
        });
    }
    let mut result = HashSet::new();
    populate_color_containers(color, &bag_data, &mut result);
    result
}

// I got a little bit frustaded that the two star didn't hit my abstraction strategy at all,
// So i just made this function take the behaviour bool flag "invert" to make life easier for myself
// Ugly and inefficient since we'll be calling this deserializing twice, fight me. Lesson: never ever design for "upcoming" requirements.
fn extract_bag_data(bag_descriptor: &str, invert: bool) -> BagData {
    let container_regex = Regex::new(r"(\w* \w*) bags").unwrap();
    let content_regex = Regex::new(r"(\d*) (\w* \w*) bag").unwrap();
    let mut map = HashMap::new();
    for line in bag_descriptor.lines() {
        let mut line_parts = line.split("contain");
        let (bag_string, content_string) = match (line_parts.next(), line_parts.next()) {
            (Some(bag), Some(contents)) => (bag, contents),
            _ => continue,
        };
        let container_bag_extraction = container_regex
            .captures(bag_string)
            .and_then(|groups| groups.get(1).map(|s| s.as_str()));

        let bag_name = match container_bag_extraction {
            Some(bag_name) => bag_name,
            _ => continue,
        };
        let bag_data = content_string.split(',').filter_map(|s| {
            content_regex.captures(s).and_then(|captures| {
                let digit = captures.get(1).and_then(|c| c.as_str().parse::<i32>().ok());
                let name = captures.get(2).map(|c| c.as_str());
                match (name, digit) {
                    (Some(name), Some(digit)) => Some((name, digit)),
                    _ => None,
                }
            })
        });
        if invert {
            for (name, count) in bag_data {
                map.entry(name)
                    .or_insert(HashMap::new())
                    .insert(bag_name, count);
            }
        } else {
            map.insert(bag_name, bag_data.collect());
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::{count_color_bag_capacity, extract_bag_data, get_possible_wrappers_for_color};
    #[test]
    fn test_for_one_star() {
        let input = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
        let one_star =
            get_possible_wrappers_for_color("shiny gold", &extract_bag_data(input, true));
        assert_eq!(4, one_star.len());
    }

    #[test]
    fn test_for_two_star() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let two_star = count_color_bag_capacity("shiny gold", &extract_bag_data(input, false));
        assert_eq!(126, two_star);
    }
}
