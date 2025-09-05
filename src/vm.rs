use std::collections::HashMap;

use crate::error::vm::VmError;
use crate::instruction::Instruction;
use crate::serde::Program;
use crate::value::VmValue;

pub struct Vm<'a> {
    pub pc: usize,
    pub registers: Vec<VmValue>,
    pub program: &'a Program,
    pub variables: HashMap<String, VmValue>, // TODO: scoping
    pub call_stack: Vec<Frame>,
}

#[derive(Debug)]
pub struct Frame {
    return_address: usize,
}

impl Frame {
    pub fn new(return_address: usize) -> Self {
        Self { return_address }
    }
}

impl<'a> Vm<'a> {
    pub fn new(program: &'a Program, register_count: usize) -> Self {
        Self {
            pc: 0,
            registers: vec![VmValue::Null; register_count],
            program,
            variables: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        while self.pc < self.instruction_count() {
            let instruction = self.current_instruction().clone();
            self.pc += 1;
            self.execute_instruction(instruction)?;
        }
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub fn visualize_callstack(&self) -> String {
        if self.call_stack.is_empty() {
            "(empty call stack)".to_string()
        } else {
            let mut s = String::from("call stack:\n");
            for (i, frame) in self.call_stack.iter().rev().enumerate() {
                s.push_str(&format!(
                    "  frame {}: return address -> {}\n",
                    i, frame.return_address
                ));
            }
            s
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), VmError> {
        use Instruction::*;
        let opcode_name = format!("{}", instruction);
        match instruction {
            LOADV { target, value } => self.set_register(target, &value)?,
            ADD { target, a, b } => {
                self.arithmetic_binop(target, a, b, opcode_name, |a, b| a + b)?
            }
            SUB { target, a, b } => {
                self.arithmetic_binop(target, a, b, opcode_name, |a, b| a - b)?
            }
            MUL { target, a, b } => {
                self.arithmetic_binop(target, a, b, opcode_name, |a, b| a * b)?
            }
            DIV { target, a, b } => {
                self.arithmetic_binop(target, a, b, opcode_name, |a, b| a / b)?
            }
            IDIV { target, a, b } => {
                self.arithmetic_binop(target, a, b, opcode_name, |a, b| f64::floor(a / b))?
            }
            POW { target, a, b } => self.arithmetic_binop(target, a, b, opcode_name, |a, b| {
                if b.fract() == 0.0 {
                    f64::powi(a, b as i32)
                } else {
                    f64::powf(a, b)
                }
            })?,
            MOD { target, a, b } => {
                self.arithmetic_binop(target, a, b, opcode_name, |a, b| a % b)?
            }
            NEGATE { target, operand } => self.arithmetic_unop(target, operand, |v| -v)?,

            AND { target, a, b } => self.logical_binop(target, a, b, |a, b| a && b)?,
            OR { target, a, b } => self.logical_binop(target, a, b, |a, b| a || b)?,
            EQ { target, a, b } => self.comparison_binop(target, a, b, |a, b| a == b)?,
            NEQ { target, a, b } => self.comparison_binop(target, a, b, |a, b| a != b)?,
            LT { target, a, b } => self.comparison_binop(target, a, b, |a, b| a < b)?,
            LTE { target, a, b } => self.comparison_binop(target, a, b, |a, b| a <= b)?,
            GT { target, a, b } => self.comparison_binop(target, a, b, |a, b| a > b)?,
            GTE { target, a, b } => self.comparison_binop(target, a, b, |a, b| a >= b)?,
            NOT { target, operand } => self.logical_unop(target, operand, |v| !v)?,
            INC { target, name } => {
                let value = self.lookup_variable(&name);
                if let VmValue::Int(n) = value {
                    let value = VmValue::Int(n + 1);
                    self.set_variable(name, &value);
                    if let Some(target) = target {
                        self.set_register(target, &value)?;
                    }
                } else {
                    return Err(VmError::OperandTypeMismatch {
                        expected: "number".to_string(),
                        actual: format!("{:?}", value),
                    });
                }
            }
            DEC { target, name } => {
                let value = self.lookup_variable(&name);
                if let VmValue::Int(n) = value {
                    let value = VmValue::Int(n - 1);
                    self.set_variable(name, &value);
                    if let Some(target) = target {
                        self.set_register(target, &value)?;
                    }
                } else {
                    return Err(VmError::OperandTypeMismatch {
                        expected: "number".to_string(),
                        actual: format!("{:?}", value),
                    });
                }
            }
            INDEX {
                target,
                object,
                index,
            } => {
                let object_value = self.get_register(object)?;
                let index_value = self.get_register(index)?;
                if let VmValue::Array(arr) = object_value {
                    if let VmValue::Int(i) = index_value {
                        let value = arr[*i as usize].clone();
                        self.set_register(target, &value)?;
                    } else {
                        return Err(VmError::InvalidIndexType(format!("{:?}", index_value)));
                    }
                } else {
                    return Err(VmError::AttemptToIndex(format!("{:?}", object_value)));
                }
            }
            INDEXK {
                target,
                object,
                index,
            } => {
                let object_value = self.get_register(object)?;
                if let VmValue::Array(arr) = object_value {
                    let value = arr[index].clone();
                    self.set_register(target, &value)?;
                } else {
                    return Err(VmError::AttemptToIndex(format!("{:?}", object_value)));
                }
            }
            STOREINDEX {
                source,
                object,
                index,
            } => {
                let object_value = self.get_register(object)?;
                let index_value = self.get_register(index)?;
                let source_value = self.get_register(source)?;
                if let VmValue::Array(mut arr) = object_value.clone() {
                    if let VmValue::Int(i) = index_value {
                        arr[*i as usize] = source_value.clone();
                        self.set_register(object, &VmValue::Array(arr))?;
                    } else {
                        return Err(VmError::InvalidIndexType(format!("{:?}", index_value)));
                    }
                } else {
                    return Err(VmError::AttemptToIndex(format!("{:?}", object_value)));
                }
            }
            STOREINDEXK {
                source,
                object,
                index,
            } => {
                let object_value = self.get_register(object)?;
                let source_value = self.get_register(source)?;
                if let VmValue::Array(mut arr) = object_value.clone() {
                    arr[index] = source_value.clone();
                    self.set_register(object, &VmValue::Array(arr))?;
                } else {
                    return Err(VmError::AttemptToIndex(format!("{:?}", object_value)));
                }
            }

            JMP(address) => self.jump(address)?,
            JZ { source, address } => {
                let source_value = self.get_register(source)?;
                if !source_value.is_truthy() {
                    self.jump(address)?
                }
            }
            JNZ { source, address } => {
                let source_value = self.get_register(source)?;
                if source_value.is_truthy() {
                    self.jump(address)?
                }
            }
            STORE { source, name } => {
                self.set_variable(name, &self.get_register(source)?.clone());
            }
            LOAD { target, name } => {
                let value = self.lookup_variable(&name);
                self.set_register(target, &value.clone())?
            }
            CALL(address) => self.call(address)?,
            RETURN => self.call_return()?,

            PRINT(target) => println!("{}", self.get_register(target)?),
            HALT => self.pc = self.instruction_count(),
        }
        Ok(())
    }

    fn set_variable(&mut self, name: String, value: &VmValue) -> Option<VmValue> {
        self.variables.insert(name, value.clone())
    }

    fn lookup_variable(&self, name: &str) -> &VmValue {
        self.variables
            .get(name)
            .expect(format!("variable {} not found in local scope", name).as_str())
    }

    fn comparison_binop<F>(
        &mut self,
        target: usize,
        a: usize,
        b: usize,
        f: F,
    ) -> Result<(), VmError>
    where
        F: FnOnce(&VmValue, &VmValue) -> bool,
    {
        let a_value = self.get_register(a)?;
        let b_value = self.get_register(b)?;
        let result = f(a_value, b_value);
        self.set_register(target, &VmValue::Boolean(result))
    }

    fn logical_unop<F>(&mut self, target: usize, operand: usize, f: F) -> Result<(), VmError>
    where
        F: FnOnce(bool) -> bool,
    {
        let operand_value = self.get_register(operand)?;
        let result = f(operand_value.is_truthy());
        self.set_register(target, &VmValue::Boolean(result))
    }

    fn logical_binop<F>(&mut self, target: usize, a: usize, b: usize, f: F) -> Result<(), VmError>
    where
        F: FnOnce(bool, bool) -> bool,
    {
        let a_value = self.get_register(a)?;
        let b_value = self.get_register(b)?;
        let result = f(a_value.is_truthy(), b_value.is_truthy());
        self.set_register(target, &VmValue::Boolean(result))
    }

    fn arithmetic_unop<F>(&mut self, target: usize, operand: usize, f: F) -> Result<(), VmError>
    where
        F: FnOnce(f64) -> f64,
    {
        let operand_value = self.get_register(operand)?;
        if let VmValue::Int(int) = operand_value {
            let result = f(*int as f64);
            self.set_register(target, &VmValue::Int(result as i32))
        } else if let VmValue::Float(float) = operand_value {
            self.set_register(target, &VmValue::Float(f(*float)))
        } else {
            Err(VmError::OperandTypeMismatch {
                expected: "number".to_string(),
                actual: format!("{:?}", operand_value),
            })
        }
    }

    fn arithmetic_binop<F>(
        &mut self,
        target: usize,
        a: usize,
        b: usize,
        opcode_name: String,
        f: F,
    ) -> Result<(), VmError>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let a_value = self.get_register(a)?;
        let b_value = self.get_register(b)?;
        let (a_number, b_number, both_int) = match (a_value, b_value) {
            (VmValue::Int(ai), VmValue::Int(bi)) => (*ai as f64, *bi as f64, true),
            (VmValue::Int(ai), VmValue::Float(bf)) => (*ai as f64, *bf, false),
            (VmValue::Float(af), VmValue::Int(bi)) => (*af, *bi as f64, false),
            (VmValue::Float(af), VmValue::Float(bf)) => (*af, *bf, false),
            (a_other, b_other) => {
                return Err(VmError::BinaryTypeMismatch {
                    opcode_name,
                    expected: "number".to_string(),
                    a_actual: format!("{:?}", a_other),
                    b_actual: format!("{:?}", b_other),
                });
            }
        };

        let result = f(a_number, b_number);
        if opcode_name == "IDIV" || (both_int && result.fract() == 0.0) {
            self.set_register(target, &VmValue::Int(result as i32))
        } else {
            self.set_register(target, &VmValue::Float(result))
        }
    }

    fn get_register(&self, index: usize) -> Result<&VmValue, VmError> {
        self.registers.get(index).ok_or_else(|| {
            VmError::RegisterOutOfBounds(format!("invalid register index {}", index))
        })
    }

    fn set_register(&mut self, index: usize, value: &VmValue) -> Result<(), VmError> {
        if let Some(reg) = self.registers.get_mut(index) {
            *reg = value.clone();
            Ok(())
        } else {
            Err(VmError::RegisterOutOfBounds(format!(
                "invalid register index {}",
                index
            )))
        }
    }

    fn jump(&mut self, address: usize) -> Result<(), VmError> {
        if address >= self.instruction_count() {
            Err(VmError::ProgramCounterOutOfBounds)
        } else {
            self.pc = address;
            Ok(())
        }
    }

    fn call(&mut self, address: usize) -> Result<(), VmError> {
        if address >= self.instruction_count() {
            return Err(VmError::ProgramCounterOutOfBounds);
        }

        self.call_stack.push(Frame::new(self.pc));
        self.pc = address;
        Ok(())
    }

    fn call_return(&mut self) -> Result<(), VmError> {
        let frame = self.call_stack.pop().ok_or(VmError::CallStackEmpty)?;
        self.pc = frame.return_address;
        Ok(())
    }

    fn current_instruction(&self) -> Instruction {
        self.program.instructions[self.pc].clone()
    }

    fn instruction_count(&self) -> usize {
        self.program.instructions.len()
    }
}
