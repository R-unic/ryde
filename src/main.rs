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
            value: VmValue::Int(1),
        },
        Instruction::STORE {
            source: 0,
            name: "x".to_string(),
        },
        Instruction::LOAD {
            target: 1,
            name: "x".to_string(),
        },
        Instruction::STORE {
            source: 1,
            name: "y".to_string(),
        },
        Instruction::LOAD {
            target: 2,
            name: "y".to_string(),
        },
        Instruction::PRINT(2),
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
