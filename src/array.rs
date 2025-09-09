use bincode::{Decode, Encode};

use crate::value::{SharedValue, VmValue};

#[derive(Encode, Decode, Hash, Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
pub struct DynamicArray(pub Vec<VmValue>);

impl DynamicArray {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_vm_value() -> VmValue {
        VmValue::DynamicArray(Self::new())
    }

    pub fn new_index_rc(&mut self, index: usize, value: SharedValue) -> () {
        self.new_index(index, value.borrow().clone());
    }

    pub fn new_index(&mut self, index: usize, value: VmValue) -> () {
        self.check_bounds(index);
        self.0[index] = value;
    }

    pub fn index(&self, index: usize) -> VmValue {
        if self.out_of_bounds(index) {
            VmValue::Null
        } else {
            self.0[index].clone()
        }
    }

    fn check_bounds(&mut self, index: usize) {
        if self.out_of_bounds(index) {
            self.0.resize(index + 1, VmValue::Null);
        }
    }

    fn out_of_bounds(&self, index: usize) -> bool {
        self.0.len() <= index
    }
}
