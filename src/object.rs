use std::{cell::RefCell, cmp::Ordering, collections::HashMap, hash::Hash, rc::Rc};

use bincode::{Decode, Encode};

use crate::value::{SharedValue, VmValue};

#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone)]
pub struct Object(pub HashMap<VmValue, VmValue>);

impl Object {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn new_vm_value() -> VmValue {
        VmValue::Object(Rc::new(RefCell::new(Self::new())))
    }

    pub fn new_index_rc(&mut self, index: VmValue, value: SharedValue) -> () {
        self.new_index(index, value.borrow().clone());
    }

    pub fn new_index(&mut self, index: VmValue, value: VmValue) -> () {
        self.0.insert(index, value);
    }

    pub fn index(&self, index: &VmValue) -> VmValue {
        if !self.0.contains_key(index) {
            VmValue::Null
        } else {
            self.0.get(&index).unwrap().clone()
        }
    }
}

impl Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut entries: Vec<_> = self.0.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));

        for (k, v) in entries {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut self_entries: Vec<_> = self.0.iter().collect();
        let mut other_entries: Vec<_> = other.0.iter().collect();
        self_entries.sort_by(|a, b| a.0.cmp(b.0));
        other_entries.sort_by(|a, b| a.0.cmp(b.0));

        // compare lexicographically
        for ((k1, v1), (k2, v2)) in self_entries.iter().zip(other_entries.iter()) {
            match k1.cmp(k2) {
                Ordering::Equal => match v1.cmp(v2) {
                    Ordering::Equal => continue,
                    non_eq => return Some(non_eq),
                },
                non_eq => return Some(non_eq),
            }
        }

        // if lengths differ, shorter map is "less"
        self_entries.len().partial_cmp(&other_entries.len())
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
