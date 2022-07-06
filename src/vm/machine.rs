use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;

#[derive(Debug)]
pub struct VM {
    pub stack: Vec<Value>,
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
                    let mut a = self.stack.last_mut().unwrap();
                    a.add(b);
                }
                ByteCode::SUB => {
                    let b = self.stack.pop().unwrap();
                    let mut a = self.stack.last_mut().unwrap();
                    a.sub(b);
                }
                ByteCode::MUL => {
                    let b = self.stack.pop().unwrap();
                    let mut a = self.stack.last_mut().unwrap();
                    a.mul(b);
                }
                ByteCode::DIV => {
                    let b = self.stack.pop().unwrap();
                    let mut a = self.stack.last_mut().unwrap();
                    a.div(b);
                }
                ByteCode::REM => {
                    let b = self.stack.pop().unwrap();
                    let mut a = self.stack.last_mut().unwrap();
                    a.rem(b);
                }
                ByteCode::AND => {
                    let b = self.stack.pop().unwrap();
                    let mut a = self.stack.last_mut().unwrap();
                    a.and(b);
                }
                ByteCode::OR => {
                    let b = self.stack.pop().unwrap();
                    let mut a = self.stack.last_mut().unwrap();
                    a.or(b);
                }
                ByteCode::XOR => {
                    let b = self.stack.pop().unwrap();
                    let mut a = self.stack.last_mut().unwrap();
                    a.xor(b);
                }
                ByteCode::NOT => {
                    let mut a = self.stack.last_mut().unwrap();
                    a.not();
                }
                _ => todo!("what the fuck!"),
            }
        }
    }
}
