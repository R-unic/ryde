use std::fmt;

#[derive(Debug)]
pub enum VmError {
    RegisterOutOfBounds(String),
    ProgramCounterOutOfBounds,
    CallStackEmpty,
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
    VariableNotFound(String),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::RegisterOutOfBounds(msg) => write!(f, "Register error: {}", msg),
            VmError::ProgramCounterOutOfBounds => write!(f, "Program counter out of bounds"),
            VmError::CallStackEmpty => write!(f, "Call stack is empty, cannot return"),
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
            VmError::VariableNotFound(name) => write!(f, "Variable '{}' not found", name),
        }
    }
}
