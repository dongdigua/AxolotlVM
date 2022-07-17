use bincode::{Encode, Decode};
use crate::vm::value::Value;
use crate::builtin::linkedlist::List;

// it seems bincode cannot encode/decode Linkedlist
// so maybe I should implment a Linkedlist myself
// https://course.rs/too-many-lists/intro.html
#[derive(Clone, PartialEq, Encode, Decode, Debug)]
pub enum ObjType {
    Cons(List<Value>),
    Func,
    String(String),
}
