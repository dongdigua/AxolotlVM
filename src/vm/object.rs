use bincode::{Encode, Decode};
use crate::vm::value::Value;
use crate::builtin::linkedlist::List;
use std::fmt::{Display, Formatter};

// it seems bincode cannot encode/decode Linkedlist
// so maybe I should implment a Linkedlist myself
// https://course.rs/too-many-lists/intro.html
#[derive(Clone, PartialEq, Encode, Decode, Debug)]
pub enum ObjType {
    Cons(List<Value>),
    Func,
    Str(String),
}

impl Display for ObjType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjType::Cons(l) => write!(f, "{:?}", l),
            ObjType::Func => todo!(),
            ObjType::Str(s) => write!(f, "{:?}", s),
        }
    }
}
