use std::{error::Error, fmt};

#[derive(Debug)]
pub enum VmError {
    RegisterOutOfBounds(usize),
    VariableNotFound(String),
    ProgramCounterOutOfBounds,
    CallStackEmpty,
    AttemptToIndex(String),
    InvalidIndexType(String),
    OperandTypeMismatch {
        expected: String,
        actual: String,
    },
    BinaryTypeMismatch {
        opcode_name: String,
        expected: String,
        a_actual: String,
        b_actual: String,
    },
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::RegisterOutOfBounds(index) => write!(f, "Invalid register index: {}", index),
            VmError::VariableNotFound(name) => {
                write!(f, "Variable '{}' not found in local scope", name)
            }
            VmError::ProgramCounterOutOfBounds => write!(f, "Program counter out of bounds"),
            VmError::CallStackEmpty => write!(f, "Call stack is empty, cannot return"),
            VmError::AttemptToIndex(actual) => write!(f, "Attempt to index '{}'", actual),
            VmError::InvalidIndexType(actual) => write!(f, "Invalid index type, got '{}'", actual),
            VmError::OperandTypeMismatch { expected, actual } => {
                write!(f, "Expected type '{}', got '{}'", expected, actual)
            }
            VmError::BinaryTypeMismatch {
                opcode_name,
                expected,
                a_actual,
                b_actual,
            } => {
                write!(
                    f,
                    "Expected type '{}' for operands of '{}' operation, got '{}' {} '{}'",
                    expected, opcode_name, a_actual, opcode_name, b_actual
                )
            }
        }
    }
}

impl Error for VmError {}
