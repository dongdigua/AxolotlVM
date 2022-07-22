// generate bytecode from lisp expression
use crate::frontend::parser::Parsed;
use crate::frontend::token::Token;
use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use crate::vm::object::ObjType;
use std::collections::HashMap;
use std::rc::Rc;

#[derive (Debug, Copy, Clone)]
pub enum CodeGenError {
    WrongNumberOfArgument(u8, u8),
    ArgTypeError,
    SymbolNotFound,
    IDK,
}

// normally last() should't return None, this message is for me when I forgot what's wrong
const STACK_LAST_ERROR: &'static str = "[CODEGEN]: Env scope stack error";

#[derive (Debug)]
pub struct GenEnv {
    pub sym: Vec<HashMap<String, usize>>,  // 作用域也是通过栈来实现递归
    pub pool_index: usize
}

impl GenEnv {
    pub fn new() -> Self {
        GenEnv {
            sym: vec![HashMap::new()],
            pool_index: 0,
        }
    }

    pub fn generate_with_halt(&mut self, expr: &Parsed) -> Result<Vec<ByteCode>, CodeGenError> {
        println!("{:?}", &expr);
        match self.generate(expr) {
            Ok(mut code) => {
                code.push(ByteCode::HALT);
                Ok(code)
            }
            e => e
        }
    }

    fn generate(&mut self, expr: &Parsed) -> Result<Vec<ByteCode>, CodeGenError> {
        match expr {
            Parsed::Token(token) => {
                match token {
                    Token::Sym(s) => {
                        let mut index = None;
                        for scope in self.sym.iter().rev() {
                            if let Some(i) = scope.get(s) {
                                index = Some(*i);
                                break;
                            }
                        }
                        match index {
                            Some(i) => Ok(vec![ByteCode::Get(i)]),
                            None => Err(CodeGenError::SymbolNotFound),
                        }
                    },
                    Token::Int(i)  => Ok(vec![ByteCode::Push(Value::Int(*i))]),
                    Token::Float(f)  => Ok(vec![ByteCode::Push(Value::Float(*f))]),
                    Token::Char(c)  => Ok(vec![ByteCode::Push(Value::Char(*c))]),
                    Token::Str(s)  => Ok(vec![ByteCode::Push(
                        Value::Ref(
                            Rc::new(ObjType::Str(s.to_string()))
                        )
                    )]),
                    _ => todo!("other tokens"),
                }
            },
            Parsed::List(list) => {
                match list.len() - 1 {
                    0 => todo!(),
                    1 => self.single_arg(&expr),
                    2 => self.double_arg(&expr),
                    3 => todo!(),
                    _more => todo!(),
                }
            }
        }
    }


    fn single_arg(&mut self, _expr: &Parsed) -> Result<Vec<ByteCode>, CodeGenError> {
        todo!()
    }

    fn double_arg(&mut self, list: &Parsed) -> Result<Vec<ByteCode>, CodeGenError> {
        let expr = if let Parsed::List(expr) = list {expr} else {todo!("should'n be this")};
        match expr[0] {
            Parsed::Token(Token::Define) => {
                if let Parsed::Token(Token::Sym(sym)) = &expr[1] {
                    let mut val = self.generate(&expr[2])?;
                    let current_scope = self.sym.last_mut().expect(STACK_LAST_ERROR);

                    if current_scope.contains_key(sym) {
                        // redefine
                        let index = *current_scope.get(sym).unwrap();
                        val.push(ByteCode::Set(index));
                        val.push(ByteCode::Get(index));  // should return the value
                        Ok(val)
                    } else {
                        current_scope
                            .insert(sym.to_string(), self.pool_index);
                        val.push(ByteCode::Set(self.pool_index));
                        val.push(ByteCode::Get(self.pool_index));  // should return the value
                        self.pool_index += 1;
                        Ok(val)
                    }
                } else {
                    Err(CodeGenError::ArgTypeError)
                }
            }
            _ => Err(CodeGenError::IDK),
        }
    }

}
