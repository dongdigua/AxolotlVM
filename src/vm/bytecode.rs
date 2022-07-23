use crate::vm::value::Value;
use bincode::{Encode, Decode};

#[derive (Clone, Debug, PartialEq, Encode, Decode)]
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
    Arg(usize),  // for calling lambda function, like a relative get

    Jmp(usize),
    PopJmpIf(usize),
    PopJmpIfNot(usize),

    Ret,
    Call(usize),

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

    // for lisp
    CollectList(usize),
    CollectCharList(usize),
    CallTopFn,  // the number of parameter is based on the argc of Func, Func should be on top of the stack
}

