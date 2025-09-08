use ryde::{
    // instruction::Instruction,
    serde::Program,
    // value::VmValue,
    vm::Vm,
};

fn main() {
    // let instructions: Vec<Instruction> = vec![
    //     Instruction::NEW_ARRAY(0)
    //     Instruction::ARRAY_PUSHK {
    //         target: 0,
    //         value: VmValue::Array(Box::new(vec![VmValue::Int(1), VmValue::Int(2)])),
    //     },
    //     Instruction::ARRAY_PUSHK {
    //         target: 0,
    //         value: VmValue::Array(Box::new(vec![VmValue::Int(3), VmValue::Int(4)])),
    //     },
    //     Instruction::HALT,
    // ];

    // let program = Program::from_instructions(instructions);
    // let binary = serializer::serialize(&program).unwrap();
    // println!("{:02X?}", binary);
    let program = Program::from_file("out.bin").unwrap();
    let mut vm = Vm::new(&program, 4);
    if let Err(e) = vm.run() {
        eprintln!("VM error: {}", e);
    }

    println!("\nRegisters: {:?}", vm.registers);
}
