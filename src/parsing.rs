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
    UnknownCommand(String),
}

pub fn parse_instruction(instruction_split: Vec<&str>) -> Result<Instruction, ParseError> {
    macro_rules! get {
        ($option:expr) => {
            if let Some(unwrapped) = $option {
                unwrapped
            } else {
                return Err(ParseError::MissingParameter);
            }
        };
    }

    let first = *match instruction_split.first() {
        Some(first) => first,
        None => return Err(ParseError::EmptyLine), // TODO: Implement comments
    };

    use Instruction as Ins;

    Ok(match first {
        "move_to_char_right" => Ins::MoveToCharRight(get!(instruction_split.get(1)).to_string()),
        "move_to_char_left" => Ins::MoveToCharLeft(get!(instruction_split.get(1)).to_string()),

        _ => return Err(ParseError::UnknownCommand(first.to_string())),
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

    for line in file_contents.lines() {
        let split: Vec<&str> = line.split(", ").collect();
        let instruction = parse_instruction(split)?;

        instructions.push(instruction);
    }

    Ok(instructions)
}
