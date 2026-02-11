pub mod stack;
use std::{env, fs, process};

use emu::Interpreter;

fn main() {
    let mut program: Vec<String> = Vec::new();

    let cli: Vec<String> = env::args().collect();
    let path = &cli[1];

    let file = fs::read_to_string(&path);

    for line in file.unwrap().lines() {
        program.push(line.to_string());
    }

    let mut inter = Interpreter::new();

    match inter.execute(&program) {
        Ok(output) => {
            for line in output {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1)
        }
    }
}
