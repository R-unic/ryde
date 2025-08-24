use ryde::instruction::Instruction;
use ryde::{error::VmError, value::VmValue, vm::Vm};

#[test]
fn test_loadv() -> () {
    let program = vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(420),
        },
        Instruction::HALT,
    ];

    let mut vm = Vm::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(420));
}

#[test]
fn test_add() -> () {
    let program = vec![
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
    ];

    let mut vm = Vm::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(25));
}

#[test]
fn test_sub() -> () {
    let program = vec![
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
    ];

    let mut vm = Vm::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(42));
}

#[test]
fn test_mul() -> () {
    let program = vec![
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
    ];

    let mut vm = Vm::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(69));
}

#[test]
fn test_div() -> () {
    let program = vec![
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
    ];

    let mut vm = Vm::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Int(69));
}

#[test]
fn test_halt() -> () {
    let program = vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Boolean(true),
        },
        Instruction::HALT,
        Instruction::LOADV {
            target: 0,
            value: VmValue::Float(420.69),
        },
    ];

    let mut vm = Vm::new(program, 4);
    vm.run().unwrap();

    assert_eq!(vm.registers[0], VmValue::Boolean(true));
}

#[test]
fn test_invalid_register() -> () {
    let program = vec![
        Instruction::LOADV {
            target: 10,
            value: VmValue::Int(69),
        },
        Instruction::HALT,
    ];

    let mut vm = Vm::new(program, 4);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::RegisterOutOfBounds(_))));
}
