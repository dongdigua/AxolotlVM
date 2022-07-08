#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use axolotl::vm::machine::VM;
use axolotl::vm::bytecode::ByteCode;
use axolotl::vm::value::Value;
use axolotl::asm;

fn prog() {
    let program: Vec<ByteCode> = vec![
        ByteCode::Push(Value::Int(1)),
        ByteCode::Push(Value::Int(2)),
        ByteCode::Add,
        ByteCode::Set(0),
        ByteCode::Get(0),
        ByteCode::Jump(7),
        ByteCode::Push(Value::Int(114514)),
        ByteCode::Push(Value::Int(0)),
        ByteCode::HALT
    ];

    let mut machine = VM::new();
    machine.run(&program);
    println!("\n{:?}", machine);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        prog();
    } else if args.len() == 3 {
        let filename = &args[2];
        match &args[1][..] {
            "com" => {
                asm::compile(filename);
            },
            "bin" => {
                asm::execute_bin(filename);
            },
            _ => panic!("Unknown argument!")
        }
    } else {
        panic!("Too many arguments!");
    }
}
