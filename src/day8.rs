use crate::utils::{read_file, SplitOnce};
use std::collections::HashSet;

type Instruction = (String, isize);

type Instructions = Vec<Instruction>;
struct Game<'a> {
    instructions: &'a Instructions,
    insanity_repair_mode: bool,
}

trait SetupGameState {
    fn setup(self, noop_and_jmp_replace_index: Option<usize>);
}

struct GameIteratorState {
    value: isize,
    next_index: isize,
    noop_and_jmp_index: usize,
    executed_indexes: HashSet<isize>,
}

impl GameIteratorState {
    fn new() -> GameIteratorState {
        GameIteratorState {
            noop_and_jmp_index: 0,
            value: 0,
            next_index: 0,
            executed_indexes: HashSet::new(),
        }
    }
}

struct GameIterator<'a> {
    instructions: &'a Instructions,
    noop_and_jmp_replace_index: Option<usize>,
    state: GameIteratorState,
}

impl<'a> Iterator for GameIterator<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state.executed_indexes.contains(&self.state.next_index) {
            match self.noop_and_jmp_replace_index {
                Some(value) => {
                    self.state = GameIteratorState::new();
                    self.noop_and_jmp_replace_index = Some(value + 1);
                }
                None => {
                    return None;
                }
            };
        };

        self.state.executed_indexes.insert(self.state.next_index);

        let (instruction, argument) = match self.instructions.get(self.state.next_index as usize) {
            Some(value) => value,
            None => {
                return None;
            }
        };

        let fixed_instruction = match instruction.as_ref() {
            value @ "nop" | value @ "jmp" if self.noop_and_jmp_replace_index.is_some() => {
                self.state.noop_and_jmp_index += 1;
                if self.state.noop_and_jmp_index == self.noop_and_jmp_replace_index.unwrap() {
                    match value {
                        "nop" => "jmp",
                        "jmp" => "nop",
                        val => val,
                    }
                } else {
                    value
                }
            }
            any => any,
        };

        match fixed_instruction {
            "acc" => {
                println!("Accumalating {}", argument);
                self.state.value += argument;
                self.state.next_index += 1;
            }
            "jmp" => {
                self.state.noop_and_jmp_index += 1;
                self.state.next_index += *argument;
            }
            "nop" => {
                self.state.noop_and_jmp_index += 1;
                self.state.next_index += 1;
            }
            _ => panic!(format!("Argument: {} is not recognized", instruction)),
        };

        Some(self.state.value)
    }
}

impl<'a> IntoIterator for Game<'a> {
    type Item = isize;

    type IntoIter = GameIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GameIterator {
            instructions: self.instructions,
            noop_and_jmp_replace_index: match self.insanity_repair_mode {
                true => Some(0),
                false => None,
            },
            state: GameIteratorState::new(),
        }
    }
}

pub fn main() {
    let input = read_file("day8.txt").unwrap();
    let instructions = parse_instruction_set(&input);
    let one_star = last_value_of_game(Game {
        instructions: &instructions,
        insanity_repair_mode: false,
    });
    println!("Result for one star is {}", one_star);
    let two_star = last_value_of_game(Game {
        instructions: &instructions,
        insanity_repair_mode: true,
    });
    println!("Result for two star is {}", two_star);
}

fn parse_instruction_set(input: &str) -> Instructions {
    input
        .lines()
        .filter_map(|line| {
            line.split_one_time(" ").and_then(|(op, arg)| {
                arg.parse()
                    .ok()
                    .map(|arg_number| (String::from(op), arg_number))
            })
        })
        .collect()
}

// fn find_end_of_working_mod(game: Game, fix_index: usize) -> isize {
//     let mut iter = game.into_iter();
// }

fn last_value_of_game(instructions: Game) -> isize {
    instructions.into_iter().last().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{last_value_of_game, parse_instruction_set, Game};

    static INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_for_one_star() {
        let one_star = last_value_of_game(Game {
            instructions: &parse_instruction_set(INPUT),
            insanity_repair_mode: false,
        });
        assert_eq!(one_star, 5)
    }

    #[test]
    fn test_for_two_star() {
        let one_star = last_value_of_game(Game {
            instructions: &parse_instruction_set(INPUT),
            insanity_repair_mode: true,
        });
        assert_eq!(one_star, 8)
    }
}
