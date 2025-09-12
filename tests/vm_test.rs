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

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(420));
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

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(25));
}

#[test]
fn test_addk() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(60),
        },
        Instruction::ADDK {
            target: 0,
            a_value: VmValue::Int(9),
            b: 0,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(69));
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

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(42));
}

#[test]
fn test_subk() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(6),
        },
        Instruction::SUBK {
            target: 0,
            a_value: VmValue::Int(75),
            b: 0,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(69));
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

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(69));
}

#[test]
fn test_mulk() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(23),
        },
        Instruction::MULK {
            target: 0,
            a_value: VmValue::Int(3),
            b: 0,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(69));
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

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(69));
}

#[test]
fn test_divk() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(5),
        },
        Instruction::DIVK {
            target: 0,
            a_value: VmValue::Int(345),
            b: 0,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(69));
}

#[test]
fn test_idiv() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(25),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(4),
        },
        Instruction::IDIV {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(6));
}

#[test]
fn test_idivk() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(4),
        },
        Instruction::IDIVK {
            target: 0,
            a_value: VmValue::Int(25),
            b: 0,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(6));
}

#[test]
fn test_pow() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(9),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(2),
        },
        Instruction::POW {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(81));
}

#[test]
fn test_powk() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(2),
        },
        Instruction::POWK {
            target: 0,
            a_value: VmValue::Int(9),
            b: 0,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(81));
}

#[test]
fn test_mod() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(25),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(9),
        },
        Instruction::MOD {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(7));
}

#[test]
fn test_modk() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(9),
        },
        Instruction::MODK {
            target: 0,
            a_value: VmValue::Int(25),
            b: 0,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(7));
}

#[test]
fn test_and() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Boolean(true),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Boolean(false),
        },
        Instruction::AND {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Boolean(true),
        },
        Instruction::LOADV {
            target: 2,
            value: VmValue::Boolean(true),
        },
        Instruction::AND {
            target: 1,
            a: 1,
            b: 2,
        },
        Instruction::LOADV {
            target: 2,
            value: VmValue::Boolean(false),
        },
        Instruction::LOADV {
            target: 3,
            value: VmValue::Boolean(false),
        },
        Instruction::AND {
            target: 2,
            a: 2,
            b: 3,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Boolean(false));
    assert_eq!(*vm.registers[1].borrow(), VmValue::Boolean(true));
    assert_eq!(*vm.registers[2].borrow(), VmValue::Boolean(false));
}

#[test]
fn test_andk() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Boolean(false),
        },
        Instruction::ANDK {
            target: 0,
            a_value: VmValue::Boolean(true),
            b: 0,
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Boolean(true),
        },
        Instruction::ANDK {
            target: 1,
            a_value: VmValue::Boolean(true),
            b: 1,
        },
        Instruction::LOADV {
            target: 2,
            value: VmValue::Boolean(false),
        },
        Instruction::ANDK {
            target: 2,
            a_value: VmValue::Boolean(false),
            b: 2,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Boolean(false));
    assert_eq!(*vm.registers[1].borrow(), VmValue::Boolean(true));
    assert_eq!(*vm.registers[2].borrow(), VmValue::Boolean(false));
}

#[test]
fn test_or() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Boolean(true),
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Boolean(false),
        },
        Instruction::OR {
            target: 0,
            a: 0,
            b: 1,
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Boolean(true),
        },
        Instruction::LOADV {
            target: 2,
            value: VmValue::Boolean(true),
        },
        Instruction::OR {
            target: 1,
            a: 1,
            b: 2,
        },
        Instruction::LOADV {
            target: 2,
            value: VmValue::Boolean(false),
        },
        Instruction::LOADV {
            target: 3,
            value: VmValue::Boolean(false),
        },
        Instruction::OR {
            target: 2,
            a: 2,
            b: 3,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Boolean(true));
    assert_eq!(*vm.registers[1].borrow(), VmValue::Boolean(true));
    assert_eq!(*vm.registers[2].borrow(), VmValue::Boolean(false));
}

#[test]
fn test_ork() -> () {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Boolean(false),
        },
        Instruction::ORK {
            target: 0,
            a_value: VmValue::Boolean(true),
            b: 0,
        },
        Instruction::LOADV {
            target: 1,
            value: VmValue::Boolean(true),
        },
        Instruction::ORK {
            target: 1,
            a_value: VmValue::Boolean(true),
            b: 1,
        },
        Instruction::LOADV {
            target: 2,
            value: VmValue::Boolean(false),
        },
        Instruction::ORK {
            target: 2,
            a_value: VmValue::Boolean(false),
            b: 2,
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Boolean(true));
    assert_eq!(*vm.registers[1].borrow(), VmValue::Boolean(true));
    assert_eq!(*vm.registers[2].borrow(), VmValue::Boolean(false));
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

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(-69));
    assert_eq!(*vm.registers[1].borrow(), VmValue::Int(420));
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

    assert_eq!(*vm.registers[1].borrow(), VmValue::Int(420));
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

    assert_eq!(*vm.registers[1].borrow(), VmValue::Int(420));
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

    assert_eq!(*vm.registers[1].borrow(), VmValue::Int(123));
    assert_eq!(*vm.variables.get("x").unwrap().borrow(), VmValue::Int(123));
}

#[test]
fn test_call_and_return() {
    let program = Program::from_instructions(vec![
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(10),
        },
        Instruction::CALL(4),
        Instruction::LOADV {
            target: 1,
            value: VmValue::Int(420),
        },
        Instruction::HALT,
        Instruction::LOADV {
            target: 2,
            value: VmValue::Int(100),
        },
        Instruction::RETURN,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    assert_eq!(*vm.registers[0].borrow(), VmValue::Int(10));
    assert_eq!(*vm.registers[1].borrow(), VmValue::Int(420));
    assert_eq!(*vm.registers[2].borrow(), VmValue::Int(100));
    assert_eq!(vm.call_stack.len(), 0);
}

#[test]
fn test_return_without_call() {
    let program = Program::from_instructions(vec![Instruction::RETURN, Instruction::HALT]);
    let mut vm = Vm::new(&program, 4);
    let result = vm.run();

    assert!(matches!(result, Err(VmError::CallStackEmpty)));
}

#[test]
fn test_visualize_callstack() {
    let program = Program::from_instructions(vec![
        Instruction::CALL(2),
        Instruction::HALT,
        Instruction::LOADV {
            target: 0,
            value: VmValue::Int(420),
        },
        Instruction::HALT,
    ]);

    let mut vm = Vm::new(&program, 4);
    vm.run().unwrap();

    let callstack_vis = vm.visualize_callstack();
    assert!(callstack_vis.contains("return address"));
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

    assert_eq!(*vm.registers[0].borrow(), VmValue::Boolean(true));
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
