use std::collections::HashMap;
use std::ops;
use crate::ByteCode;
use crate::errors::BytecodeError;
use crate::operation::Operation;

#[derive(Debug)]
pub struct Interpreter {
    pub bc: ByteCode,
    pub stack: Vec<i32>,
    pub variables: HashMap<String, i32>
}

impl Interpreter {
    pub fn execute_code(&mut self) -> Result<i32, BytecodeError> {
        for op in self.bc.operations.clone() {
            let res = self.process_instruction(&op);
            match res {
                Ok(val) => { if val { break; } }
                Err(e) => return Err(e)
            }
        }
        if self.stack.is_empty() {
            return Err(BytecodeError::OperationProcessError("There is no variable left in the stack".to_string()));
        }
        return Ok(self.stack.pop().unwrap());
    }

    fn process_instruction(&mut self, op: &Operation) -> Result<bool, BytecodeError> {
        match op {
            Operation::LoadVal(val) => self.stack.push(*val),
            Operation::WriteVar(variable) => return self.process_write_op(variable),
            Operation::ReadVar(variable) => self.process_read_op(variable),
            Operation::ReturnValue => return Ok(true),
            Operation::Add => return self.process_arithmetic_op(ops::Add::add),
            Operation::Divide => return self.process_arithmetic_op(ops::Div::div),
            Operation::Multiply => return self.process_arithmetic_op(ops::Mul::mul),
            Operation::Subtract => return self.process_arithmetic_op(ops::Sub::sub),
        };
        return Ok(false);
    }

    fn process_write_op(&mut self, variable: &String) -> Result<bool, BytecodeError> {
        if self.stack.is_empty() {
            return Err(BytecodeError::OperationProcessError("There is no variable in the stack".to_string()));
        }
        self.variables.insert(variable.clone(), self.stack.pop().unwrap());
        return Ok(false);
    }

    fn process_read_op(&mut self, variable: &String) {
        match self.variables.get(variable) {
            Some(value) => self.stack.push(*value),
            _ => println!("The variable {} does not exist", variable)
        }
    }

    fn process_arithmetic_op<F>(&mut self, apply: F) -> Result<bool, BytecodeError>
        where F: Fn(i32, i32) -> i32 {
        if self.stack.len() < 2 {
            return Err(BytecodeError::OperationProcessError("There stack size is less than 2 and op could not be performed".to_string()));
        };
        let right_operand = self.stack.pop().unwrap();
        let left_operand = self.stack.pop().unwrap();
        let result: i32 = apply(left_operand, right_operand);
        self.stack.push(result);
        Ok(false)
    }
}