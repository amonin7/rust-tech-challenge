use crate::operation::Operation;
use crate::errors::BytecodeError;

/// The structure represents all the bytecode, which was passed as the input
#[derive(Debug)]
pub struct ByteCode {
    /// List of all operations
    pub operations: Vec<Operation>
}

impl ByteCode {

    /// Adds a single operation into operations list \
    /// In case of failure - returns an error \
    /// Otherwise returns the boolean value, representing whether the given operation was terminating one
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