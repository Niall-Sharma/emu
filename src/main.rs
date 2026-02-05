mod stack;
use std::{env, fs};

fn main() -> Result<(), &'static str> {
    let mut stack = stack::Stack::new();

    let mut program: Vec<String> = Vec::new();

    let cli: Vec<String> = env::args().collect();
    let path = &cli[1];

    let file = fs::read_to_string(&path);

    for line in file.unwrap().lines() {
        if line != "" {
            program.push(line.to_string());
        }
    }
    let mut line_number = 0;
    while line_number < program.len() {
        let line = &program[line_number];
        let ops: Vec<&str> = line.split_whitespace().collect();
        match ops.get(0) {
            Some(&"POP") => match stack.pop() {
                Ok(value) => println!("{:?}", value),
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"PUSH") => match ops[1].parse::<i32>() {
                Ok(op) => stack.push(op),
                Err(_) => {
                    let var = ops[1].to_string();
                    match stack.read_variable(&var) {
                        Ok(op) => stack.push(*op),
                        Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                    }
                }
            },
            Some(&"SUB") => match stack.pop() {
                Ok(op1) => match stack.pop() {
                    Ok(op2) => stack.push(op2 - op1),
                    Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                },
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"ADD") => match stack.pop() {
                Ok(op1) => match stack.pop() {
                    Ok(op2) => stack.push(op1 + op2),
                    Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                },
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"MULT") => match stack.pop() {
                Ok(op1) => match stack.pop() {
                    Ok(op2) => stack.push(op1 * op2),
                    Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                },
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"DIV") => match stack.pop() {
                Ok(op1) => match stack.pop() {
                    Ok(op2) => stack.push(op2 / op1),
                    Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                },
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"MOD") => match stack.pop() {
                Ok(op1) => match stack.pop() {
                    Ok(op2) => stack.push(op2 % op1),
                    Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                },
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"PRINT") => {
                let op = ops[1];
                println!("{:?}", op);
            }
            Some(&"VAR") => {
                let op = ops[1];
                match stack.read_variable(&op.to_string()) {
                    Ok(val) => println!("{:?}", val),
                    Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                }
            }
            Some(&"ASSIGN") => match stack.pop() {
                Ok(value) => {
                    let var = ops[1].to_string();
                    match stack.assign_variable(var, &value) {
                        Ok(_) => {}
                        Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                    }
                }
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"GZ") => match stack.pop() {
                Ok(value) => {
                    if value > 0 {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                }
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"LZ") => match stack.pop() {
                Ok(value) => {
                    if value < 0 {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                }
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            Some(&"LABEL") => {
                let var = ops[1].to_string();
                match i32::try_from(line_number + 1) {
                    Ok(line_num) => match stack.assign_variable(var, &line_num) {
                        Ok(_) => {}
                        Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                    },
                    Err(_) => generic_error_handler(ops.get(0).unwrap(), line_number),
                }
            }
            Some(&"JNZ") => match stack.peek() {
                Ok(op) => {
                    if op != &0i32 {
                        let var = ops[1].to_string();
                        match stack.read_variable(&var) {
                            Ok(val) => match usize::try_from(*val) {
                                Ok(val) => {
                                    line_number = val;
                                    continue;
                                }
                                Err(_) => generic_error_handler(ops.get(0).unwrap(), line_number),
                            },
                            Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
                        }
                    }
                }
                Err(e) => error_handler(ops.get(0).unwrap(), line_number, e),
            },
            None | Some(&_) => eprintln!("ERROR: INVALID operation at LINE: {line_number}"),
        }
        line_number = line_number + 1;
    }
    Ok(())
}

fn generic_error_handler(op: &str, line_number: usize) {
    eprintln!("ERROR: Invalid {op} at LINE: {line_number}");
    std::process::exit(1)
}
fn error_handler(op: &str, line_number: usize, e: &'static str) {
    eprintln!("ERROR: Invalid {op} at LINE: {line_number}: {e}");
    std::process::exit(1)
}
