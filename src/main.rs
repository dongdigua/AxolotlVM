mod vm;

use crate::vm::machine::VM;
use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;

fn main() {
    let program: Vec<ByteCode> = vec![
        ByteCode::LOAD(Value::Int(1)),
        ByteCode::LOAD(Value::Int(2)),
        ByteCode::ADD,
        ByteCode::LOAD(Value::Int(2)),
        ByteCode::AND,
        ByteCode::HALT
    ];
    
    let mut machine = VM::new();
    machine.run(&program);
    println!("{:?}", machine);
}
