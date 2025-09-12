pub mod state;

use crate::{
    aot::state::{AotState, AotTarget},
    instruction::Instruction,
    serde::Program,
    value::VmValue,
};

pub fn compile(program: &Program) -> String {
    let mut state = AotState::new(AotTarget::Win64);
    state.write_header();

    for instruction in program.instructions.iter() {
        emit_instruction(&mut state, &instruction);
    }

    state.assembly
}

const COMMENT_WIDTH: usize = 20;

fn emit_instruction(state: &mut AotState, instruction: &Instruction) -> () {
    let written = match instruction {
        Instruction::LOADV { target, value } => load_immediate(state, target, value),
        Instruction::ADD { target: _, a, b } => state.write_add(*a, *b),
        Instruction::PRINT(target) => state.write_print(*target),
        Instruction::PRINTK(value) => {
            state.write(format!("; {:?}", instruction));
            state.write_line();

            let vreg = state.alloc_new_vreg();
            load_immediate(state, &vreg, value);
            state.write_line();
            state.write_print(vreg)
        }
        Instruction::HALT => "".to_string(),
        _ => panic!("unsupported instruction: {}", instruction),
    };

    if written.len() == 0 || written.contains("\n") {
        return;
    }

    let padding = COMMENT_WIDTH.saturating_sub(written.len());
    state.write(format!("{}; {:?}", " ".repeat(padding), instruction));
    state.write_line();
}

fn load_immediate(state: &mut AotState, target: &usize, value: &VmValue) -> String {
    state.write_loadv(*target, get_immediate(value))
}

fn get_immediate(value: &VmValue) -> String {
    if let VmValue::Int(value) = value {
        value.to_string()
    } else {
        panic!("only immediates are currently supported for VmValues");
    }
}

fn vreg_to_reg(vreg: usize) -> String {
    match vreg {
        0 => "rax",
        1 => "rcx",
        2 => "rdx",
        _ => panic!("inconvertible vreg: {}", vreg),
    }
    .to_string()
}
