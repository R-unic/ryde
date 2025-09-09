use ryde::{serde::Program, vm::Vm};

fn main() {
    // let mut o = Object::new();
    // o.new_index(VmValue::Int(2), VmValue::Int(4));
    // o.new_index(VmValue::Int(6), VmValue::Int(8));

    // let instructions: Vec<Instruction> = vec![
    //     Instruction::LOADV {
    //         target: 0,
    //         value: VmValue::Object(o),
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
