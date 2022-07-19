use std::rc::Rc;

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

    Def,
    Set,
    Cond,
    Match,

    List,
    Car,
    Cdr,

    Lambda,
    Require,
    Provide,

}

pub enum LispVal {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    List(Rc<Vec<LispVal>>),
}
