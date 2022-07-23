use bincode::{Encode, Decode};
use crate::vm::value::Value;
use crate::vm::bytecode::ByteCode;
use crate::builtin::linkedlist::List;
use std::fmt::{Display, Formatter};

// it seems bincode cannot encode/decode Linkedlist
// so maybe I should implment a Linkedlist myself
// https://course.rs/too-many-lists/intro.html
#[derive(Clone, PartialEq, Encode, Decode, Debug)]
pub enum ObjType {
    Cons(List<Value>),
    Func(usize, Vec<ByteCode>),
    Str(String),
}

impl Display for ObjType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjType::Cons(l) => write!(f, "{:?}", l),
            ObjType::Func(_argc, body) => write!(f, "{:?}", body),
            ObjType::Str(s) => write!(f, "{:?}", s),
        }
    }
}
