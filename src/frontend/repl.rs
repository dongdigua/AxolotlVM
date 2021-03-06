use dialoguer::{Input, History};
use crate::frontend::parser;
use crate::frontend::gen::GenEnv;
use crate::vm::machine::VM;
use std::collections::VecDeque;

pub fn repl() {
    println!("pid: {}", std::process::id());

    let mut env = GenEnv::new();
    let mut vm = VM::new(100, true);

    let mut counter = 0;
    let mut history = ReplHistory::new();
    loop {
        let input: String = Input::new()
            .with_prompt(format!("repl [{}]", counter))
            .history_with(&mut history)
            .interact_text()
            .unwrap();

        match parser::parse(&input) {
            Ok(parsed) => {
                match env.generate_with_halt(&parsed) {
                    Ok(code) => {
                        println!("{:?}\n{:?}", &code, &env);
                        vm.run(&code);
                        println!("{:?}", vm.stack.last().unwrap());
                        vm.reset_pc();  // otherwise it just halts
                        counter += 1;
                    }
                    Err(err) => println!("[CODEGEN]: {:?}", err)
                }
            }
            Err(err) => println!("[PARSER]: {:?}", err),
        }
    }
}

struct ReplHistory {
    history: VecDeque<String>,
}

// new feature in 0.10.1
impl History<String> for ReplHistory {
    fn read(&self, pos: usize) -> Option<String> {
        match self.history.get(pos) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }

    fn write(&mut self, val: &String) {
        self.history.push_front(val.clone())
    }
}

impl ReplHistory {
    fn new() -> Self {
        ReplHistory {
            history: VecDeque::new()
        }
    }
}

