use std::io::{self, BufRead, stdin, stdout, Write};
use std::fs::File;

pub mod token;
use token::Token;

fn _sim_terminal() {
    let mut exit_status = false;

    let _test_token = Token::Var("x");

    while !exit_status {
        let mut input = String::new();

        print!("> ");
        let _= stdout().flush();

        stdin()
            .read_line(&mut input)
            .expect("Incorrect input received");
        input = String::from(input.trim());

        if input == String::from("exit") {
            exit_status = true;
        }
    }
}

fn _read_file(filepath: &str) {
    // open file
    let file;
    if let Ok(f) = File::open(filepath) {
        file = io::BufReader::new(f);
    } else {
        panic!("File could not be opened");
    }

    // prep for parsing
    let buffer = String::new();
    
    for line in file.lines() {
        match line {
            Ok(l) => {
                
            },
            Err(_) => {panic!("File could not be read");}
        }
    }
}