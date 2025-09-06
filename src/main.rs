use ryde::{
    instruction::Instruction,
    serde::Program, // serializer
    value::VmValue,
    vm::Vm,
};

fn main() {
    let instructions: Vec<Instruction> = vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Array(Box::new(vec![
                VmValue::Int(420),
                VmValue::Int(69),
                VmValue::Int(1337),
            ])),
        },
        Instruction::INDEXK {
            target: 1,
            object: 0,
            index: 2,
        },
        Instruction::PRINT(1),
        Instruction::LOADV {
            target: 1,
            value: VmValue::String("balls".into()),
        },
        Instruction::STORE_INDEXK {
            source: 1,
            object: 0,
            index: 2,
        },
        Instruction::INDEXK {
            target: 1,
            object: 0,
            index: 2,
        },
        Instruction::PRINT(1),
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
