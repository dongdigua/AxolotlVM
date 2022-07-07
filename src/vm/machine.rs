use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use std::{thread, time};
use std::io::Write;
use console::Term;


#[derive(Debug)]
pub struct VM {
    pub stack: Vec<Value>,
    pub constant_pool: Vec<Value>
}

impl VM {
    pub fn new() -> Self {
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation
        VM {
            stack: Vec::with_capacity(256),
            constant_pool: Vec::with_capacity(16),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }

    pub fn run(&mut self, program: &Vec<ByteCode>) {
        for byte in program {
            match byte {
                ByteCode::HALT => break,
                ByteCode::PUSH(value) => self.stack.push(value.clone()),
                ByteCode::POP => {self.stack.pop();}

                ByteCode::SET(index) => self.constant_pool.insert(*index, self.stack.pop().unwrap()),
                ByteCode::LOAD(index) => self.stack.push(self.constant_pool[*index].clone()),

                ByteCode::ADD => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();  // no need to "let mut a"
                    a.add(b);
                }
                ByteCode::SUB => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.sub(b);
                }
                ByteCode::MUL => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.mul(b);
                }
                ByteCode::DIV => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.div(b);
                }
                ByteCode::REM => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.rem(b);
                }
                ByteCode::AND => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.and(b);
                }
                ByteCode::OR => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.or(b);
                }
                ByteCode::XOR => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.xor(b);
                }
                ByteCode::NOT => {
                    let a = self.stack.last_mut().unwrap();
                    a.not();
                }
                _ => todo!("what the fuck!"),
            }
            self.render(byte, Term::stdout());
        }
    }

    pub fn render(&self, byte: &ByteCode, mut term: Term) {
        write!(term, "{:?}\n", byte).unwrap();
        for i in &self.stack {
            write!(term, "|{:?}", i).unwrap();
        }
        write!(term, "|").unwrap();
        //term.flush();
        thread::sleep(time::Duration::from_millis(500));
        term.clear_line().unwrap();
        term.clear_last_lines(1).unwrap();
    }
}
