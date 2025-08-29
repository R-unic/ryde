use ryde::{
    instruction::Instruction,
    serde::{Program, serializer},
    value::VmValue,
    vm::Vm,
};

fn main() {
    let instructions: Vec<Instruction> = vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(10),
        },
        Instruction::STORE {
            source: 0,
            name: "x".to_string(),
        },
        Instruction::LOAD {
            target: 0,
            name: "x".to_string(),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(10),
        },
        Instruction::EQ {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::JZ {
            source: 0,
            address: 8,
        },
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(69),
        },
        Instruction::JMP(9),
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(420),
        },
        Instruction::PRINT(0),
        Instruction::HALT,
    ];

    // let program = Program::from_instructions(instructions);
    // let binary = serializer::serialize(&program).unwrap();
    // println!("{:02X?}", binary);
    let program = Program::from_file("out.bin").unwrap();
    let mut vm = Vm::new(&program, 8);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }
}
