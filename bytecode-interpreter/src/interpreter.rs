use std::collections::HashMap;
use std::ops;
use crate::bytecode::ByteCode;
use crate::errors::BytecodeError;
use crate::operation::Operation;
use crate::process_result::ProcessResult;
use crate::loop_info::LoopInfo;

#[derive(Debug)]
pub struct Interpreter {
    pub bc: ByteCode,
    pub stack: Vec<i32>,
    pub variables: HashMap<String, i32>,
    pub loop_info: LoopInfo,
}

impl Interpreter {
    pub fn execute_code(&mut self) -> Result<i32, BytecodeError> {
        for (i, op) in self.bc.operations.clone().iter().enumerate() {
            let res = self.process_instruction(&op);
            match res {
                Ok(val) => match val {
                    ProcessResult::Continue => continue,
                    ProcessResult::StartLoop => self.loop_info.start_op_index = i + 1,
                    ProcessResult::EndLoop => {
                        self.loop_info.end_op_index = i;
                        self.execute_loop();
                    },
                    ProcessResult::Return => break
                }
                Err(e) => return Err(e)
            }
        }
        if self.stack.is_empty() {
            return Err(BytecodeError::OperationProcessError("There is no variable left in the stack".to_string()));
        }
        return Ok(self.stack.pop().unwrap());
    }

    fn process_instruction(&mut self, op: &Operation) -> Result<ProcessResult, BytecodeError> {
        match op {
            Operation::LoadVal(val) => self.stack.push(*val),
            Operation::Loop(val) => {
                self.loop_info.iterations_num = *val - 1;
                return Ok(ProcessResult::StartLoop);
            },
            Operation::WriteVar(variable) => return self.process_write_op(variable),
            Operation::ReadVar(variable) => self.process_read_op(variable),
            Operation::ReturnValue => return Ok(ProcessResult::Return),
            Operation::Add => return self.process_arithmetic_op(ops::Add::add),
            Operation::Divide => return self.process_arithmetic_op(ops::Div::div),
            Operation::Multiply => return self.process_arithmetic_op(ops::Mul::mul),
            Operation::Subtract => return self.process_arithmetic_op(ops::Sub::sub),
            Operation::EndLoop => return Ok(ProcessResult::EndLoop),
        };
        return Ok(ProcessResult::Continue);
    }

    fn process_write_op(&mut self, variable: &String) -> Result<ProcessResult, BytecodeError> {
        if self.stack.is_empty() {
            return Err(BytecodeError::OperationProcessError("There is no variable in the stack".to_string()));
        }
        self.variables.insert(variable.clone(), self.stack.pop().unwrap());
        return Ok(ProcessResult::Continue);
    }

    fn process_read_op(&mut self, variable: &String) {
        match self.variables.get(variable) {
            Some(value) => self.stack.push(*value),
            _ => println!("The variable {} does not exist", variable)
        }
    }

    fn process_arithmetic_op<F>(&mut self, apply: F) -> Result<ProcessResult, BytecodeError>
        where F: Fn(i32, i32) -> i32 {
        if self.stack.len() < 2 {
            return Err(BytecodeError::OperationProcessError("There stack size is less than 2 and op could not be performed".to_string()));
        };
        let right_operand = self.stack.pop().unwrap();
        let left_operand = self.stack.pop().unwrap();
        let result: i32 = apply(left_operand, right_operand);
        self.stack.push(result);
        Ok(ProcessResult::Continue)
    }

    fn execute_loop(&mut self) {
        for _ in 0..self.loop_info.iterations_num {
            for i in self.loop_info.start_op_index..self.loop_info.end_op_index {
                self.process_instruction(&self.bc.operations[i].clone());
            }
        }
    }
}