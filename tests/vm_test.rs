use axolotl::vm::machine::VM;
use axolotl::vm::bytecode::ByteCode::{
    Push,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    Not,
    HALT
};
use axolotl::vm::value::Value;

#[test]
fn test_add_two() {
    let program = vec![
        Push(Value::Int(1)),
        Push(Value::Int(1)),
        Add,
        HALT
    ];

    let mut machine = VM::new();
    machine.run(&program);
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

    let mut machine = VM::new();
    machine.run(&program);
    assert_eq!([Value::Int(-7)], machine.stack[..])
}
        

