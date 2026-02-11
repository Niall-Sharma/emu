use super::*;

#[test]
fn test_stack_push() -> Result<(), String> {
    let mut stack = Stack::new();
    stack.push(5);
    assert_eq!(stack.stack_array, [5]);
    Ok(())
}

#[test]
fn test_stack_pop() -> Result<(), String> {
    let mut stack = Stack::new();
    stack.push(5);
    stack.pop()?;
    assert_eq!(stack.stack_array, []);
    Ok(())
}

#[test]
fn test_assign_variable() -> Result<(), String> {
    let mut stack = Stack::new();
    stack.push(5);
    stack.assign_variable("x".to_string(), &5)?;
    assert_eq!(stack.stack_variables.contains_key("x"), true);
    Ok(())
}

#[test]
fn test_read_variable() -> Result<(), String> {
    let mut stack = Stack::new();
    stack.push(5);
    stack.assign_variable("x".to_string(), &5)?;
    assert_eq!(stack.read_variable(&"x".to_string())?, &5);
    Ok(())
}

#[test]
fn test_peek() -> Result<(), String> {
    let mut stack = Stack::new();
    stack.push(5);
    assert_eq!(stack.peek()?, &5);
    Ok(())
}
