mod operations;
pub mod stack;

use stack::Stack;

pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }
    pub fn execute(&mut self, program: &[String]) -> Result<Vec<String>, String> {
        let mut output = Vec::new();
        let mut line_number = 0;
        while line_number < program.len() {
            let line = &program[line_number];

            let line = line.split("#").next().unwrap_or("").trim();
            if line.is_empty() {
                line_number += 1;
                continue;
            }

            let ops: Vec<&str> = line.split_whitespace().collect();

            match self.execute_operation(&ops, &mut line_number, &mut output) {
                Ok(should_continue) => {
                    if should_continue {
                        continue;
                    }
                    line_number += 1;
                }
                Err(e) => return Err(format!("Line {}: {}", line_number, e)),
            }
        }
        Ok(output)
    }

    fn execute_operation(
        &mut self,
        ops: &[&str],
        line_number: &mut usize,
        output: &mut Vec<String>,
    ) -> Result<bool, String> {
        match ops.get(0) {
            Some(&"POP" | &"PUSH") => {
                operations::stack_op(ops.get(0).copied(), ops, &mut self.stack, output)?
            }
            Some(&"DIV" | &"MOD" | &"MULT" | &"ADD" | &"SUB") => {
                operations::prim_op(ops.get(0).copied(), &mut self.stack)?
            }
            Some(&"PRINT") => {
                output.push(ops[1].to_string());
            }
            Some(&"VAR" | &"ASSIGN") => {
                operations::var_op(ops.get(0).copied(), ops, &mut self.stack, output)?
            }
            Some(&"GZ" | &"LZ") => operations::eq_op(ops.get(0).copied(), &mut self.stack)?,

            Some(&"LABEL") => {
                operations::branch_op(ops.get(0).copied(), &mut self.stack, ops, &line_number)?
            }
            Some(&"JNZ") => {
                let op = self.stack.peek().map_err(|e| e.to_string())?;
                if op != &0i32 {
                    let var = ops[1].to_string();
                    let value = *self.stack.read_variable(&var).map_err(|e| e.to_string())?;
                    *line_number = usize::try_from(value).map_err(|e| e.to_string())?;
                    return Ok(true);
                }
            }
            _ => return Err(format!("Invalid operation: {:?}", ops.get(0))),
        }
        Ok(false)
    }
}
