use crate::{instruction::Instruction, value::VmValue};
use bincode::{
    Decode, Encode,
    config::{self, Configuration},
    error::DecodeError,
};

pub mod deserializer;
pub mod serializer;

const CONFIG: Configuration = config::standard();
const CURRENT_VERSION: u8 = 0;

#[repr(C)]
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Program {
    pub version: u8,
    pub constant_pool: Vec<VmValue>,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum ProgramError {
    FileError(std::io::Error),
    DecodeError(DecodeError),
}

impl Program {
    pub fn new(instructions: Vec<Instruction>, constant_pool: Vec<VmValue>) -> Self {
        Self {
            version: CURRENT_VERSION,
            instructions,
            constant_pool,
        }
    }

    pub fn from_instructions(instructions: Vec<Instruction>) -> Self {
        Self {
            version: CURRENT_VERSION,
            instructions,
            constant_pool: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> Result<Program, ProgramError> {
        let binary = std::fs::read(path).map_err(ProgramError::FileError)?;
        deserializer::deserialize(binary).map_err(ProgramError::DecodeError)
    }
}
