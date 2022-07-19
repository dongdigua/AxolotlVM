use crate::vm::bytecode::ByteCode;
use crate::vm::value::Value;
use crate::vm::object::ObjType;

use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;

fn pre_process(file: String) -> (Vec<String>, HashMap<String, usize>) {
    let re_trim = Regex::new(r"\s*;;.+$").unwrap();
    let re_empty = Regex::new(r"^\s*$").unwrap();
    let re_lable = Regex::new(r"\s+<- (.+)$").unwrap();
    
    let mut processed = vec![];
    let mut lable_pool = HashMap::new();

    for raw_line in file.lines() {
        let line = &re_trim.replace(raw_line, "");  // Cow<'t, str>
        if re_empty.is_match(line) {
            continue
        }

        if re_lable.is_match(line) {
            let cap = re_lable.captures(line).unwrap();
            lable_pool.insert(cap[1].to_owned(), processed.len());
            let line_trimed_lable = re_lable.replace(line, "");
            processed.push(line_trimed_lable.to_string());
        } else {
            processed.push(line.to_string());
        }
    }
    (processed, lable_pool)
}


pub fn compile_to_enum(file_content: String) -> Vec<ByteCode> {
    let re_push_int   = Regex::new(r"^push (\-?\d+)$").unwrap();
    let re_push_float = Regex::new(r"^push (\-?\d+.\d+)$").unwrap();
    let re_push_char  = Regex::new(r"^push '(\w)'$").unwrap();
    let re_push_str   = Regex::new(r#"^push "(.+)"$"#).unwrap();

    let re_instr_usize =
        Regex::new(r"^(jmp|pop_jmp_if|pop_jmp_if_not|get|set|call|collect_list) (\d+)$").unwrap();
    let re_copy = Regex::new(r"^copy -(\d+)$").unwrap();
    let re_instr_lable = Regex::new(r"^(jmp|pop_jmp_if|pop_jmp_if_not|call) (.+)$").unwrap();

    let mut prog = vec![];
    let (processed, lable_pool) = pre_process(file_content);
    for line in processed {
        let line = &line[..];
        let current_code = match line {
            "HALT" => ByteCode::HALT,
            "pop"  => ByteCode::Pop,
            "dup"  => ByteCode::Dup,
            "swap" => ByteCode::Swap,
            "ret"  => ByteCode::Ret,
            ">"    => ByteCode::Greater,
            "<"    => ByteCode::Less,
            ">="   => ByteCode::GreaterEq,
            "<="   => ByteCode::LessEq,
            "=="   => ByteCode::Eq,
            "!="   => ByteCode::Neq,
            "==="  => ByteCode::Seq,
            "!=="  => ByteCode::Sneq,
            "+"    => ByteCode::Add,
            "-"    => ByteCode::Sub,
            "*"    => ByteCode::Mul,
            "/"    => ByteCode::Div,
            "%"    => ByteCode::Rem,
            "++"   => ByteCode::Inc,
            "--"   => ByteCode::Dec,
            "&"    => ByteCode::And,
            "|"    => ByteCode::Or,
            "!"    => ByteCode::Not,
            "^"    => ByteCode::Xor,
            _ => {
                if re_push_int.is_match(line) {
                    let cap = re_push_int.captures(line).unwrap();
                    // the full match is at capture group 0.
                    let the_int = cap[1].parse::<i64>().unwrap();

                    ByteCode::Push(Value::Int(the_int))
                } else if re_push_float.is_match(line) {
                    let cap = re_push_float.captures(line).unwrap();
                    let the_float = cap[1].parse::<f64>().unwrap();

                    ByteCode::Push(Value::Float(the_float))
                } else if re_push_char.is_match(line) {
                    let cap = re_push_char.captures(line).unwrap();
                    let the_char = cap[1].chars().collect::<Vec<_>>()[0];

                    ByteCode::Push(Value::Char(the_char as u32))
                } else if re_push_str.is_match(line) {
                    let cap = re_push_str.captures(line).unwrap();
                    let the_string = cap[1].to_string();

                    ByteCode::Push(Value::Ref(Rc::new(ObjType::Str(the_string))))
                } else if re_copy.is_match(line) {
                    let cap = re_copy.captures(line).unwrap();
                    let the_usize = cap[1].parse::<usize>().unwrap();

                    ByteCode::Copy(the_usize)
                } else if re_instr_usize.is_match(line) {
                    let cap = re_instr_usize.captures(line).unwrap();
                    let instruction = &cap[1];
                    let the_usize = cap[2].parse::<usize>().unwrap();

                    match instruction {
                        "jmp"            => ByteCode::Jmp(the_usize),
                        "pop_jmp_if"     => ByteCode::PopJmpIf(the_usize),
                        "pop_jmp_if_not" => ByteCode::PopJmpIfNot(the_usize),
                        "get"            => ByteCode::Get(the_usize),
                        "set"            => ByteCode::Set(the_usize),
                        "call"           => ByteCode::Call(the_usize),
                        "collect_list"   => ByteCode::CollectList(the_usize),
                        _                => panic!("[COMPILE]: Unknown instruction followed by usize")
                    }
                } else if re_instr_lable.is_match(line) {
                    let cap = re_instr_lable.captures(line).unwrap();
                    let instruction = &cap[1];
                    let index = *lable_pool.get(&cap[2]).unwrap();

                    match instruction {
                        "jmp"            => ByteCode::Jmp(index),
                        "pop_jmp_if"     => ByteCode::PopJmpIf(index),
                        "pop_jmp_if_not" => ByteCode::PopJmpIfNot(index),
                        "call"           => ByteCode::Call(index),
                        _                => todo!()
                    }
                } else {
                    panic!("[COMPILE]: Unknown instruction {}\n{:?}", line, lable_pool)
                }
            }
        };
        prog.push(current_code);
    }

    prog
}

