use std::collections::HashMap;
use regex::Regex;

#[derive (Clone, PartialEq, Debug)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Inc,
    Dec,
    And,
    Or,
    Not,
    Xor,

    Greater,
    GreaterEq,
    Less,
    LessEq,
    Eq,
    Neq,
    Seq,
    Sneq,

    Define,
    Set,
    Cond,
    Match,

    List,
    Car,
    Cdr,

    Lambda,
    Require,
    Provide,

    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(u32),
    Str(String),
    Sym(String),
}

use Token::*;

pub fn tokenlize(s: String) -> Token {
    let token_map: HashMap<&'static str, Token> = HashMap::from([
        ("+"       , Add),
        ("-"       , Sub),
        ("*"       , Mul),
        ("/"       , Div),
        ("%"       , Rem),
        ("++"      , Inc),
        ("--"      , Dec),
        ("&"       , And),
        ("|"       , Or),
        ("!"       , Not),
        ("^"       , Xor),
        (">"       , Greater),
        (">="      , GreaterEq),
        ("<"       , Less),
        ("<="      , LessEq),
        ("=="      , Eq),
        ("!="      , Neq),
        ("==="     , Seq),
        ("!=="     , Sneq),

        ("def"     , Define),
        ("set"     , Set),
        ("cond"    , Cond),
        ("match"   , Match),

        ("list"    , List),
        ("car"     , Car),
        ("cdr"     , Cdr),

        ("lambda"  , Lambda),
        ("λ"       , Lambda),
        ("require" , Require),
        ("provide" , Provide),

        ("nil"     , Nil),
        ("true"    , Bool(true)),
        ("false"   , Bool(false))
    ]);

    let re_int   = Regex::new(r#"^(\d+)$"#)    .unwrap();
    let re_float = Regex::new(r#"^(\d+.\d+)$"#).unwrap();
    let re_str   = Regex::new(r#"^"(.*)"$"#)   .unwrap();
    let re_char  = Regex::new(r#"^\\(.)$"#)    .unwrap();
    // char is something like \A
    // simmilar with clojure and racket
    // because ' is quote

    match token_map.get(s.as_str()) {
        Some(token) => token.clone(),
        None => {
            if re_int.is_match(&s) {
                let cap = re_int.captures(&s).unwrap();
                let the_int = cap[1].parse::<i64>().unwrap();
                Int(the_int)
            } else if re_float.is_match(&s) {
                let cap = re_float.captures(&s).unwrap();
                let the_float = cap[1].parse::<f64>().unwrap();
                Float(the_float)
            } else if re_char.is_match(&s) {
                // copied from asm.rs
                let cap = re_char.captures(&s).unwrap();
                let the_char = cap[1].chars().collect::<Vec<_>>()[0];
                Char(the_char as u32)
            } else if re_str.is_match(&s) {
                let cap = re_str.captures(&s).unwrap();
                Str(cap[1].to_string())
            } else {
                Sym(s)
            }
        }
    }
}
