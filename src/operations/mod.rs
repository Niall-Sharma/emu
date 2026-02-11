use crate::stack::Stack;

#[cfg(test)]
mod operation_tests;

pub fn prim_op(op: Option<&str>, stack: &mut Stack) -> Result<(), String> {
    match op {
        Some("SUB") => {
            let op1 = stack.pop().map_err(|e| e.to_string())?;
            let op2 = stack.pop().map_err(|e| e.to_string())?;
            stack.push(op2 - op1);
        }
        Some("ADD") => {
            let op1 = stack.pop().map_err(|e| e.to_string())?;
            let op2 = stack.pop().map_err(|e| e.to_string())?;
            stack.push(op1 + op2);
        }
        Some("MULT") => {
            let op1 = stack.pop().map_err(|e| e.to_string())?;
            let op2 = stack.pop().map_err(|e| e.to_string())?;
            stack.push(op1 * op2);
        }
        Some("DIV") => {
            let op1 = stack.pop().map_err(|e| e.to_string())?;
            let op2 = stack.pop().map_err(|e| e.to_string())?;
            stack.push(op2 / op1);
        }
        Some("MOD") => {
            let op1 = stack.pop().map_err(|e| e.to_string())?;
            let op2 = stack.pop().map_err(|e| e.to_string())?;
            stack.push(op2 % op1);
        }
        _ => return Err(format!("Invalid operation: {:?}", op)),
    }
    Ok(())
}

pub fn stack_op(
    op: Option<&str>,
    ops: &[&str],
    stack: &mut Stack,
    output: &mut Vec<String>,
) -> Result<(), String> {
    match op {
        Some("POP") => {
            let value = stack.pop().map_err(|e| e.to_string())?;
            output.push(format!("{:?}", value));
        }
        Some("PUSH") => match ops[1].parse::<i32>() {
            Ok(op) => stack.push(op),
            Err(_) => {
                let var = ops[1].to_string();
                let value = stack.read_variable(&var).map_err(|e| e.to_string())?;
                stack.push(*value);
            }
        },
        _ => return Err(format!("Invalid operation: {:?}", op)),
    }
    Ok(())
}

pub fn var_op(
    op: Option<&str>,
    ops: &[&str],
    stack: &mut Stack,
    output: &mut Vec<String>,
) -> Result<(), String> {
    match op {
        Some("VAR") => {
            let op = ops[1];
            let value = stack
                .read_variable(&op.to_string())
                .map_err(|e| e.to_string())?;
            output.push(value.to_string());
        }
        Some("ASSIGN") => {
            let value = stack.pop().map_err(|e| e.to_string())?;
            let var = ops[1].to_string();
            let _ = stack
                .assign_variable(var, &value)
                .map_err(|e| e.to_string())?;
        }
        _ => return Err(format!("Invalid operation: {:?}", op)),
    }
    Ok(())
}

pub fn eq_op(op: Option<&str>, stack: &mut Stack) -> Result<(), String> {
    match op {
        Some("LZ") => {
            let value = stack.pop().map_err(|e| e.to_string())?;
            if value < 0 {
                stack.push(1);
            } else {
                stack.push(0);
            }
        }
        Some("GZ") => {
            let value = stack.pop().map_err(|e| e.to_string())?;
            if value > 0 {
                stack.push(1);
            } else {
                stack.push(0);
            }
        }
        _ => return Err(format!("Invalid operation: {:?}", op)),
    }
    Ok(())
}

pub fn branch_op(
    op: Option<&str>,
    stack: &mut Stack,
    ops: &[&str],
    line_number: &usize,
) -> Result<(), String> {
    match op {
        Some("LABEL") => {
            let var = ops[1].to_string();
            let line_num = i32::try_from(*line_number + 1).map_err(|e| e.to_string())?;
            let _ = stack
                .assign_variable(var, &line_num)
                .map_err(|e| e.to_string());
        }

        _ => return Err(format!("Invalid operation: {:?}", op)),
    }
    Ok(())
}
