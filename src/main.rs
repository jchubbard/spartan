use std::io::{stdin, stdout, Write};

fn main() {
    let mut exit_status = false;

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