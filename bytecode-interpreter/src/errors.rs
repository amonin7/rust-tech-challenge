#[derive(Debug, PartialEq)]
pub enum BytecodeError {
    OperationParsingError(String),
    OperationProcessError(String),
}
