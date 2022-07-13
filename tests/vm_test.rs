use axolotl::vm::machine::VM;
use axolotl::vm::bytecode::ByteCode::{self, *};
use axolotl::vm::value::Value;

fn run_prog(program: Vec<ByteCode>) -> VM {
    let mut machine = VM::default();
    machine.run(&program);
    machine
}
#[test]
fn test_add_two() {
    let program = vec![
        Push(Value::Int(1)),
        Push(Value::Int(1)),
        Add,
        HALT
    ];

    let machine = run_prog(program);
    assert_eq!([Value::Int(2)], machine.stack[..])
}

#[test]
fn test_logical() {
    let program = vec![
        Push(Value::Int(5)),
        Push(Value::Int(3)),
        Xor,
        Not,
        HALT,
    ];

    let machine = run_prog(program);
    assert_eq!([Value::Int(-7)], machine.stack[..])
}

#[test]
fn test_different_type() {
    let program = vec![
        Push(Value::Int(1)),
        Push(Value::Float(2.0)),
        Add,
        HALT
    ];
    let machine = run_prog(program);
    assert_eq!([Value::Int(3)], machine.stack[..])
}

#[test]
fn test_jump() {
    let program = vec![
        Jump(2),
        Push(Value::Bool(false)),
        Push(Value::Bool(true)),
        HALT
    ];
    let machine = run_prog(program);
    assert_eq!([Value::Bool(true)], machine.stack[..])
}

#[test]
fn test_conditional_jump() {
    let program = vec![
        Push(Value::Int(1)),
        Push(Value::Int(5)),
        Greater,
        PopJumpIfNot(5),
        PopJumpIf(7),
        Dec,
        Jump(2),
        HALT
    ];
    let machine = run_prog(program);
    assert_eq!([Value::Int(1), Value::Int(0)], machine.stack[..])
}

#[test]
#[should_panic(expected = "[RUNTIME]: STACK UNDERFLOW")]
fn stack_underflow() {
    let program = vec![
        Push(Value::Int(1)),
        Pop,
        Pop
    ];
    run_prog(program);  
}
