use std::fs;

use ryde::{aot, serde::Program, vm::Vm};

fn main() {
    // let instructions: Vec<Instruction> = vec![
    //     Instruction::LOADV {
    //         target: 0,
    //         value: VmValue::Int(5),
    //     },
    //     Instruction::LOADV {
    //         target: 1,
    //         value: VmValue::Int(10),
    //     },
    //     Instruction::ADD {
    //         target: 0,
    //         a: 0,
    //         b: 1,
    //     },
    //     Instruction::PRINT(0),
    //     Instruction::HALT,
    // ];

    // let program = Program::from_instructions(instructions);
    // let binary = serializer::serialize(&program).unwrap();
    // println!("{:02X?}", binary);
    let program = Program::from_file("out.bin").unwrap();

    let aot = true;
    if aot {
        let asm = aot::compile(&program);
        fs::write("out.asm", &asm).expect("failed to write to assembly file");
        println!("{}", asm);
    } else {
        let mut vm = Vm::new(&program, 4);
        if let Err(e) = vm.run() {
            eprintln!("VM error: {}", e);
        }

        println!("\nRegisters: {:?}", vm.registers);
    }
}
