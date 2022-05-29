use std::collections::HashMap;
use std::ops;
use crate::bytecode::ByteCode;
use crate::errors::BytecodeError;
use crate::operation::Operation;
use crate::process_result::ProcessResult;
use crate::loop_info::LoopInfo;

/// Structure represents the ByteCode interpreter
#[derive(Debug)]
pub struct Interpreter {
    /// Bytecode instructions
    pub bc: ByteCode,
    /// Interpreter's stack of values
    pub stack: Vec<i32>,
    /// Variables dictionary
    /// - key - String, variable "name"
    /// - value - Integer, variable value
    pub variables: HashMap<String, i32>,
    /// Information about cycle appearance
    pub loop_info: LoopInfo,
}

impl Interpreter {
    /// main function, that iterates by the bytecode instructions and executes each one \
    /// Returns:
    /// - Error, if some occurred
    /// - the top value in the stack otherwise
    pub fn execute_code(&mut self) -> Result<i32, BytecodeError> {
        for (i, op) in self.bc.operations.clone().iter().enumerate() {
            let res = self.process_instruction(&op, &i);
            match res {
                Ok(val) => match val {
                    ProcessResult::Return => break,
                    ProcessResult::Continue => continue,
                }
                Err(e) => return Err(e)
            }
        }
        if self.stack.is_empty() {
            return Err(BytecodeError::OperationProcessError("There is nothing to return - the stack is empty.".to_string()));
        }
        return Ok(self.stack.pop().unwrap());
    }

    /// Function to process each operation separately \
    /// Returns:
    /// - Error, if some occurred
    /// - the result of the instruction execution process otherwise
    fn process_instruction(&mut self, op: &Operation, op_index: &usize) -> Result<ProcessResult, BytecodeError> {
        match op {
            Operation::LoadVal(val) => self.stack.push(*val),
            Operation::Loop(val) => return self.process_start_loop_op(op_index, val),
            Operation::WriteVar(variable) => return self.process_write_op(variable),
            Operation::ReadVar(variable) => self.process_read_op(variable),
            Operation::ReturnValue => return Ok(ProcessResult::Return),
            Operation::Add => return self.process_arithmetic_op(ops::Add::add),
            Operation::Divide => return self.process_arithmetic_op(ops::Div::div),
            Operation::Multiply => return self.process_arithmetic_op(ops::Mul::mul),
            Operation::Subtract => return self.process_arithmetic_op(ops::Sub::sub),
            Operation::EndLoop => return self.process_end_loop_op(op_index),
        };
        return Ok(ProcessResult::Continue);
    }

    /// Makes all operations needed to, when the loop instruction met
    fn process_start_loop_op(&mut self, op_index: &usize, val: &u32) -> Result<ProcessResult, BytecodeError> {
        if self.loop_info.has_loop {
            return Err(BytecodeError::OperationProcessError("There is already initialized loop".to_string()))
        }
        self.loop_info.has_loop = true;
        self.loop_info.start_op_index = op_index + 1;
        self.loop_info.iterations_num = *val - 1;
        return Ok(ProcessResult::Continue);
    }

    /// Makes all operations needed to, when the EndLoop instruction met \
    /// these are
    /// - set the last op to execute index
    /// - execute the loop instructions
    fn process_end_loop_op(&mut self, op_index: &usize) -> Result<ProcessResult, BytecodeError> {
        if !self.loop_info.has_loop {
            return Err(BytecodeError::OperationProcessError("There is no initialized loop yet".to_string()))
        }

        self.loop_info.end_op_index = *op_index;
        self.loop_info.has_loop = false;
        let res = self.execute_loop();
        return match res {
            Ok(_) => Ok(ProcessResult::Continue),
            Err(e) => Err(e)
        }
    }

    /// Pops the value from the stack \
    /// Adds it into dictionary with the provided key
    fn process_write_op(&mut self, variable: &String) -> Result<ProcessResult, BytecodeError> {
        if self.stack.is_empty() {
            return Err(BytecodeError::OperationProcessError("There is no variable in the stack".to_string()));
        }
        self.variables.insert(variable.clone(), self.stack.pop().unwrap());
        return Ok(ProcessResult::Continue);
    }

    /// Gets the value from the dictionary by the provided key \
    /// Puts this value into stack \
    /// If there is no such variable in the dict - JUST prints the corresponding message \
    /// **Note**: It was made like so, because the user may make a mistake, while writing the var name. \
    /// So this gives the possibility to find the read the correct variable.
    fn process_read_op(&mut self, variable: &String) {
        match self.variables.get(variable) {
            Some(value) => self.stack.push(*value),
            _ => println!("The variable {} does not exist", variable)
        }
    }

    /// Just processes simple arithmetic operation \
    /// - Pops 2 top values from the stack
    /// - apply an arithmetic operation to them
    /// - puts the result into the stack
    ///
    /// In case of error - returns the error
    fn process_arithmetic_op<F>(&mut self, apply: F) -> Result<ProcessResult, BytecodeError>
        where F: Fn(i32, i32) -> i32 {
        if self.stack.len() < 2 {
            return Err(BytecodeError::OperationProcessError("There stack size is less than 2 and op could not be performed".to_string()));
        };
        let right_operand = self.stack.pop().unwrap();
        let left_operand = self.stack.pop().unwrap();
        let result: i32 = apply(left_operand, right_operand);
        self.stack.push(result);
        return Ok(ProcessResult::Continue)
    }

    /// Basic simple implementation of loop execution
    fn execute_loop(&mut self) -> Result<ProcessResult, BytecodeError> {
        for _ in 0..self.loop_info.iterations_num {
            for i in self.loop_info.start_op_index..self.loop_info.end_op_index {
                let res = self.process_instruction(&self.bc.operations[i].clone(), &i);
                match res {
                    Ok(_) => continue,
                    Err(e) => return Err(e)
                }
            }
        }
        return Ok(ProcessResult::Continue);
    }
}