use crate::aot::vreg_to_reg;

const TAB: &str = "  ";
const CONVERT_LOOP: &str = "
convert_loop:
    xor rdx, rdx
    mov rax, rbx
    div rcx                 ; rax = rax / 10, rdx = remainder
    add dl, '0'             ; convert remainder to ASCII
    dec rdi
    mov [rdi], dl
    mov rbx, rax
    test rax, rax
    jnz convert_loop

    mov ecx, -11             ; STD_OUTPUT_HANDLE
    call GetStdHandle        ; handle in rax

    mov rcx, rax             ; handle
    lea rdx, [rdi]           ; buffer start
    mov r8d, 10              ; max length
    sub rsp, 32              ; shadow space
    lea r9, [rsp+20]         ; &written
    xor r10, r10             ; NULL
    call WriteConsoleA
    add rsp, 32

    xor ecx, ecx             ; exit code 0
    call ExitProcess
";

pub struct AotState {
    indent: usize,
    pub print_used: bool,
    pub assembly: String,
}

impl AotState {
    pub fn new() -> AotState {
        AotState {
            indent: 0,
            print_used: false,
            assembly: String::new(),
        }
    }

    pub fn write_loadv(&mut self, vreg: usize, src: String) -> String {
        let reg = vreg_to_reg(vreg);
        self.write_instruction("mov", vec![reg, src])
    }

    pub fn write_add(&mut self, vreg0: usize, vreg1: usize) -> String {
        let reg0 = vreg_to_reg(vreg0);
        let reg1 = vreg_to_reg(vreg1);
        self.write_instruction("add", vec![reg0, reg1])
    }

    pub fn write_print(&mut self, vreg: usize) -> String {
        self.slice(|s| {
            let reg = vreg_to_reg(vreg);
            s.write_instruction("mov", vec!["r12".to_string(), reg.clone()]);
            s.write_line();
            s.write_instruction("mov", vec!["rbx".to_string(), "r12".to_string()]);
            s.write_line();
            s.write_instruction("lea", vec!["rdi".to_string(), "[rel buf+9]".to_string()]);
            s.write_line();
            s.write_instruction("mov", vec!["rcx".to_string(), "10".to_string()]);
            s.write_line();

            if !s.print_used {
                s.pop_indent(); // end main indentation for the loop label
                s.write_borrowed(CONVERT_LOOP);
                s.print_used = true;
            }
        })
    }

    pub fn write_header(&mut self) -> () {
        self.write_borrowed("global main");
        self.write_line();
        self.write_extern("GetStdHandle");
        self.write_extern("WriteConsoleA");
        self.write_extern("ExitProcess");
        self.write_line();

        self.section("data");
        self.write_borrowed("buf db \"          \", 10");
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
