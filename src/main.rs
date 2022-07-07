#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
mod vm;

use crate::vm::machine::VM;
use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;

fn main() {
    let program: Vec<ByteCode> = vec![
        ByteCode::PUSH(Value::Int(1)),
        ByteCode::PUSH(Value::Int(2)),
        ByteCode::ADD,
        ByteCode::SET(0),
        ByteCode::LOAD(0),
        ByteCode::HALT
    ];

    let mut machine = VM::new();
    machine.run(&program);
    println!("\n{:?}", machine);
}
