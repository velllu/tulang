use std::fmt::Display;

pub enum Instruction {
    Alphabet(Vec<char>),
    MoveToCharRight(char, Vec<(char, char)>),
    MoveToCharLeft(char, Vec<(char, char)>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: make code DRYer
        match self {
            Self::MoveToCharRight(char, replaces) => {
                write!(f, "move to char {} on the right", char)?;

                for replace in replaces {
                    write!(f, ", replacing {} with {}", replace.0, replace.1)?;
                }

                Ok(())
            }

            Self::MoveToCharLeft(char, replaces) => {
                write!(f, "move to char {} on the left", char)?;

                for replace in replaces {
                    write!(f, ", replacing {} with {}", replace.0, replace.1)?;
                }

                Ok(())
            }

            Self::Alphabet(chars) => write!(f, "alphabet: {}", chars.iter().collect::<String>()),
        }
    }
}
