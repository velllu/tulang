use std::{env::current_exe, fmt::Display, path::Path};

use crate::{
    opcodes::Instruction,
    parsing::{parse_file, ParseError},
};

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct State {
    direction: Direction,
    current_state: u32,
    next_state: u32,
    replace: (char, char),
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{},{},{},{})",
            self.current_state,
            self.replace.0,
            self.next_state,
            self.replace.1,
            match self.direction {
                Direction::Left => "<",
                Direction::Right => ">",
            }
        )
    }
}

pub struct TuringMachine {
    instructions: Vec<Instruction>,
    alphabet: Vec<char>,
}

impl TuringMachine {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ParseError> {
        let instructions = parse_file(path)?;

        // The program 100% has an alphabet instruction as the first instruction because
        // it will return an error in `parsing.rs` if it didn't have one
        let alphabet = instructions.get(0).unwrap();
        let alphabet = match alphabet {
            Instruction::Alphabet(alphabet) => alphabet,
            _ => unreachable!(),
        };

        Ok(Self {
            alphabet: alphabet.to_vec(),
            instructions,
        })
    }

    pub fn print_instructions(&self) {
        for instruction in self.instructions.iter() {
            println!("{}", instruction);
        }
    }

    fn calculate_move_states(
        &self,
        excluded_char: char,
        replaces: &Vec<(char, char)>,
        direction: &Direction,
        current_state: &mut u32,
    ) -> Vec<State> {
        let alphabet_without_char: Vec<char> = self
            .alphabet
            .iter()
            .filter(|x| **x != excluded_char) // We will add this back later
            .cloned()
            .collect();

        let mut states: Vec<State> = Vec::new();
        for character in alphabet_without_char.iter() {
            if replaces
                .iter()
                .any(|(replace_from, _)| replace_from == character)
            {
                // We skip if character needs to be replaced, we will add that later
                continue;
            }

            states.push(State {
                direction: *direction,
                current_state: *current_state,
                next_state: *current_state,
                replace: (*character, *character),
            })
        }

        // We now add all the the replaces
        for (replace_from, replace_to) in replaces {
            states.push(State {
                direction: *direction,
                current_state: *current_state,
                next_state: *current_state,
                replace: (*replace_from, *replace_to),
            })
        }

        // We also have to add a state for the excluded character, where we make it go
        // to the next state
        states.push(State {
            direction: *direction,
            current_state: *current_state,
            next_state: current_state.wrapping_add(1),
            replace: (excluded_char, excluded_char),
        });

        // We also have to account for the "blank" space that is not in the alphabet
        if excluded_char != '-' {
            states.push(State {
                direction: *direction,
                current_state: *current_state,
                next_state: *current_state,
                replace: ('-', '-'),
            });
        }

        *current_state += 1;

        states
    }

    pub fn calculate_states(&self) -> Vec<State> {
        // Now we actually do the calculating
        let mut states: Vec<State> = Vec::new();
        let mut current_state: u32 = 0;

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::MoveToCharLeft(char, replaces) => {
                    states.append(&mut self.calculate_move_states(
                        *char,
                        replaces,
                        &Direction::Left,
                        &mut current_state,
                    ))
                }

                Instruction::MoveToCharRight(char, replaces) => {
                    states.append(&mut self.calculate_move_states(
                        *char,
                        replaces,
                        &Direction::Right,
                        &mut current_state,
                    ))
                }

                _ => {}
            }
        }

        states
    }
}
