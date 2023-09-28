use std::fmt::Display;

use crate::calculation::Direction;

pub enum Instruction {
    /// # Usage in scripting
    /// Every file needs to only have one of this, and it must be the first statement,
    /// examples usage is `alphabet, abcde`, where the second argument is all the chars
    /// you want to use in the turing machine. This is needed to calculate all the states
    /// when moving between char to char for example without having the end user type out
    /// all the characters everytime
    ///
    /// # Usage in this codebase
    /// `Vec<char>` is the list of characters of the alphabet
    Alphabet(Vec<char>),

    /// # Usage in scripting
    /// # Examples
    /// - `move_to_char, left, -` moves to the nearest blank on the left
    /// - `move_to_char, left, -, b -> a` moves to the nearest blank on the left and
    /// replaces all Bs with As
    /// - `move_to_char, right, d, b -> a, c -> a` moves to the nearest blank on the
    /// right and replaces all Bs with As and Cs with As
    ///
    /// # Usage in this codebase
    /// `char` is the character to move to, `Vec<(char, char)>` is the list of characters
    /// to replaces and to what replace to
    MoveToChar(Direction, char, Vec<(char, char)>),

    BeginLoop,
    EndLoop(Direction, Vec<(char, char)>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: make code DRYer
        match self {
            Self::MoveToChar(direction, char, replaces) => {
                write!(f, "move to char {} on the {}", char, direction)?;

                for replace in replaces {
                    write!(f, ", replacing {} with {}", replace.0, replace.1)?;
                }

                Ok(())
            }

            Self::BeginLoop => write!(f, "starting loop"),
            Self::EndLoop(direction, _) => write!(f, "ending loop and going to {}", direction),
            Self::Alphabet(chars) => write!(f, "alphabet: {}", chars.iter().collect::<String>()),
        }
    }
}
