use std::{cell::RefCell, hash::Hash, rc::Rc};

use bincode::{Decode, Encode};

use crate::value::{SharedValue, VmValue};

#[derive(Encode, Decode, Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct DynamicArray(pub Rc<RefCell<Vec<VmValue>>>);

impl DynamicArray {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    pub fn new_vm_value() -> VmValue {
        VmValue::DynamicArray(DynamicArray::new())
    }

    pub fn new_index_rc(&mut self, index: usize, value: SharedValue) -> () {
        self.new_index(index, value.borrow().clone());
    }

    pub fn new_index(&mut self, index: usize, value: VmValue) -> () {
        self.check_bounds(index);

        let mut vec = self.0.borrow_mut();
        vec[index] = value;
    }

    pub fn index(&self, index: usize) -> VmValue {
        if self.out_of_bounds(index) {
            VmValue::Null
        } else {
            self.0.borrow()[index].clone()
        }
    }

    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    fn check_bounds(&mut self, index: usize) {
        if self.out_of_bounds(index) {
            self.0.borrow_mut().resize(index + 1, VmValue::Null);
        }
    }

    fn out_of_bounds(&self, index: usize) -> bool {
        self.len() <= index
    }
}

impl Hash for DynamicArray {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}
