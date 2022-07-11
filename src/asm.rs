use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use std::collections::HashMap;
use regex::Regex;

fn pre_process(file: String) -> (Vec<String>, HashMap<String, usize>) {
    let re_trim = Regex::new(r"\s+;;.+$").unwrap();
    let re_empty = Regex::new(r"^\s*$").unwrap();
    let re_lable = Regex::new(r"\s+<- (.+)$").unwrap();
    
    let mut processed = vec![];
    let mut lable_pool = HashMap::new();

    for raw_line in file.lines() {
        let line = &*re_trim.replace(raw_line, "");
        if re_empty.is_match(line) {
            continue
        }

        if re_lable.is_match(line) {
            let cap = re_lable.captures(line).unwrap();
            lable_pool.insert(cap[1].to_owned(), processed.len());
            let line_trimed_lable = &*re_lable.replace(line, "");
            processed.push(line_trimed_lable.to_owned());
        } else {
            processed.push(line.to_owned());
        }
    }
    (processed, lable_pool)
}


pub fn compile_to_enum(file_content: String) -> Vec<ByteCode> {
    let re_push_int = Regex::new(r"^push (\-?\d+)$").unwrap();
    let re_push_float = Regex::new(r"^push (\-?\d+.\d+)$").unwrap();
    let re_push_char = Regex::new(r"^push '(\w)'$").unwrap();
    let re_instr_usize =
        Regex::new(r"^(jump|pop_jump_if|pop_jump_if_not|get|set) (\d+)$").unwrap();
    let re_copy = Regex::new(r"^copy -(\d+)$").unwrap();
    let re_jump = Regex::new(r"^(jump|pop_jump_if|pop_jump_if_not) (.+)$").unwrap();

    let mut prog = vec![];
    let (processed, lable_pool) = pre_process(file_content);
    for line in processed {
        let line = &line[..];
        let current_code = match line {
            "HALT" => ByteCode::HALT,
            "pop" => ByteCode::Pop,
            "dup" => ByteCode::Dup,
            "swap" => ByteCode::Swap,
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
                    let the_int = cap[1].parse::<i64>().unwrap();

                    ByteCode::Push(Value::Int(the_int))
                } else if re_push_float.is_match(line) {
                    let cap = re_push_float.captures(line).unwrap();
                    let the_float = cap[1].parse::<f32>().unwrap();

                    ByteCode::Push(Value::Float(the_float))
                } else if re_push_char.is_match(line) {
                    let cap = re_push_char.captures(line).unwrap();
                    let the_char = cap[1].chars().collect::<Vec<_>>()[0];

                    ByteCode::Push(Value::Char(the_char as u32))
                } else if re_copy.is_match(line) {
                    let cap = re_copy.captures(line).unwrap();
                    let the_usize = cap[1].parse::<usize>().unwrap();

                    ByteCode::Copy(the_usize)
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
                } else if re_jump.is_match(line) {
                    let cap = re_jump.captures(line).unwrap();
                    let instruction = &cap[1];
                    let index = lable_pool.get(&cap[2]).unwrap();

                    match instruction {
                        "jump" => ByteCode::Jump(*index),
                        "pop_jump_if" => ByteCode::PopJumpIf(*index),
                        "pop_jump_if_not" => ByteCode::PopJumpIfNot(*index),
                        _ => todo!()
                    }
                } else {
                    todo!("wrong {}\n{:?}", line, lable_pool)
                }
            }
        };
        prog.push(current_code);
    }

    prog
}

