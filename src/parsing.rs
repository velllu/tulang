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
    UnknownCommand,
}

fn get_char(instruction_split: Vec<&str>, argument_number: usize) -> Result<char, ParseError> {
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

pub fn parse_instruction(instruction_split: Vec<&str>) -> Result<Instruction, ParseError> {
    let first = *match instruction_split.first() {
        Some(first) => first,
        None => return Err(ParseError::EmptyLine), // TODO: Implement comments
    };

    Ok(match first {
        "move_to_char_right" => Instruction::MoveToCharRight(get_char(instruction_split, 1)?),
        "move_to_char_left" => Instruction::MoveToCharLeft(get_char(instruction_split, 1)?),

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

    for line in file_contents.lines() {
        let split: Vec<&str> = line.split(", ").collect();
        let instruction = parse_instruction(split)?;

        instructions.push(instruction);
    }

    Ok(instructions)
}
