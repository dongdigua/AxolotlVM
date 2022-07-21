// generate bytecode from lisp expression
use crate::frontend::parser::Parsed;
use crate::frontend::token::Token;
use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use std::collections::HashMap;

#[derive (Debug, Copy, Clone)]
pub enum CodeGenError {
    WrongNumberOfArgument(u8, u8),
    ArgTypeError,
    IDK,
}

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
                    Token::Sym(_s) => todo!("get"),
                    Token::Int(i)  => Ok(vec![ByteCode::Push(Value::Int(*i))]),
                    _ => todo!(),
                }
            },
            Parsed::List(list) => {
                match list.len() - 1 {
                    0 => todo!("should be some special function"),
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
                    self.sym
                        .last_mut()
                        .expect("[CODEGEN]: Scope stack error")
                        .insert(sym.to_string(), self.pool_index);
                    let mut val = self.generate(&expr[2])?;
                    val.push(ByteCode::Set(self.pool_index));
                    val.push(ByteCode::Get(self.pool_index));  // should return the value
                    self.pool_index += 1;
                    Ok(val)
                } else {
                    Err(CodeGenError::ArgTypeError)
                }
            }
            _ => Err(CodeGenError::IDK),
        }
    }

}
