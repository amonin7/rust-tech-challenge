#[derive(Debug)]
pub enum BytecodeError {
    OperationParsingError(String),
    OperationProcessError(String),
}
