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
    assert_eq!([Value::Int(2)], machine.stack[..]);

    /*
    in the book:
    需要注意的是，在一些语言和测试框架中，
    断言两个值相等的函数的参数叫做 expected 和 actual，
    而且指定参数的顺序是很关键的。
    然而在 Rust 中，他们则叫做 left 和 right，
    同时指定期望的值和被测试代码产生的值的顺序并不重要。
     */
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
    assert_eq!([Value::Int(-7)], machine.stack[..]);
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
    assert_eq!([Value::Int(3)], machine.stack[..]);
}

#[test]
fn test_jmp() {
    let program = vec![
        Jmp(2),
        Push(Value::Bool(false)),
        Push(Value::Bool(true)),
        HALT
    ];
    let machine = run_prog(program);
    assert_eq!([Value::Bool(true)], machine.stack[..]);
}

#[test]
fn test_conditional_jmp() {
    let program = vec![
        Push(Value::Int(1)),
        Push(Value::Int(5)),
        Greater,
        PopJmpIfNot(5),
        PopJmpIf(7),
        Dec,
        Jmp(2),
        HALT
    ];
    let machine = run_prog(program);
    assert_eq!([Value::Int(1), Value::Int(0)], machine.stack[..]);
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

#[test]
fn test_function_call() {
    let program = vec![
        Push(Value::Int(1)),
        Call(3),
        HALT,
        Swap,    // where the function begins
        Pop,
        Ret,
        HALT
    ];
    let machine = run_prog(program);
    assert!(machine.stack.is_empty());
}

