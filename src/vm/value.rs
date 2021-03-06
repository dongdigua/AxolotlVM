use std::rc::Rc;
use std::fmt::{Debug, Formatter};
use bincode::{Encode, Decode};
use crate::vm::object::ObjType;

#[derive(Clone, PartialEq, Encode, Decode)]
pub enum Value {
    Int(i64),
    Float(f64),
    Char(u32),
    Bool(bool),
    Ref(Rc<ObjType>),
    Nil,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(i) => write!(f, "{:.5}", i),
            Value::Char(i) => write!(f, "'{}'", unsafe { std::char::from_u32_unchecked(*i) }),
            Value::Bool(i) => write!(f, "{}", i),
            Value::Ref(rf) => write!(f, "{:?}", rf),
            Value::Nil => write!(f, "nil"),
        }
    }
}

const LOGICAL_WRONG_TYPE: &'static str = "[VALUE]: Wrong type for logical computing";
const COMPARE_WRONG_TYPE: &'static str = "[VALUE]: Wrong type for comparation";

impl Value {
    fn try_into_int(self) -> i64 {
        match self {
            Value::Int(val) => val,
            Value::Float(val) => val as i64,
            Value::Char(val) => val as i64,
            Value::Nil => 0_i64,
            _ => panic!("[VALUE]: Cannot convert to Value::Int"),
        }
    }

    fn try_into_float(self) -> f64 {
        match self {
            Value::Float(val) => val,
            Value::Int(val) => val as f64,
            Value::Char(val) => val as u8 as f64,
            Value::Nil => 0f64,
            _ => panic!("[VALUE]: Cannot convert to Value::Float"),
        }
    }

    fn try_into_char(self) -> u32 {
        match self {
            Value::Char(val) => val,
            Value::Int(val) => val as u32,
            Value::Float(val) => val as u32,
            _ => panic!("[VALUE]: Cannot convert to Value::Char"),
        }
    }

    fn try_into_bool(self) -> bool {
        match self {
            Value::Bool(val) => val,
            _ => panic!("[VALUE]: Cannot convert to Value::Bool")
        }
    }

    #[inline (always)]
    fn arithmetic(
        &mut self,
        val: Value,
        func_int: &dyn Fn(i64, i64) -> i64,
        func_float: &dyn Fn(f64, f64) -> f64,
        func_uint: &dyn Fn(u32, u32) -> u32,
    ) {
        match self {
            Value::Int(s) => *s = func_int(*s, val.try_into_int()),
            Value::Float(s) => *s = func_float(*s, val.try_into_float()),
            Value::Char(s) => *s = func_uint(*s, val.try_into_char()),
            _ => panic!("[VALUE]: Wrong type for arithmetical computing"),
        }
    }

    #[inline (always)]
    pub fn add(&mut self, val: Value) {
        self.arithmetic(
            val,
            &|x: i64, y: i64| x + y,
            &|x: f64, y: f64| x + y,
            &|x: u32, y: u32| x + y,
        )
    }

    #[inline (always)]
    pub fn sub(&mut self, val: Value) {
        self.arithmetic(
            val,
            &|x: i64, y: i64| x - y,
            &|x: f64, y: f64| x - y,
            &|x: u32, y: u32| x - y,
        )
    }

    #[inline (always)]
    pub fn mul(&mut self, val: Value) {
        self.arithmetic(
            val,
            &|x: i64, y: i64| x * y,
            &|x: f64, y: f64| x * y,
            &|x: u32, y: u32| x * y,
        )
    }

    #[inline (always)]
    pub fn div(&mut self, val: Value) {
        self.arithmetic(
            val,
            &|x: i64, y: i64| x / y,
            &|x: f64, y: f64| x / y,
            &|x: u32, y: u32| x / y,
        )
    }

    #[inline (always)]
    pub fn rem(&mut self, val: Value) {
        self.arithmetic(
            val,
            &|x: i64, y: i64| x % y,
            &|x: f64, y: f64| x % y,
            &|x: u32, y: u32| x % y,
        )
    }

    pub fn and(&mut self, val: Value) {
        match self {
            Value::Int(s) => *s = *s & val.try_into_int(),
            Value::Bool(s) => *s = *s & val.try_into_bool(),
            Value::Char(s) => *s = *s & val.try_into_char(),
            _ => panic!("{}", LOGICAL_WRONG_TYPE)
        }
    }

    pub fn or(&mut self, val: Value) {
        match self {
            Value::Int(s) => *s = *s | val.try_into_int(),
            Value::Bool(s) => *s = *s | val.try_into_bool(),
            Value::Char(s) => *s = *s | val.try_into_char(),
            _ => panic!("{}", LOGICAL_WRONG_TYPE)
        }
    }

    pub fn xor(&mut self, val: Value) {
        match self {
            Value::Int(s) => *s = *s ^ val.try_into_int(),
            Value::Bool(s) => *s = *s ^ val.try_into_bool(),
            Value::Char(s) => *s = *s ^ val.try_into_char(),
            _ => panic!("{}", LOGICAL_WRONG_TYPE)
        }
    }

    pub fn not(&mut self) {
        match self {
            Value::Int(s) => *s = ! *s,
            Value::Bool(s) => *s = ! *s,
            Value::Char(s) => *s = ! *s,
            _ => panic!("{}", LOGICAL_WRONG_TYPE)
        }
    }

    pub fn gt(&self, val: Value) -> bool {
        match self {
            Value::Int(s) => *s > val.try_into_int(),
            Value::Float(s) => *s > val.try_into_float(),
            Value::Char(s) => *s > val.try_into_char(),
            _ => panic!("{}", COMPARE_WRONG_TYPE)
        }
    }

    pub fn lt(&self, val: Value) -> bool {
        match self {
            Value::Int(s) => *s < val.try_into_int(),
            Value::Float(s) => *s < val.try_into_float(),
            Value::Char(s) => *s < val.try_into_char(),
            _ => panic!("{}", COMPARE_WRONG_TYPE)
        }
    }

    pub fn eq(&self, val: Value) -> bool {
        match self {
            Value::Int(s) => *s == val.try_into_int(),
            Value::Float(s) => *s == val.try_into_float(),
            Value::Char(s) => *s == val.try_into_char(),
            _ => panic!("{}", COMPARE_WRONG_TYPE)
        }
    }
}
