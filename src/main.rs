#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

#[macro_use]
extern crate clap;

use axolotl::vm::machine::VM;
use axolotl::vm::bytecode::ByteCode;
use axolotl::vm::value::Value;
use axolotl::asm;
use std::fs;
use clap::{Arg, App, SubCommand};

fn prog(delay: u64) {
    let program: Vec<ByteCode> = vec![
        ByteCode::Push(Value::Int(1)),
        ByteCode::Push(Value::Int(5)),
        ByteCode::Greater,
        ByteCode::PopJumpIfNot(5),
        ByteCode::PopJumpIf(7),
        ByteCode::Dec,
        ByteCode::Jump(2),
        ByteCode::HALT
    ];

    let mut machine = VM::new(delay);
    machine.run(&program);
    println!("\n{:?}", machine);
}

fn main() {
    // https://www.jianshu.com/p/bc693e49670f
    let matches = App::new("AxolotlVM")
        .version("0.1.0")
        .about("A stacked-based virtual machine")
        .author("By: 董地瓜@bilibili")
        .arg(Arg::new("delay")
             .required(false)
             .short('t')
             .value_name("DELAY")
             .takes_value(true)
             .help("The delay of each cycle"))
        .subcommand(App::new("run")
                    .about("Run VM assembly file.")
                    .arg(Arg::new("ASM")
                         .help("assembly file")
                         .required(true)))
        .subcommand(App::new("com")
                    .about("Compile the source file to asm.")
                    .arg(Arg::new("SOURCE")
                         .help("source file")
                         .required(true)))
        .get_matches();

    let delay = matches.value_of("delay")
        .unwrap_or("100")
        .parse::<u64>()
        .unwrap();
    let mut status = true;
    // stolen from GloomScript
    if status {
        matches.subcommand_matches("run").map(|m| {
            status = false;
            let file = m.value_of("ASM").unwrap();
            println!("asm: {}", file);

            let content = fs::read_to_string(file).unwrap();
            let program = asm::compile_to_enum(content);

            let mut machine = VM::new(delay);
            machine.run(&program);
            println!("{:?}", machine);
        });
    }

    if status {
        matches.subcommand_matches("com").map(|m| {
            status = false;
            let file = m.value_of("SOURCE").unwrap();
            println!("source: {}", file);
        });
    }

    if status {
        prog(delay);
    }
}
