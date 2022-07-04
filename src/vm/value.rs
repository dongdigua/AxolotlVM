use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Char(u32),
    Bool(bool),
    Nil,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(i) => write!(f, "{:.5}", i),
            Value::Char(i) => write!(f, "'{}'", i),
            Value::Bool(i) => write!(f, "{}", i),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Value {
    pub fn add(a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Int(value_a), Value::Int(value_b)) => Value::Int(value_a + value_b),
            (Value::Float(value_a), Value::Float(value_b)) => Value::Float(value_a + value_b),
            _ => panic!("unmached or wrong type"),
        }
    }

    pub fn sub(a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Int(value_a), Value::Int(value_b)) => Value::Int(value_a - value_b),
            (Value::Float(value_a), Value::Float(value_b)) => Value::Float(value_a - value_b),
            _ => panic!("unmached or wrong type"),
        }
    }

    pub fn mul(a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Int(value_a), Value::Int(value_b)) => Value::Int(value_a * value_b),
            (Value::Float(value_a), Value::Float(value_b)) => Value::Float(value_a * value_b),
            _ => panic!("unmached or wrong type"),
        }
    }

    pub fn div(a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Int(value_a), Value::Int(value_b)) => Value::Int(value_a / value_b),
            (Value::Float(value_a), Value::Float(value_b)) => Value::Float(value_a / value_b),
            _ => panic!("unmached or wrong type"),
        }
    }
    
    pub fn rem(a: Value, b: Value) -> Value {
        match (a, b) {
            (Value::Int(value_a), Value::Int(value_b)) => Value::Int(value_a % value_b),
            (Value::Float(value_a), Value::Float(value_b)) => Value::Float(value_a % value_b),
            _ => panic!("unmached or wrong type"),
        }
    }
}
