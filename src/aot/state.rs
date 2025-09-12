use crate::aot::vreg_to_reg;

const TAB: &str = "  ";

#[derive(PartialEq, Eq)]
pub enum AotTarget {
    Win64,
}

impl AotTarget {
    pub fn is_win64(&self) -> bool {
        self == &AotTarget::Win64
    }
}

pub struct AotState {
    target: AotTarget,
    indent: usize,
    closest_free_vreg: usize,
    pub print_used: bool,
    pub assembly: String,
}

impl AotState {
    pub fn new(target: AotTarget) -> AotState {
        AotState {
            target,
            indent: 0,
            closest_free_vreg: 0,
            print_used: false,
            assembly: String::new(),
        }
    }

    pub fn alloc_vreg(&mut self, vreg: usize) -> usize {
        self.closest_free_vreg = vreg + 1;
        vreg
    }

    pub fn alloc_new_vreg(&mut self) -> usize {
        let allocated = self.closest_free_vreg;
        self.closest_free_vreg += 1;
        allocated
    }

    pub fn alloc_reg(&mut self, vreg: usize) -> String {
        vreg_to_reg(self.alloc_vreg(vreg))
    }

    pub fn free_vreg(&mut self, vreg: usize) -> () {
        if vreg <= self.closest_free_vreg {
            self.closest_free_vreg = vreg;
        }
    }

    pub fn write_loadv(&mut self, vreg: usize, src: String) -> String {
        let reg = self.alloc_reg(vreg);
        self.write_instruction("mov", vec![reg, src])
    }

    pub fn write_add(&mut self, vreg0: usize, vreg1: usize) -> String {
        let reg0 = vreg_to_reg(vreg0);
        let reg1 = vreg_to_reg(vreg1);
        self.write_instruction("add", vec![reg0, reg1])
    }

    pub fn write_print(&mut self, vreg: usize) -> String {
        self.free_vreg(vreg);
        self.slice(|s| {
            let reg = vreg_to_reg(vreg);

            if s.target.is_win64() {
                // shadow space for win64 calling convention
                s.write_instruction("sub", vec!["rsp".to_string(), "40".to_string()]);
                s.write_line();
            }
            s.write_instruction("lea", vec!["rcx".to_string(), "[rel fmt]".to_string()]);
            s.write_line();
            s.write_instruction("mov", vec!["rdx".to_string(), reg]);
            s.write_line();
            s.write_instruction("call", vec!["printf".to_string()]);
            s.write_line();

            if s.target.is_win64() {
                // restore stack
                s.write_instruction("add", vec!["rsp".to_string(), "40".to_string()]);
                s.write_line();
            }
        })
    }

    pub fn write_header(&mut self) -> () {
        self.write_borrowed("global main");
        self.write_line();
        self.write_extern("printf");
        self.write_line();

        self.section("data");
        self.write_borrowed("fmt db \"%d\", 10, 0");
        self.write_line();
        self.write_line();

        self.section("text");
        self.write_label("main");
    }

    fn section(&mut self, name: &str) -> () {
        self.write_borrowed("section .");
        self.write_borrowed(name);
        self.write_line();
    }

    fn write_extern(&mut self, name: &str) -> () {
        self.write_borrowed("extern ");
        self.write_borrowed(name);
        self.write_line();
    }

    fn write_label(&mut self, name: &str) {
        self.write_borrowed(name);
        self.write_borrowed(":");
        self.push_indent();
        self.write_line();
    }

    pub fn write_unit_instruction(&mut self, instruction: &str) -> String {
        self.write_borrowed(instruction);
        instruction.to_string()
    }

    pub fn write_instruction(&mut self, instruction: &str, operands: Vec<String>) -> String {
        self.slice(|s| {
            s.write_borrowed(instruction);
            s.write_borrowed(" ");
            let length = operands.len();
            for i in 0..length {
                s.write_borrowed(&operands[i]);
                if i != length - 1 {
                    s.write_borrowed(", ");
                }
            }
        })
    }

    pub fn push_indent(&mut self) -> () {
        self.indent += 1;
    }

    pub fn pop_indent(&mut self) -> () {
        self.indent -= 1;
    }

    pub fn write_line(&mut self) -> () {
        self.write_borrowed("\n");
        if self.indent > 0 {
            self.write(TAB.repeat(self.indent));
        }
    }

    pub fn write(&mut self, s: String) -> () {
        self.assembly.push_str(&s);
    }

    pub fn write_borrowed(&mut self, s: &str) -> () {
        self.assembly.push_str(s);
    }

    fn slice<F>(&mut self, f: F) -> String
    where
        F: FnOnce(&mut Self),
    {
        let start = self.assembly.len();
        f(self); // pass mutable reference into closure
        let end = self.assembly.len();
        self.assembly[start..end].to_string()
    }
}
