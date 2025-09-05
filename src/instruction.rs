use std::fmt;

use bincode::{Decode, Encode};

use crate::value::VmValue;

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum Instruction {
    // /// Load a constant value from the constant pool into a register
    // LOADC {
    //     target: usize,
    //     constant_index: usize,
    // },
    /// Load a constant value directly into a register
    LOADV { target: usize, value: VmValue },

    /// target = a + b
    ADD { target: usize, a: usize, b: usize },
    /// target = a - b
    SUB { target: usize, a: usize, b: usize },
    /// target = a * b
    MUL { target: usize, a: usize, b: usize },
    /// target = a / b
    DIV { target: usize, a: usize, b: usize },
    /// target = a // b
    IDIV { target: usize, a: usize, b: usize },
    /// target = a ^ b
    POW { target: usize, a: usize, b: usize },
    /// target = a % b
    MOD { target: usize, a: usize, b: usize },
    /// target = -operand
    NEGATE { target: usize, operand: usize },

    /// target = a && b
    AND { target: usize, a: usize, b: usize },
    /// target = a || b
    OR { target: usize, a: usize, b: usize },
    /// target = a == b
    EQ { target: usize, a: usize, b: usize },
    /// target = a != b
    NEQ { target: usize, a: usize, b: usize },
    /// target = a < b
    LT { target: usize, a: usize, b: usize },
    /// target = a <= b
    LTE { target: usize, a: usize, b: usize },
    /// target = a > b
    GT { target: usize, a: usize, b: usize },
    /// target = a >= b
    GTE { target: usize, a: usize, b: usize },
    /// target = !operand
    NOT { target: usize, operand: usize },
    /// Increment variable `name` by 1, and store the result in `target` if specified
    INC { target: Option<usize>, name: String },
    /// Decrement variable `name` by 1, and store the result in `target` if specified
    DEC { target: Option<usize>, name: String },
    /// target = object[index] -- register index
    INDEX {
        target: usize,
        object: usize,
        index: usize, // register
    },
    /// target = object[index] -- constant index
    INDEXK {
        target: usize,
        object: usize,
        index: usize, // constant
    },
    /// object[index] = source -- register index
    STOREINDEX {
        source: usize,
        object: usize,
        index: usize, // register
    },
    /// object[index] = source -- constant index
    STOREINDEXK {
        source: usize,
        object: usize,
        index: usize, // constant
    },

    /// Jump to instruction at the specified address
    JMP(usize),
    /// Jump to instruction at `address` if the value of `source` is not truthy
    JZ { source: usize, address: usize },
    /// Jump to instruction at `address` if the value of `source` is truthy
    JNZ { source: usize, address: usize },

    /// Store the value of register `target` into variable `name`
    STORE { source: usize, name: String },
    /// Load the value of variable `name` into register `target`
    LOAD { target: usize, name: String },
    /// Call a subroutine at the specified instruction
    CALL(usize),
    /// Return from a subroutine
    RETURN,

    /// Prints the value of the specified register into stdout
    PRINT(usize),
    /// Stops execution
    HALT,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{:?}", self).split(" ").next().unwrap().trim()
        )
    }
}
