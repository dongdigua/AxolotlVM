use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use std::{thread, time};
use std::io::Write;
use console::{Term, Key};


#[derive(Debug)]
pub struct VM {
    pub stack: Vec<Value>,
    pub pc: usize,  // program counter
    pub constant_pool: Vec<Value>,

    delay: u64,
    render: bool,
    debug: bool,
}

impl VM {
    pub fn default() -> Self {
        VM {
            stack: Vec::with_capacity(256),
            pc: 0,
            constant_pool: Vec::with_capacity(16),
            delay: 100,
            render: false,
            debug: false,
        }
    }

    pub fn new(delay: u64, render: bool, debug: bool) -> Self {
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation
        VM {
            stack: Vec::with_capacity(256),
            pc: 0,
            constant_pool: Vec::with_capacity(16),
            delay: delay,
            render,
            debug,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("[RUNTIME]: STACK UNDERFLOW")
    }

    pub fn run(&mut self, program: &Vec<ByteCode>) {
        loop {
            let byte = &program[self.pc];
            match byte {
                ByteCode::HALT => break,
                ByteCode::Push(value) => self.stack.push(value.clone()),
                ByteCode::Pop => {self.pop();}
                ByteCode::Dup => {
                    let a = self.stack.last().unwrap();
                    self.stack.push(*a);
                }
                ByteCode::Copy(relative_index) => {
                    // from tsoding live 1 fibonacci
                    // Copy(0) == Dup
                    let val = self.stack[self.stack.len() - 1 - *relative_index];
                    self.stack.push(val);
                }
                ByteCode::Swap => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(b);
                    self.stack.push(a);
                }

                ByteCode::Set(index) => {
                    if self.constant_pool.len() > *index {
                        self.constant_pool[*index] = self.pop();
                    } else {
                        let val = self.pop();
                        self.constant_pool.insert(*index, val);
                    }
                },
                ByteCode::Get(index) => self.stack.push(self.constant_pool[*index].clone()),

                ByteCode::Jmp(pc) => self.pc = pc - 1,
                ByteCode::PopJmpIf(pc) => {
                    if *self.stack.last().unwrap() == Value::Bool(true) {
                        self.stack.pop();
                        self.pc = pc - 1;
                    }
                }
                ByteCode::PopJmpIfNot(pc) => {
                    if *self.stack.last().unwrap() == Value::Bool(false) {
                        self.stack.pop();
                        self.pc = pc - 1;
                    }
                }

                // normally it should pop two and push one,
                // but I want to resuce the number of operation
                ByteCode::Add => {
                    let b = self.pop();
                    let a = self.stack.last_mut().unwrap();  // no need to "let mut a"
                    a.add(b);
                }
                ByteCode::Inc => {
                    let a = self.stack.last_mut().unwrap();
                    a.add(Value::Int(1));
                }
                ByteCode::Sub => {
                    let b = self.pop();
                    let a = self.stack.last_mut().unwrap();
                    a.sub(b);
                }
                ByteCode::Dec => {
                    let a = self.stack.last_mut().unwrap();
                    a.sub(Value::Int(1));
                }
                ByteCode::Mul => {
                    let b = self.pop();
                    let a = self.stack.last_mut().unwrap();
                    a.mul(b);
                }
                ByteCode::Div => {
                    let b = self.pop();
                    let a = self.stack.last_mut().unwrap();
                    a.div(b);
                }
                ByteCode::Rem => {
                    let b = self.pop();
                    let a = self.stack.last_mut().unwrap();
                    a.rem(b);
                }
                ByteCode::And => {
                    let b = self.pop();
                    let a = self.stack.last_mut().unwrap();
                    a.and(b);
                }
                ByteCode::Or => {
                    let b = self.pop();
                    let a = self.stack.last_mut().unwrap();
                    a.or(b);
                }
                ByteCode::Xor => {
                    let b = self.pop();
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
                ByteCode::GreaterEq => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(! a.lt(*b)));
                }
                ByteCode::Less => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(a.lt(*b)));
                }
                ByteCode::LessEq => {
                    let b = self.stack.last().unwrap();
                    let a = self.stack[self.stack.len() - 2];
                    self.stack.push(Value::Bool(! a.gt(*b)));
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
                //_ => todo!("wtf!"),
            }
            if self.render {
                self.render(byte, self.delay, self.debug, Term::stdout());
            }
            self.pc += 1;
        }
    }

    pub fn render(&mut self, byte: &ByteCode, delay: u64, debug: bool, mut term: Term) {
        write!(term, "{:?}\n", byte).unwrap();
        for i in &self.stack {
            write!(term, "|{:?}", i).unwrap();
        }
        write!(term, "|").unwrap();
        if debug {
            loop {
                match term.read_key().unwrap() {
                    Key::Enter => break,
                    Key::Char(' ') => break,
                    Key::Char('r') => {
                        self.debug = false;
                        break
                    }
                    _ => (),
                }
            }
        } else {
            thread::sleep(time::Duration::from_millis(delay));
        }
        term.clear_line().unwrap();
        term.clear_last_lines(1).unwrap();
    }
}
