use bincode::{Encode, Decode};
use crate::vm::value::Value;
use crate::builtin::linkedlist::List;
use std::fmt::{Debug, Formatter};

// it seems bincode cannot encode/decode Linkedlist
// so maybe I should implment a Linkedlist myself
// https://course.rs/too-many-lists/intro.html
#[derive(Clone, PartialEq, Encode, Decode)]
pub enum ObjType {
    Cons(List<Value>),
    Func,
    String(String),
}

impl Debug for ObjType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjType::Cons(l) => write!(f, "{:?}", l),
            ObjType::Func => todo!(),
            ObjType::String(s) => write!(f, "{:?}", s),
        }
    }
}
