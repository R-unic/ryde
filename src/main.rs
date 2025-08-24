use ryde::{instruction::Instruction, serde::Program, value::VmValue, vm::Vm};

fn main() {
    let instructions: Vec<Instruction> = vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(69),
        },
        Instruction::PRINT(0),
        Instruction::HALT,
    ];

    let program = Program::from_instructions(instructions);
    let mut vm = Vm::new(&program, 8);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }
}
