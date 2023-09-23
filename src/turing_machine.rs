use std::path::Path;

use crate::{
    opcodes::Instruction,
    parsing::{parse_file, ParseError},
};

pub struct TuringMachine {
    instructions: Vec<Instruction>,
}

impl TuringMachine {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ParseError> {
        let instructions = parse_file(path)?;
        Ok(Self { instructions })
    }

    pub fn print_instructions(&self) {
        for instruction in self.instructions.iter() {
            println!("{}", instruction);
        }
    }
}
