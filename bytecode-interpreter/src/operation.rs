use crate::errors::BytecodeError;
use crate::errors::BytecodeError::OperationParsingError;

/// Structure represents all possible operation instructions, that are supported by this interpreter
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    /// Puts value into the stack
    LoadVal(i32),
    /// Cycle instructions for fixed amount of iterations
    Loop(u32),
    /// Puts value into the stack after getting it from variable's dictionary
    ReadVar(String),
    /// Pops value from the stack and writes it into variable (in dictionary)
    WriteVar(String),
    /// + operator between 2 numbers in the stack
    Add,
    /// - operator between 2 numbers in the stack
    Subtract,
    /// * operator between 2 numbers in the stack
    Multiply,
    /// / operator between 2 numbers in the stack
    Divide,
    /// cycle termination operation
    EndLoop,
    /// execution termination operation
    ReturnValue
}

impl Operation {

    /// Parses Operation from the given string
    pub fn parse_operation(op: String) -> Result<Operation, BytecodeError> {
        let op_split: Vec<&str> = op.split_whitespace().collect();
        return match op_split.len() {
            1 => match op_split[0] {
                    "ADD" => Ok(Operation::Add),
                    "SUBTRACT" => Ok(Operation::Subtract),
                    "MULTIPLY" => Ok(Operation::Multiply),
                    "DIVIDE" => Ok(Operation::Divide),
                    "END_LOOP" => Ok(Operation::EndLoop),
                    "RETURN_VALUE" => Ok(Operation::ReturnValue),
                    _ => Err(OperationParsingError("Unknown operation provided".to_string()))
                },
            2 => match op_split[0] {
                    "LOAD_VAL" => Self::parse_load_op(&op_split),
                    "LOOP" => Self::parse_loop_op(&op_split),
                    "WRITE_VAR" => Ok(Operation::WriteVar(op_split[1].to_string())),
                    "READ_VAR" => Ok(Operation::ReadVar(op_split[1].to_string())),
                    _ => Err(OperationParsingError("Unknown operation provided".to_string()))
                },
            _ => Err(OperationParsingError("No operation or too many args provided".to_string()))
        }
    }

    /// Parses LoadVal operation parameter
    fn parse_load_op(op_split: &Vec<&str>) -> Result<Operation, BytecodeError> {
        let value = op_split[1].parse::<i32>();
        if value.is_ok() {
            Ok(Operation::LoadVal(value.unwrap()))
        } else {
            Err(OperationParsingError("Failed to parse Op argument (should be integer)".to_string()))
        }
    }

    /// Parses Loop operation parameter
    fn parse_loop_op(op_split: &Vec<&str>) -> Result<Operation, BytecodeError> {
        let value = op_split[1].parse::<u32>();
        if value.is_ok() {
            Ok(Operation::Loop(value.unwrap()))
        } else {
            Err(OperationParsingError("Failed to parse Op argument (should be integer)".to_string()))
        }
    }

}
