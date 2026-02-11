use super::*;
use crate::{Interpreter, stack::Stack};

macro_rules! test_op {
    ($name:ident, $program:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let mut interp = Interpreter::new();
            let program: Vec<String> = $program.iter().map(|s| s.to_string()).collect();

            let output = interp.execute(&program).unwrap();
            assert_eq!(output, $expected);
        }
    };
}

test_op!(test_push_pop, &["PUSH 1", "POP"], vec!["1"]);

test_op!(test_add, &["PUSH 1", "PUSH -1", "ADD", "POP"], vec!["0"]);
test_op!(test_div, &["PUSH 2", "PUSH 1", "DIV", "POP"], vec!["2"]);
test_op!(test_sub, &["PUSH 2", "PUSH 1", "SUB", "POP"], vec!["1"]);
test_op!(test_mod, &["PUSH 3", "PUSH 2", "MOD", "POP"], vec!["1"]);
test_op!(test_mult, &["PUSH 2", "PUSH 2", "MULT", "POP"], vec!["4"]);

test_op!(test_gz_true, &["PUSH 1", "GZ", "POP"], vec!["1"]);
test_op!(test_gz_false, &["PUSH -1", "GZ", "POP"], vec!["0"]);
test_op!(test_lz_true, &["PUSH -1", "LZ", "POP"], vec!["1"]);
test_op!(test_lz_false, &["PUSH 1", "LZ", "POP"], vec!["0"]);

#[test]
fn assign_test() -> Result<(), String> {
    let stack = &mut Stack::new();
    let op = Some("ASSIGN");
    let ops: &[&str] = &["ASSIGN", "X"];
    let mut output: Vec<String> = Vec::new();

    stack.push(5);
    var_op(op, ops, stack, &mut output)?;
    assert_eq!(stack.read_variable(&"X".to_string())?, &5i32);
    Ok(())
}

#[test]
fn var_test() -> Result<(), String> {
    let stack = &mut Stack::new();
    let op = Some("VAR");
    let ops: &[&str] = &["VAR", "X"];
    let mut output: Vec<String> = Vec::new();

    stack.assign_variable("X".to_string(), &5i32)?;
    var_op(op, ops, stack, &mut output)?;
    assert_eq!(output, ["5"]);
    Ok(())
}
