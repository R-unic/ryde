use bincode::{Decode, Encode};
use core::fmt;
use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt::Formatter,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::{array::DynamicArray, error::vm::VmError, object::Object};

pub type SharedValue = Rc<RefCell<VmValue>>;

#[derive(Encode, Decode, Debug, Clone)]
pub enum VmValue {
    Float(f64),
    Int(i32),
    String(String),
    Boolean(bool),
    DynamicArray(DynamicArray),
    Object(Object),
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

    pub fn as_array_mut(&mut self) -> Result<&mut DynamicArray, VmError> {
        match self {
            VmValue::DynamicArray(v) => Ok(v),
            default => Err(VmError::AttemptToIndex(format!("{:?}", default))),
        }
    }

    pub fn as_array(&self) -> Result<&DynamicArray, VmError> {
        match self {
            VmValue::DynamicArray(v) => Ok(v),
            default => Err(VmError::AttemptToIndex(format!("{:?}", default))),
        }
    }

    pub fn as_object_mut(&mut self) -> Result<&mut Object, VmError> {
        match self {
            VmValue::Object(v) => Ok(v),
            default => Err(VmError::AttemptToIndex(format!("{:?}", default))),
        }
    }

    pub fn as_object(&self) -> Result<&Object, VmError> {
        match self {
            VmValue::Object(v) => Ok(v),
            default => Err(VmError::AttemptToIndex(format!("{:?}", default))),
        }
    }

    fn inspect(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
        match self {
            VmValue::Float(v) => write!(f, "{}", v),
            VmValue::Int(v) => write!(f, "{}", v),
            VmValue::Boolean(v) => write!(f, "{}", v),
            VmValue::String(bytes) => write!(f, "\"{}\"", bytes),
            VmValue::DynamicArray(arr) => {
                write!(f, "[")?;

                let length = arr.0.borrow().len();
                let is_long = length >= 3;
                if is_long {
                    write!(f, "\n")?;
                    write_tab(f, indent)?;
                }
                for (i, value) in arr.0.borrow().iter().enumerate() {
                    if is_long {
                        value.inspect(f, indent + 1)?;
                    } else {
                        write!(f, "{}", value)?;
                    }
                    if i < length - 1 {
                        write!(f, ", ")?;
                        if is_long {
                            write!(f, "\n")?;
                            write_tab(f, indent)?;
                        }
                    }
                }
                if is_long {
                    write!(f, "\n")?;
                    write_tab(f, indent - 1)?;
                }
                write!(f, "]")
            }
            VmValue::Object(object) => {
                write!(f, "{{")?;

                let length = object.0.borrow().len();
                let is_long = length >= 3;
                if is_long {
                    write!(f, "\n")?;
                    write_tab(f, indent)?;
                } else if length > 0 {
                    write!(f, " ")?;
                }

                for (i, (key, value)) in object.0.borrow().iter().enumerate() {
                    write!(f, "[{}]: ", key)?;
                    if is_long {
                        value.inspect(f, indent + 1)?;
                    } else {
                        write!(f, "{}", value)?;
                    }

                    if i < length - 1 {
                        write!(f, ", ")?;
                        if is_long {
                            write!(f, "\n")?;
                            write_tab(f, indent)?;
                        }
                    }
                }

                if is_long {
                    write!(f, "\n")?;
                    write_tab(f, indent - 1)?;
                } else if length > 0 {
                    write!(f, " ")?;
                }
                write!(f, "}}")
            }
            VmValue::Null => write!(f, "null"),
        }
    }
}

impl Hash for VmValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            VmValue::Float(f) => f.to_bits().hash(state), // hash the raw bits
            VmValue::Int(i) => i.hash(state),
            VmValue::String(s) => s.hash(state),
            VmValue::Boolean(b) => b.hash(state),
            VmValue::DynamicArray(arr) => arr.hash(state),
            VmValue::Object(obj) => obj.hash(state),
            VmValue::Null => ().hash(state),
        }
    }
}

const TAB: &str = "  ";
fn write_tab(f: &mut Formatter<'_>, indent: usize) -> Result<(), std::fmt::Error> {
    write!(f, "{}", TAB.repeat(indent))
}

impl fmt::Display for VmValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inspect(f, 1)
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
impl Ord for VmValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
