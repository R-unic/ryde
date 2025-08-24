use ryde::{instruction::Instruction, value::VmValue, vm::Vm};

fn main() {
    let program = vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(6),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(5),
        },
        Instruction::NEQ {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::PRINT(0),
        Instruction::HALT,
    ];

    let mut vm = Vm::new(program, 8);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }
}
