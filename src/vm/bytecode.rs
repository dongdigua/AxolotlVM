use crate::vm::value::Value;

#[derive (Debug, PartialEq)]
pub enum ByteCode {
    HALT,
    PUSH(Value),
    POP,

    LOAD(usize),
    SET(usize),

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

