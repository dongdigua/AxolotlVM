use axolotl::vm::machine::VM;
use axolotl::vm::bytecode::ByteCode::{
    PUSH,
    ADD,
    SUB,
    MUL,
    DIV,
    REM,
    AND,
    OR,
    XOR,
    NOT,
    HALT
};
use axolotl::vm::value::Value;

#[test]
fn test_add_two() {
    let program = vec![
        PUSH(Value::Int(1)),
        PUSH(Value::Int(1)),
        ADD,
        HALT
    ];

    let mut machine = VM::new();
    machine.run(&program);
    assert_eq!([Value::Int(2)], machine.stack[..])
}

#[test]
fn test_logical() {
    let program = vec![
        PUSH(Value::Int(5)),
        PUSH(Value::Int(3)),
        XOR,
        NOT
    ];

    let mut machine = VM::new();
    machine.run(&program);
    assert_eq!([Value::Int(-7)], machine.stack[..])
}
        

