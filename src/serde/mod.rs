use crate::{instruction::Instruction, value::VmValue};
use bincode::{
    Decode, Encode,
    config::{self, Configuration},
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
}
