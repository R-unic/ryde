pub mod state;

use crate::{aot::state::AotState, instruction::Instruction, serde::Program, value::VmValue};

pub fn compile(program: &Program) -> String {
    let mut state = AotState::new();
    state.write_header();

    for instruction in program.instructions.iter() {
        emit_instruction(&mut state, &instruction);
    }

    state.assembly
}

const COMMENT_WIDTH: usize = 20;

fn emit_instruction(state: &mut AotState, instruction: &Instruction) -> () {
    let written = match instruction {
        Instruction::LOADV { target, value } => state.write_loadv(*target, get_immediate(value)),
        Instruction::ADD { target: _, a, b } => state.write_add(*a, *b),
        Instruction::PRINT(target) => state.write_print(*target),
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
