use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use std::fs;
use regex::Regex;

pub fn compile_to_enum(file_content: String) -> Vec<ByteCode> {
    let re_push_int = Regex::new(r"^push (\d+)$").unwrap();
    let re_push_float = Regex::new(r"^push (\d+.\d+)$").unwrap();
    let re_push_char = Regex::new(r"^push '(\w)'$").unwrap();
    let re_instr_usize =
        Regex::new(r"^(jump|pop_jump_if|pop_jump_if_not|get|set) (\d+)$").unwrap();

    let mut prog = vec![];

    for line in file_content.lines() {
        let current_code = match line {
            "HALT" => ByteCode::HALT,
            "pop" => ByteCode::Pop,
            ">" => ByteCode::Greater,
            "<" => ByteCode::Less,
            ">=" => ByteCode::GreaterEq,
            "<=" => ByteCode::LessEq,
            "==" => ByteCode::Eq,
            "!=" => ByteCode::Neq,
            "===" => ByteCode::Seq,
            "!==" => ByteCode::Sneq,
            "+" => ByteCode::Add,
            "-" => ByteCode::Sub,
            "*" => ByteCode::Mul,
            "/" => ByteCode::Div,
            "%" => ByteCode::Rem,
            "++" => ByteCode::Inc,
            "--" => ByteCode::Dec,
            "&" => ByteCode::And,
            "|" => ByteCode::Or,
            "!" => ByteCode::Not,
            "^" => ByteCode::Xor,
            _ => {
                if re_push_int.is_match(line) {
                    let cap = re_push_int.captures(line).unwrap();
                    // the full match is at capture group 0.
                    let the_int = cap[1].parse::<i32>().unwrap();

                    ByteCode::Push(Value::Int(the_int))
                } else if re_push_float.is_match(line) {
                    let cap = re_push_float.captures(line).unwrap();
                    let the_float = cap[1].parse::<f32>().unwrap();

                    ByteCode::Push(Value::Float(the_float))
                } else if re_push_char.is_match(line) {
                    let cap = re_push_char.captures(line).unwrap();
                    let the_char = cap[1].chars().collect::<Vec<_>>()[0];

                    ByteCode::Push(Value::Char(the_char as u32))
                } else if re_instr_usize.is_match(line) {
                    let cap = re_instr_usize.captures(line).unwrap();
                    let instruction = &cap[1];
                    let the_usize = cap[2].parse::<usize>().unwrap();

                    match instruction {
                        "jump" => ByteCode::Jump(the_usize),
                        "pop_jump_if" => ByteCode::PopJumpIf(the_usize),
                        "pop_jump_if_not" => ByteCode::PopJumpIfNot(the_usize),
                        "get" => ByteCode::Get(the_usize),
                        "set" => ByteCode::Set(the_usize),
                        _ => todo!()
                    }
                } else {
                    todo!()
                }
            }
        };
        prog.push(current_code);
    }

    prog
}


fn enum_to_bin() {}

pub fn compile() {}

pub fn execute_bin(file: &String) -> Vec<ByteCode> {
    todo!()
}
