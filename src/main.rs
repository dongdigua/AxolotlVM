#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use axolotl::vm::machine::VM;
use axolotl::vm::bytecode::ByteCode;
use axolotl::vm::value::Value;
use axolotl::asm;

use std::fs::{self, OpenOptions, File};
use std::time::Instant;

use clap::{Arg, App, SubCommand};
use bincode;

fn prog(delay: u64, render: bool) {
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

    let mut machine = VM::new(delay, render, false);
    machine.run(&program);
    println!("\n{:?}", machine);
}

fn main() {
    // https://www.jianshu.com/p/bc693e49670f
    let matches = App::new("AxolotlVM")
        .version("0.2.0")
        .about("A stacked-based virtual machine")
        .author("By: 董地瓜@bilibili")
        .arg(Arg::new("delay")
             .required(false)
             .short('t')
             .value_name("DELAY")
             .takes_value(true)
             .help("The delay of each cycle"))
        .arg(Arg::new("no-render")
             .required(false)
             .long("no-render")
             .action(clap::ArgAction::SetTrue)
             .help("Render or not"))
        .subcommand(App::new("run")
                    .about("Run VM bytecode.")
                    .arg(Arg::new("BIN")
                         .help("assembly file")
                         .required(true))
                    .arg(Arg::new("debug")
                         .required(false)
                         .short('d')
                         .long("debug")
                         .action(clap::ArgAction::SetTrue)
                         .help("stepping debug or not")))
        .subcommand(App::new("asm")
                    .about("Compile the asm file to binary.")
                    .arg(Arg::new("ASM")
                         .help("assembly file")
                         .required(true)))
        .subcommand(App::new("com")
                    .about("Compile the source file to binary.")
                    .arg(Arg::new("SOURCE")
                         .help("source file")
                         .required(true)))
        .get_matches();

    let delay = matches.value_of("delay")
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap();
    let render = ! matches.get_one::<bool>("no-render").unwrap();

    let mut status = true;
    // stolen from GloomScript

    let config = bincode::config::standard()
        .with_little_endian()
        .with_variable_int_encoding()
        .skip_fixed_array_length();

    if status {
        matches.subcommand_matches("run").map(|m| {
            status = false;

            let debug = if render {
                *m.get_one::<bool>("debug").unwrap()
            } else {
                false
            };

            let file = m.value_of("BIN").unwrap();
            println!("axolotl bin: {}", file);

            let mut bin_file = File::open(file).unwrap();
            let program = bincode::decode_from_std_read(&mut bin_file, config).unwrap();

            let now = Instant::now();
            let mut machine = VM::new(delay, render, debug);
            machine.run(&program);
            
            let elapsed = now.elapsed();
            println!("elapsed: {:?}", elapsed);
            println!("{:?}", machine);
        });
    }

    if status {
        matches.subcommand_matches("asm").map(|m| {
            status = false;
            let file = m.value_of("ASM").unwrap();
            println!("asm: {}", file);

            let content = fs::read_to_string(file).unwrap();
            let program = asm::compile_to_enum(content);

            let mut bin_file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(file.to_owned() + ".abin")
                .unwrap();
            
            bincode::encode_into_std_write(program, &mut bin_file, config).unwrap();
            println!("bytecode: {}", file.to_owned() + ".abin");
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
        prog(delay, render);
    }
}
