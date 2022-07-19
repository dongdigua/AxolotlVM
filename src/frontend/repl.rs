use dialoguer::Input;
use crate::frontend::parser;

pub fn repl() {
    let mut repl = Input::new();
    let mut counter = 0;
    loop {
        let input: String = repl
            .with_prompt(format!("repl [{}]", counter))
            .interact_text()
            .unwrap();

        match parser::parse(&input) {
            Ok(parsed) => {
                println!("{:?}", parsed);
                counter += 1;
            }
            Err(err)   => println!("[PARSER]: {:?}", err),
        }
    }
}

