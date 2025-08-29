use ryde::instruction::Instruction;
use ryde::serde::Program;
use ryde::{error::vm::VmError, value::VmValue, vm::Vm};

#[test]
fn test_loadv() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(420),
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(420));
}

#[test]
fn test_add() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(10),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(15),
        },
        Instruction::ADD {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(25));
}

#[test]
fn test_sub() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(50),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(8),
        },
        Instruction::SUB {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(42));
}

#[test]
fn test_mul() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(23),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(3),
        },
        Instruction::MUL {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(69));
}

#[test]
fn test_div() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(345),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(5),
        },
        Instruction::DIV {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(69));
}

#[test]
fn test_jump() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(-69),
        },
        Instruction::JMP(3),
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(999),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(420),
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(-69));
    assert_eq!(vm.registers[1], VmValue::Int(420));
}

#[test]
fn test_conditional_jump_taken() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Boolean(false),
        },
        Instruction::JZ {
            source: 0,
            address: 3,
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(999),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(420),
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[1], VmValue::Int(420));
}

#[test]
fn test_conditional_jump_not_taken() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Boolean(true),
        },
        Instruction::JZ {
            source: 0,
            address: 4,
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(999),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(420),
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[1], VmValue::Int(420));
}

#[test]
fn test_jump_out_of_bounds() -> () {
    let program = Program::from_instructions(vec![Instruction::JMP(100), Instruction::HALT]);
    let mut vm = Vm::new(&program, 4);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::ProgramCounterOutOfBounds)));
}

#[test]
fn test_store_and_load() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(123),
        },
        Instruction::STORE {
            source: 0,
            name: "x".to_string(),
        },
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(0),
        },
        Instruction::LOAD {
            target: 1,
            name: "x".to_string(),
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[1], VmValue::Int(123));
    assert_eq!(vm.variables.get("x"), Some(&VmValue::Int(123)));
}

#[test]
fn test_halt() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Boolean(true),
        },
        Instruction::HALT,
        Instruction::LOADV {
            target: 0,
            value: VmValue::Float(420.69),
        },
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Boolean(true));
}

#[test]
fn test_invalid_register() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 10,
            value: VmValue::Int(69),
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::RegisterOutOfBounds(_))));
}
