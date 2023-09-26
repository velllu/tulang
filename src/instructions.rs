use std::fmt::Display;

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
    /// - `move_to_char_right, -` moves to the nearest blank on the right
    /// - `move_to_char_right, -, b -> a` moves to the nearest blank on the right and
    /// replaces all Bs with As
    /// - `move_to_char_right, d, b -> a, c -> a` moves to the nearest blank on the right
    /// and replaces all Bs with As and Cs with As
    ///
    /// # Usage in this codebase
    /// `char` is the character to move to, `Vec<(char, char)>` is the list of characters
    /// to replaces and to what replace to
    MoveToCharRight(char, Vec<(char, char)>),

    /// See `MoveToCharLeft` for instructions, this is the same but going left instead
    /// of right
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
