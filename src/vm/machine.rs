use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use std::{thread, time};
use std::io::Write;
use console::Term;


#[derive(Debug)]
pub struct VM {
    pub stack: Vec<Value>,
    pub pc: usize,  // program counter
    pub constant_pool: Vec<Value>
}

impl VM {
    pub fn new() -> Self {
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation
        VM {
            stack: Vec::with_capacity(256),
            pc: 0,
            constant_pool: Vec::with_capacity(16),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new()
    }

    pub fn run(&mut self, program: &Vec<ByteCode>) {
        loop {
            let byte = &program[self.pc];
            match byte {
                ByteCode::HALT => break,
                ByteCode::Push(value) => self.stack.push(value.clone()),
                ByteCode::Pop => {self.stack.pop().unwrap();}

                ByteCode::Set(index) => self.constant_pool.insert(*index, self.stack.pop().unwrap()),
                ByteCode::Get(index) => self.stack.push(self.constant_pool[*index].clone()),

                ByteCode::Jump(pc) => self.pc = pc - 1,
                ByteCode::PopJumpIf(pc) => {
                    if *self.stack.last().unwrap() == Value::Bool(true) {
                        self.stack.pop();
                        self.pc = pc - 1;
                    }
                }
                ByteCode::PopJumpIfNot(pc) => {
                    if *self.stack.last().unwrap() == Value::Bool(false) {
                        self.stack.pop();
                        self.pc = pc - 1;
                    }
                }

                // normally it should pop two and push one,
                // but I want to resuce the number of operation
                ByteCode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();  // no need to "let mut a"
                    a.add(b);
                }
                ByteCode::Inc => {
                    let a = self.stack.last_mut().unwrap();
                    a.add(Value::Int(1));
                }
                ByteCode::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.sub(b);
                }
                ByteCode::Dec => {
                    let a = self.stack.last_mut().unwrap();
                    a.sub(Value::Int(1));
                }
                ByteCode::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.mul(b);
                }
                ByteCode::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.div(b);
                }
                ByteCode::Rem => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.rem(b);
                }
                ByteCode::And => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.and(b);
                }
                ByteCode::Or => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.or(b);
                }
                ByteCode::Xor => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.last_mut().unwrap();
                    a.xor(b);
                }
                ByteCode::Not => {
                    let a = self.stack.last_mut().unwrap();
                    a.not();
                }

                ByteCode::Greater => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(a.gt(*b)));
                }
                ByteCode::Less => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(a.lt(*b)));
                }
                ByteCode::Eq => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(a.eq(*b)));
                }
                ByteCode::Neq => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(! a.eq(*b)));
                }
                ByteCode::Seq => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(a == *b));
                }
                ByteCode::Sneq => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(a != *b));
                }
                _ => todo!("what the fuck!"),
            }
            self.render(byte, Term::stdout());
            self.pc += 1;
        }
    }

    pub fn render(&self, byte: &ByteCode, mut term: Term) {
        write!(term, "{:?}\n", byte).unwrap();
        for i in &self.stack {
            write!(term, "|{:?}", i).unwrap();
        }
        write!(term, "|").unwrap();
        //term.flush();
        thread::sleep(time::Duration::from_millis(100));
        term.clear_line().unwrap();
        term.clear_last_lines(1).unwrap();
    }
}
