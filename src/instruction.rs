use std::fmt;

use bincode::{Decode, Encode};

use crate::value::VmValue;

#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
pub enum Instruction {
    // /// Load a constant value from the constant pool into a register
    // LOADC {
    //     target: usize,
    //     constant_index: usize,
    // },
    /// Load a constant value directly into a register
    LOADV {
        target: usize,
        value: VmValue,
    },

    /// target = a + b
    ADD {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a - b
    SUB {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a * b
    MUL {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a / b
    DIV {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a // b
    IDIV {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a ^ b
    POW {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a % b
    MOD {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = -operand
    NEGATE {
        target: usize,
        operand: usize,
    },

    /// target = a && b
    AND {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a || b
    OR {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a == b
    EQ {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a != b
    NEQ {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a < b
    LT {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a <= b
    LTE {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a > b
    GT {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = a >= b
    GTE {
        target: usize,
        a: usize,
        b: usize,
    },
    /// target = !operand
    NOT {
        target: usize,
        operand: usize,
    },

    PRINT(usize),
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
