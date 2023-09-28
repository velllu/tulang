use std::{fmt::Display, path::Path};

use crate::{
    instructions::Instruction,
    parsing::{parse_file, ParseError},
};

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub struct State {
    current_state: u32,
    next_state: u32,

    /// Direction to go after replacing
    direction: Direction,

    /// First char is what we replace from and the second one is what we replace to
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

    /// The list from the `alphabet` instruction, this is needed to correctly calculate
    /// moving from one point to another
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
            direction: direction.opposite(),
            current_state: *current_state,
            next_state: current_state.wrapping_add(1),
            replace: (excluded_char, excluded_char),
        });

        // If the excluded char is not a blank, we need to add a state handling it
        // because blank is not contained in the alphabet
        // If the excluded char is a blank, we can skip this because we have already
        // added the needed state just above this comment block
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

    fn calculate_end_loop_states(
        &self,
        direction: &Direction,
        replaces: &Vec<(char, char)>,
        begin_loop_state: u32,
        current_state: &mut u32,
    ) -> Vec<State> {
        let mut states: Vec<State> = Vec::new();

        for character in self.alphabet.iter() {
            if replaces
                .iter()
                .any(|(replace_from, _)| replace_from == character)
            {
                continue;
            }

            states.push(State {
                current_state: *current_state,
                next_state: begin_loop_state,
                direction: *direction,
                replace: (*character, *character),
            });
        }

        for (replace_from, replace_to) in replaces {
            states.push(State {
                current_state: *current_state,
                next_state: begin_loop_state,
                direction: *direction,
                replace: (*replace_from, *replace_to),
            });
        }

        states
    }

    pub fn calculate_states(&self) -> Vec<State> {
        // Now we actually do the calculating
        let mut states: Vec<State> = Vec::new();
        let mut begin_loop_state: Option<u32> = None;
        let mut current_state: u32 = 0;

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::MoveToChar(direction, char, replaces) => states.append(
                    &mut self.calculate_move_states(*char, replaces, direction, &mut current_state),
                ),

                Instruction::BeginLoop => begin_loop_state = Some(current_state),

                Instruction::EndLoop(direction, replaces) => {
                    // unwrapping `begin_loop_state` is safe because we already checked
                    // in `parsing.rs` for wrong loop order
                    states.append(&mut self.calculate_end_loop_states(
                        direction,
                        replaces,
                        begin_loop_state.unwrap(),
                        &mut current_state,
                    ));

                    begin_loop_state = None;
                }

                Instruction::Alphabet(_) => {}
            }
        }

        states
    }
}
