use std::fmt::Display;

/// A character is just a letter from the alphabet
type Character = String;

pub enum Instruction {
    MoveToCharRight(Character),
    MoveToCharLeft(Character),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MoveToCharRight(char) => write!(f, "move to char {} on the right", char),
            Self::MoveToCharLeft(char) => write!(f, "move to char {} on the left", char),
        }
    }
}
