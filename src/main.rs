use ryde::{
    instruction::Instruction,
    serde::Program, // serializer
    value::VmValue,
    vm::Vm,
};

fn main() {
    let var_name = "x";
    let instructions: Vec<Instruction> = vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(68),
        },
        Instruction::STORE {
            source: 0,
            name: var_name.to_string(),
        },
        Instruction::INC {
            target: None,
            name: var_name.to_string(),
        },
        Instruction::LOAD {
            target: 0,
            name: var_name.to_string(),
        },
        Instruction::PRINT(0),
        Instruction::HALT,
    ];

    let program = Program::from_instructions(instructions);
    // let binary = serializer::serialize(&program).unwrap();
    // println!("{:02X?}", binary);
    // let program = Program::from_file("out.bin").unwrap();
    let mut vm = Vm::new(&program, 4);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }

    println!("{:?}", vm.registers);
}
