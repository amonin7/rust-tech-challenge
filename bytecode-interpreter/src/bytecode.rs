use crate::operation::Operation;
use crate::errors::BytecodeError;

#[derive(Debug)]
pub struct ByteCode {
    pub operations: Vec<Operation>
}

impl ByteCode {

    pub fn add_operation(&mut self, raw_operation: String) -> Result<bool, BytecodeError> {
        let op_result = Operation::parse_operation(raw_operation);
        if op_result.is_ok() {
            let op = op_result.unwrap();
            self.operations.push(op);
            return match self.operations.last().unwrap() {
                Operation::ReturnValue => Ok(true),
                _ => Ok(false)
            }
        }
        Err(op_result.unwrap_err())
    }

}