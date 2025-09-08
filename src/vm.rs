use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use crate::array::DynamicArray;
use crate::error::vm::{VmError, invalid_index_err};
use crate::instruction::Instruction;
use crate::serde::Program;
use crate::value::{SharedValue, VmValue};

pub struct Vm<'a> {
    pub pc: usize,
    pub registers: Vec<SharedValue>,
    pub program: &'a Program,
    pub variables: HashMap<String, SharedValue>, // TODO: scoping
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
            registers: vec![Rc::new(RefCell::new(VmValue::Null)); register_count],
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
            LOADV { target, value } => self.set_register(target, value)?,
            ADD { target, a, b } => {
                if let Err(_) = self.arithmetic_binop(target, a, b, opcode_name, |a, b| a + b) {
                    let a_value = self.get_register(a)?.clone();
                    let b_value = self.get_register(b)?.clone();
                    if let VmValue::String(a) = a_value.borrow().clone()
                        && let VmValue::String(b) = b_value.borrow().clone()
                    {
                        self.set_register(target, VmValue::String(a + &b))?;
                    }
                }
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
                let value = self.lookup_variable(&name)?;
                if let VmValue::Int(n) = *value.borrow() {
                    let new_value = VmValue::Int(n + 1);
                    self.set_variable(name, new_value.clone());
                    if let Some(target) = target {
                        self.set_register(target, new_value)?;
                    }
                } else {
                    return Err(VmError::OperandTypeMismatch {
                        expected: "number".to_string(),
                        actual: format!("{:?}", value),
                    });
                }
            }
            DEC { target, name } => {
                let value = self.lookup_variable(&name)?;
                if let VmValue::Int(n) = *value.borrow() {
                    let new_value = VmValue::Int(n - 1);
                    self.set_variable(name, new_value.clone());
                    if let Some(target) = target {
                        self.set_register(target, new_value)?;
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
                let index_value = self.get_register(index)?;
                let value = self.index_rc(object, index_value)?;
                self.set_register(target, value)?;
            }
            INDEXN {
                target,
                object,
                index,
            } => {
                let value = self.index_known(object, index)?;
                self.set_register(target, value)?;
            }
            INDEXK {
                target,
                object,
                index,
            } => {
                let value = self.index(object, &index)?;
                self.set_register(target, value)?;
            }
            STORE_INDEX {
                source,
                object,
                index,
            } => {
                let index_value = self.get_register(index)?;
                self.new_index_rc(object, index_value, source)?;
            }
            STORE_INDEXN {
                source,
                object,
                index,
            } => {
                self.new_index_known(object, index, source)?;
            }
            STORE_INDEXK {
                source,
                object,
                index,
            } => {
                self.new_index(object, &index, source)?;
            }
            DELETE_INDEX { object, index } => {
                let index_value = self.get_register(index)?;
                self.delete_index_rc(object, index_value)?;
            }
            DELETE_INDEXN { object, index } => {
                self.delete_index_known(object, index)?;
            }
            DELETE_INDEXK { object, index } => {
                self.delete_index(object, &index)?;
            }
            NEW_ARRAY(target) => self.set_register(target, DynamicArray::new_vm_value())?,
            ARRAY_PUSH { target, source } => {
                let value = self.get_register(source)?;
                let mut arr_value = self.get_register_mut(target)?;
                if let Ok(arr) = arr_value.as_array_mut() {
                    arr.0.push(value.borrow().clone());
                }
            }
            ARRAY_PUSHK { target, value } => {
                let mut arr_value = self.get_register_mut(target)?;
                if let Ok(arr) = arr_value.as_array_mut() {
                    arr.0.push(value);
                }
            }
            ARRAY_LEN { target, source } => {
                let arr_value = self.get_register(source)?;
                let arr_ref = arr_value.borrow();
                if let Ok(arr) = arr_ref.as_array() {
                    self.set_register(target, VmValue::Int(arr.0.len() as i32))?;
                }
            }

            JMP(address) => self.jump(address)?,
            JZ { source, address } => {
                let source_value = self.get_register(source)?;
                if !source_value.borrow().is_truthy() {
                    self.jump(address)?
                }
            }
            JNZ { source, address } => {
                let source_value = self.get_register(source)?;
                if source_value.borrow().is_truthy() {
                    self.jump(address)?
                }
            }
            STORE { source, name } => {
                let value = self.get_register(source)?;
                self.set_variable_rc(name, value);
            }
            LOAD { target, name } => {
                let value = self.lookup_variable(&name)?;
                self.set_register_rc(target, value)?
            }
            CALL(address) => self.call(address)?,
            RETURN => self.call_return()?,

            PRINT(target) => println!("{}", self.get_register(target)?.borrow()),
            HALT => self.pc = self.instruction_count(),
        }
        Ok(())
    }

    fn set_variable_rc(&mut self, name: String, value: SharedValue) -> Option<SharedValue> {
        self.variables.insert(name, value)
    }

    fn set_variable(&mut self, name: String, value: VmValue) -> Option<SharedValue> {
        self.set_variable_rc(name, Rc::new(RefCell::new(value)))
    }

    fn lookup_variable(&self, name: &str) -> Result<SharedValue, VmError> {
        self.variables
            .get(name)
            .cloned()
            .ok_or(VmError::VariableNotFound(name.to_string()))
    }

    fn index_rc(&self, object: usize, index: SharedValue) -> Result<VmValue, VmError> {
        self.index(object, &index.borrow())
    }

    fn index(&self, object: usize, index: &VmValue) -> Result<VmValue, VmError> {
        let object_value = self.get_register(object)?;
        if let Ok(arr) = object_value.borrow().as_array() {
            if let VmValue::Int(i) = index {
                return Ok(arr.index(*i as usize));
            } else {
                return Err(invalid_index_err(index.clone()));
            }
        }
        Ok(VmValue::Null)
    }

    fn index_known(&self, object: usize, index: usize) -> Result<VmValue, VmError> {
        let object_value = self.get_register(object)?;
        if let Ok(arr) = object_value.borrow().as_array() {
            return Ok(arr.index(index));
        }
        Ok(VmValue::Null)
    }

    fn new_index_rc(
        &mut self,
        object: usize,
        index_value: SharedValue,
        source: usize,
    ) -> Result<(), VmError> {
        self.new_index(object, &index_value.borrow(), source)
    }

    fn new_index(&mut self, object: usize, index: &VmValue, source: usize) -> Result<(), VmError> {
        let source_value = self.get_register(source)?;
        let mut object_value = self.get_register_mut(object)?;
        if let Ok(arr) = object_value.as_array_mut() {
            if let VmValue::Int(i) = index {
                arr.new_index_rc(*i as usize, source_value);
            } else {
                return Err(invalid_index_err(index.clone()));
            }
        }
        Ok(())
    }

    fn new_index_known(
        &mut self,
        object: usize,
        index: usize,
        source: usize,
    ) -> Result<(), VmError> {
        let source_value = self.get_register(source)?;
        let mut object_value = self.get_register_mut(object)?;
        if let Ok(arr) = object_value.as_array_mut() {
            arr.new_index_rc(index, source_value);
        }
        Ok(())
    }

    fn delete_index_rc(&mut self, object: usize, index: SharedValue) -> Result<(), VmError> {
        self.delete_index(object, &index.borrow())
    }

    fn delete_index(&mut self, object: usize, index: &VmValue) -> Result<(), VmError> {
        let mut object_value = self.get_register_mut(object)?;
        if let Ok(arr) = object_value.as_array_mut() {
            if let VmValue::Int(i) = index {
                arr.new_index(*i as usize, VmValue::Null);
            } else {
                return Err(invalid_index_err(index.clone()));
            }
        }
        Ok(())
    }

    fn delete_index_known(&mut self, object: usize, index: usize) -> Result<(), VmError> {
        let mut object_value = self.get_register_mut(object)?;
        if let Ok(arr) = object_value.as_array_mut() {
            arr.new_index(index, VmValue::Null);
        }
        Ok(())
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
        let result = f(&*a_value.borrow(), &*b_value.borrow());
        self.set_register(target, VmValue::Boolean(result))
    }

    fn logical_unop<F>(&mut self, target: usize, operand: usize, f: F) -> Result<(), VmError>
    where
        F: FnOnce(bool) -> bool,
    {
        let operand_value = self.get_register(operand)?;
        let result = f(operand_value.borrow().is_truthy());
        self.set_register(target, VmValue::Boolean(result))
    }

    fn logical_binop<F>(&mut self, target: usize, a: usize, b: usize, f: F) -> Result<(), VmError>
    where
        F: FnOnce(bool, bool) -> bool,
    {
        let a_value = self.get_register(a)?;
        let b_value = self.get_register(b)?;
        let result = f(a_value.borrow().is_truthy(), b_value.borrow().is_truthy());
        self.set_register(target, VmValue::Boolean(result))
    }

    fn arithmetic_unop<F>(&mut self, target: usize, operand: usize, f: F) -> Result<(), VmError>
    where
        F: FnOnce(f64) -> f64,
    {
        let operand_value = self.get_register(operand)?;
        if let VmValue::Int(int) = *operand_value.borrow() {
            let result = f(int as f64);
            self.set_register(target, VmValue::Int(result as i32))
        } else if let VmValue::Float(float) = *operand_value.borrow() {
            self.set_register(target, VmValue::Float(f(float)))
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
        let a_ref = a_value.borrow();
        let b_ref = b_value.borrow();
        let (a_number, b_number, both_int) = match (&*a_ref, &*b_ref) {
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
            self.set_register(target, VmValue::Int(result as i32))
        } else {
            self.set_register(target, VmValue::Float(result))
        }
    }

    fn get_register(&self, index: usize) -> Result<SharedValue, VmError> {
        self.registers
            .get(index)
            .cloned()
            .ok_or(VmError::RegisterOutOfBounds(index))
    }

    fn get_register_mut(&mut self, index: usize) -> Result<RefMut<'_, VmValue>, VmError> {
        self.registers
            .get(index)
            .ok_or(VmError::RegisterOutOfBounds(index))
            .map(|rc| rc.borrow_mut())
    }

    fn set_register_rc(&mut self, index: usize, value: SharedValue) -> Result<(), VmError> {
        if let Some(reg) = self.registers.get_mut(index) {
            *reg = value;
            Ok(())
        } else {
            Err(VmError::RegisterOutOfBounds(index))
        }
    }

    fn set_register(&mut self, index: usize, value: VmValue) -> Result<(), VmError> {
        self.set_register_rc(index, Rc::new(RefCell::new(value)))
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

    fn current_instruction(&self) -> &Instruction {
        &self.program.instructions[self.pc]
    }

    fn instruction_count(&self) -> usize {
        self.program.instructions.len()
    }
}
