use std::collections::HashMap;

#[cfg(test)]
mod stack_tests;

pub struct Stack {
    size: u32,
    stack_array: Vec<i32>,
    stack_variables: HashMap<String, i32>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            size: 0,
            stack_array: Vec::new(),
            stack_variables: HashMap::new(),
        }
    }

    pub fn pop(&mut self) -> Result<i32, &'static str> {
        match self.stack_array.pop() {
            None => Err("Empty stack"),
            Some(value) => {
                self.size = self.size - 1;
                println!("{:?}", value);
                Ok(value)
            }
        }
    }

    pub fn push(&mut self, op: i32) {
        self.stack_array.push(op);
        self.size = self.size + 1;
    }

    pub fn assign_variable(&mut self, var: String, value: &i32) -> Result<(), &'static str> {
        self.stack_variables.insert(var, *value);
        Ok(())
    }

    pub fn read_variable(&self, var: &String) -> Result<&i32, &'static str> {
        match self.stack_variables.get(var) {
            Some(op) => Ok(op),
            None => Err("Variable does not exist"),
        }
    }

    pub fn peek(&self) -> Result<&i32, &'static str> {
        self.stack_array.last().ok_or("Empty stack")
    }
}
