use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
    //constant_pool:
}

impl VM {
    pub fn new() -> Self {
        VM { stack: vec![] }
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }

    pub fn run(&mut self, program: &Vec<ByteCode>) {
        for byte in program {
            match byte {
                ByteCode::HALT => break,
                ByteCode::LOAD(value) => self.stack.push(value.clone()),
                ByteCode::ADD => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::add(a, b));
                }
                ByteCode::SUB => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::sub(a, b));
                }
                ByteCode::MUL => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::mul(a, b));
                }
                ByteCode::DIV => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::div(a, b));
                }
                ByteCode::REM => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::rem(a, b));
                }
                _ => todo!("what the fuck!"),
            }
        }
    }
}
