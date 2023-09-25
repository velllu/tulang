use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::opcodes::Instruction;

#[derive(Debug)]
pub enum ParseError {
    CouldNotOpenFile,
    CouldNotReadFile,
    EmptyLine,
    MissingParameter,
    OnlyOneLetterAllowed,
    OnlyOneAlphabetAllowed,
    NoAlphabet,
    CouldNotParseReplace,
    CouldNotParseCharacter,
    EmptyFile,
    UnknownCommand,
}

fn get_char(instruction_split: &Vec<&str>, argument_number: usize) -> Result<char, ParseError> {
    let element = match instruction_split.get(argument_number) {
        Some(element) => element,
        None => return Err(ParseError::MissingParameter),
    };

    if element.len() != 1 {
        return Err(ParseError::OnlyOneLetterAllowed);
    }

    // This should never fail because we just checked if `element` is composed by only
    // one char
    Ok(element.chars().next().unwrap())
}

fn get_replaces(instruction_split: &Vec<&str>) -> Result<Vec<(char, char)>, ParseError> {
    // We need to remove the first two elements because they will be both
    // `move_to_char_right/left` and the actual character, we just need to focust on the
    // part after
    let (_, instruction_split) = instruction_split.split_at(2);

    let mut replaces: Vec<(char, char)> = Vec::new();

    // The syntax looks like this: `a -> d, b -> c`
    for replace in instruction_split {
        // We need to further split the ` -> `
        let (replace_from, replace_to) = match replace.split_once(" -> ") {
            Some((replace_from, replace_to)) => (replace_from, replace_to),
            None => return Err(ParseError::CouldNotParseReplace),
        };

        // ... and then we need to convert it to chars
        if replace_from.len() != 1 || replace_to.len() != 1 {
            return Err(ParseError::CouldNotParseCharacter);
        }

        // (this is safe because we just checked)
        let replace_from = replace_from.chars().next().unwrap();
        let replace_to = replace_to.chars().next().unwrap();

        replaces.push((replace_from, replace_to));
    }

    Ok(replaces)
}

pub fn parse_instruction(instruction_split: Vec<&str>) -> Result<Instruction, ParseError> {
    let first = *match instruction_split.first() {
        Some(first) => first,
        None => return Err(ParseError::EmptyLine), // TODO: Implement comments
    };

    Ok(match first {
        "move_to_char_right" => Instruction::MoveToCharRight(
            get_char(&instruction_split, 1)?,
            get_replaces(&instruction_split)?,
        ),

        "move_to_char_left" => Instruction::MoveToCharLeft(
            get_char(&instruction_split, 1)?,
            get_replaces(&instruction_split)?,
        ),

        "alphabet" => {
            let string = instruction_split.get(1);
            let string = match string {
                Some(string) => string,
                None => return Err(ParseError::MissingParameter),
            };

            let mut chars: Vec<char> = Vec::new();
            for char in string.chars() {
                chars.push(char);
            }

            Instruction::Alphabet(chars)
        }

        _ => return Err(ParseError::UnknownCommand),
    })
}

pub fn parse_file(path: impl AsRef<Path>) -> Result<Vec<Instruction>, ParseError> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(ParseError::CouldNotOpenFile),
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(_) => (),
        Err(_) => return Err(ParseError::CouldNotReadFile),
    }

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut alphabet_already_exists = false;
    for line in file_contents.lines() {
        let split: Vec<&str> = line.split(", ").collect();
        let instruction = parse_instruction(split)?;

        // Only one alphabet istruction is allowed
        match instruction {
            Instruction::Alphabet(_) => {
                if alphabet_already_exists {
                    return Err(ParseError::OnlyOneAlphabetAllowed);
                }

                alphabet_already_exists = true;
            }

            _ => {}
        }

        instructions.push(instruction);
    }

    if let Some(first_instruction) = instructions.get(0) {
        // The alphabet instruction must be the first instruction
        match first_instruction {
            Instruction::Alphabet(_) => {}
            _ => return Err(ParseError::NoAlphabet),
        }
    } else {
        // and the file also cannot be empty
        return Err(ParseError::EmptyFile);
    }

    // TODO: Verify that instructions contain *only* chars allowed in the alphabet

    Ok(instructions)
}
