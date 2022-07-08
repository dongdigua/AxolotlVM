use crate::vm::bytecode::ByteCode;
use std::fs;

pub fn compile(file: &String) -> Vec<u8> {
    let content = fs::read_to_string(file).unwrap();
    for line in content.lines() {
    }
    todo!()
}

pub fn execute_bin(file: &String) -> Vec<ByteCode> {
    todo!()
}
