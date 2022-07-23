/// generate bytecode from lisp expression
use crate::frontend::parser::Parsed;
use crate::frontend::token::Token;
use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use crate::vm::object::ObjType;
use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;

#[derive (Debug, Copy, Clone)]
pub enum CodeGenError {
    WrongNumberOfArgument(u8, u8),
    ArgTypeError,
    SymbolNotFound,
    NotValidLambda,
    IDK,
}

// normally last() should't return None, this message is for me when I forgot what's wrong
const STACK_LAST_ERROR: &'static str = "[CODEGEN]: Env scope stack error";
const SHOULDNOT_REACH:  &'static str = "[CODEGEN]: Reached unexpected feild";

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
        //println!("{:?}", &expr);
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
                        if s.starts_with("@") {
                            // for lambda
                            let re_param_index = Regex::new(r"@(\d+)").unwrap();
                            let cap = re_param_index.captures(&s).unwrap();
                            let param_index = cap[1].parse::<usize>().unwrap();
                            return Ok(vec![ByteCode::Arg(param_index)])
                        }

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
                    Token::Bool(b)  => Ok(vec![ByteCode::Push(Value::Bool(*b))]),
                    Token::Int(i)   => Ok(vec![ByteCode::Push(Value::Int(*i))]),
                    Token::Float(f) => Ok(vec![ByteCode::Push(Value::Float(*f))]),
                    Token::Char(c)  => Ok(vec![ByteCode::Push(Value::Char(*c))]),
                    Token::Str(s)   => Ok(vec![ByteCode::Push(
                        Value::Ref(
                            Rc::new(ObjType::Str(s.to_string()))
                        )
                    )]),
                    _ => todo!("other tokens"),
                }
            },
            Parsed::List(list) => {
                match list.len() - 1 {
                    0 => todo!("call a funtcion"),
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
        let expr = if let Parsed::List(expr) = list { expr } else { todo!("{}", SHOULDNOT_REACH) };
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

            // arithmetic operation
            Parsed::Token(Token::Add | Token::Sub | Token::Mul | Token:: Div | Token::Rem) => {
                match (&expr[1], &expr[2]) {
                    (Parsed::Token(Token::Int(_) | Token::Float(_) | Token::Char(_) | Token::Sym(_)),
                     Parsed::Token(Token::Int(_) | Token::Float(_) | Token::Char(_) | Token::Sym(_))) => {
                        let token = if let Parsed::Token(token) = &expr[0] { token } else { todo!("{}", SHOULDNOT_REACH) };
                        let operator = match token {
                            Token::Add => ByteCode::Add,
                            Token::Sub => ByteCode::Sub,
                            Token::Mul => ByteCode::Mul,
                            Token::Div => ByteCode::Div,
                            Token::Rem => ByteCode::Rem,
                            _ => todo!("{}", SHOULDNOT_REACH)
                        };
                        let mut res = vec![];
                        res.append(&mut self.generate(&expr[1])?);
                        res.append(&mut self.generate(&expr[2])?);
                        res.push(operator);
                        Ok(res)
                    }
                    _ => Err(CodeGenError::ArgTypeError)
                }
            }

            // logical operation
            Parsed::Token(Token::And | Token::Or | Token::Xor) => {
                match (&expr[1], &expr[2]) {
                    (Parsed::Token(Token::Int(_) | Token::Bool(_) | Token::Char(_) | Token::Sym(_)),
                     Parsed::Token(Token::Int(_) | Token::Bool(_) | Token::Char(_) | Token::Sym(_))) => {
                        let token = if let Parsed::Token(token) = &expr[0] { token } else { todo!("{}", SHOULDNOT_REACH) };
                        let operator = match token {
                            Token::And => ByteCode::And,
                            Token::Or  => ByteCode::Or,
                            Token::Xor => ByteCode::Xor,
                            _ => todo!("{}", SHOULDNOT_REACH)
                        };
                        let mut res = vec![];
                        res.append(&mut self.generate(&expr[1])?);
                        res.append(&mut self.generate(&expr[2])?);
                        res.push(operator);
                        Ok(res)
                    }
                    _ => Err(CodeGenError::ArgTypeError)
                }
            }

            Parsed::Token(Token::Lambda) => {
                let argv = if let Parsed::List(argv) = &expr[1] { argv } else { return Err(CodeGenError::NotValidLambda) };
                let body = &expr[2];
                {
                    // checks if argv are all symbol
                    let mut valid = true;
                    for arg in argv {
                        match arg {
                            Parsed::Token(Token::Sym(_)) => (),
                            _ => valid = false,
                        }
                    }
                    if ! valid {
                        return Err(CodeGenError::NotValidLambda)
                    }
                }
                let hashmap_iter = argv.iter().enumerate().map(|(index, arg)| {
                    let sym = if let Parsed::Token(Token::Sym(sym)) = arg { sym } else { todo!() };
                    (sym.to_string(), format!("@{}", index))  // like elixir &1, but starts from 0
                });
                // that♂s good
                // iterator is elegant like Enum.map/2 in elixir
                // and use an acc vector will have a compile error
                // because the compiler don't know the size of vector
                let replaced = rec_replace_sym(body, &HashMap::from_iter(hashmap_iter));
                Ok(vec![ByteCode::Push(
                    Value::Ref(
                        Rc::new(ObjType::Func(argv.len(), self.generate(&replaced)?))
                ))])
            }
            _ => Err(CodeGenError::IDK),
        }

    }

}

fn rec_replace_sym(expr: &Parsed, mapper: &HashMap<String, String>) -> Parsed {
    match expr {
        Parsed::List(list) => {
            let mut acc = vec![];
            for i in list {
                acc.push(rec_replace_sym(i, mapper))
            }
            Parsed::List(acc)
        }
        Parsed::Token(Token::Sym(sym)) => {
            match mapper.get(sym) {
                Some(to) => Parsed::Token(Token::Sym(to.clone())),
                None => expr.clone(),
            }
        },
        _ => expr.clone(),
    }
}
