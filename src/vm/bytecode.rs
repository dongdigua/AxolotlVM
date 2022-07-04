use crate::vm::value::Value;

#[derive (Debug)]
pub enum ByteCode {
    HALT,
    LOAD(Value),
    ADD,
    SUB,
    MUL,
    DIV,
    REM,
    AND,
    OR,
    NOT,
    XOR,
}

