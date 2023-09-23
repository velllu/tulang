use std::fmt::Display;

pub enum Instruction {
    Alphabet(Vec<char>),
    MoveToCharRight(char),
    MoveToCharLeft(char),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MoveToCharRight(char) => write!(f, "move to char {} on the right", char),
            Self::MoveToCharLeft(char) => write!(f, "move to char {} on the left", char),
            Self::Alphabet(chars) => write!(f, "alphabet: {}", chars.iter().collect::<String>()),
        }
    }
}
