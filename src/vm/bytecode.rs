use crate::vm::value::Value;
use bincode::{Encode, Decode};

#[derive (Debug, PartialEq, Encode, Decode)]
pub enum ByteCode {
    // https://course.rs/practice/naming.html
    HALT,
    Push(Value),
    Pop,
    Swap,
    Dup,
    Copy(usize),

    Get(usize),
    Set(usize),

    Jump(usize),
    PopJumpIf(usize),
    PopJumpIfNot(usize),

    Greater,
    GreaterEq,
    Less,
    LessEq,
    Eq,
    Neq,
    Seq,  // strict
    Sneq,

    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Inc,
    Dec,
    And,
    Or,
    Not,
    Xor,
}

