mod vm;

use crate::vm::machine::VM;
use crate::vm::bytecode::ByteCode;
use crate::vm::value::{self, Value};

fn main() {
    let program: Vec<ByteCode> = vec![
        ByteCode::LOAD(Value::Int(1)),
        ByteCode::LOAD(Value::Int(2)),
        ByteCode::ADD,
        ByteCode::LOAD(Value::Int(2)),
        ByteCode::DIV,
        ByteCode::LOAD(Value::Int(114514)),
        ByteCode::XOR,
        ByteCode::HALT
    ];
    
    let mut machine = VM::new();
    machine.run(&program);
    println!("{:?}", machine);
}
