use ryde::{
    instruction::Instruction,
    serde::{Program, serializer},
    value::VmValue,
    vm::Vm,
};

fn main() {
    let instructions: Vec<Instruction> = vec![
        Instruction::CALL(2),
        Instruction::HALT,
        Instruction::LOADV {
            target: 0,
            value: VmValue::String("hello, world!".into()),
        },
        Instruction::PRINT(0),
        Instruction::HALT,
    ];

    // let program = Program::from_instructions(instructions);
    // let binary = serializer::serialize(&program).unwrap();
    // println!("{:02X?}", binary);
    let program = Program::from_file("out.bin").unwrap();
    let mut vm = Vm::new(&program, 4);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }
}
