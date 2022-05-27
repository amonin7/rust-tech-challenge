use crate::errors::BytecodeError;
use crate::errors::BytecodeError::OperationParsingError;

#[derive(Debug, Clone)]
pub enum Operation {
    LoadVal(i32),
    WriteVar(String),
    ReadVar(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    ReturnValue
}

impl Operation {

    pub fn parse_operation(op: String) -> Result<Operation, BytecodeError> {
        let op_split: Vec<&str> = op.split_whitespace().collect();
        if op_split.len() == 0 || op_split.len() > 2 {
            Err(OperationParsingError("No operation or too many args provided".to_string()))
        } else if op_split.len() == 1 {
            match op_split[0] {
                "ADD" => Ok(Operation::Add),
                "SUBTRACT" => Ok(Operation::Subtract),
                "MULTIPLY" => Ok(Operation::Multiply),
                "DIVIDE" => Ok(Operation::Divide),
                "RETURN_VALUE" => Ok(Operation::ReturnValue),
                _ => Err(OperationParsingError("Unknown operation provided".to_string()))
            }
        } else {
            match op_split[0] {
                "LOAD_VAL" => Self::parse_load_op(&op_split),
                "WRITE_VAR" => Ok(Operation::WriteVar(op_split[1].to_string())),
                "READ_VAR" => Ok(Operation::ReadVar(op_split[1].to_string())),
                _ => Err(OperationParsingError("Unknown operation provided".to_string()))
            }
        }
    }

    fn parse_load_op(op_split: &Vec<&str>) -> Result<Operation, BytecodeError> {
        let value = op_split[1].parse::<i32>();
        if value.is_ok() {
            Ok(Operation::LoadVal(value.unwrap()))
        } else {
            Err(OperationParsingError("Failed to parse Op argument (should be integer)".to_string()))
        }
    }

}
