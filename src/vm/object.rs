use std::rc::Rc;
use bincode::{Encode, Decode};

#[derive (Clone, PartialEq, Encode, Decode, Debug)]
pub struct ObjRef {
    obj: Rc<i32>
}

