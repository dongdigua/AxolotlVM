use crate::vm::value::Value;

#[derive (Debug, PartialEq)]
pub enum ByteCode {
    // https://course.rs/practice/naming.html
    HALT,
    Push(Value),
    Pop,

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

    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Not,
    Xor,
}

