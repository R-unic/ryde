use core::fmt;
use std::cmp::Ordering;

use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone)]
pub enum VmValue {
    Float(f64),
    Int(i32),
    String(Vec<u8>),
    Boolean(bool),
    Array(Box<Vec<VmValue>>),
    Null,
}

impl VmValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            VmValue::Boolean(v) => *v,
            VmValue::Null => false,
            _ => true,
        }
    }
}

impl fmt::Display for VmValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmValue::Float(v) => write!(f, "{}", v),
            VmValue::Int(v) => write!(f, "{}", v),
            VmValue::Boolean(v) => write!(f, "{}", v),
            VmValue::String(bytes) => write!(
                f,
                "{}",
                String::from_utf8(bytes.to_vec()).expect("failed to convert bytes into string")
            ),
            VmValue::Array(v) => {
                write!(f, "[")?;
                for (i, value) in v.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i < v.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            VmValue::Null => write!(f, "null"),
        }
    }
}

impl PartialOrd for VmValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (VmValue::Int(a), VmValue::Int(b)) => a.partial_cmp(b),
            (VmValue::Float(a), VmValue::Float(b)) => a.partial_cmp(b),
            (VmValue::Int(a), VmValue::Float(b)) => (*a as f64).partial_cmp(b),
            (VmValue::Float(a), VmValue::Int(b)) => a.partial_cmp(&(*b as f64)),
            (VmValue::String(a), VmValue::String(b)) => a.partial_cmp(b),
            _ => None, // incomparable
        }
    }
}

impl PartialEq for VmValue {
    /// Comparison for VmValues
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VmValue::Int(a), VmValue::Int(b)) => *a == *b,
            (VmValue::Float(a), VmValue::Float(b)) => *a == *b,
            (VmValue::Int(a), VmValue::Float(b)) => (*a as f64) == *b,
            (VmValue::Float(a), VmValue::Int(b)) => *a == (*b as f64),
            (VmValue::Boolean(a), VmValue::Boolean(b)) => *a == *b,
            (VmValue::String(a), VmValue::String(b)) => *a == *b,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for VmValue {}
