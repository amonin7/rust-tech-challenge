
/// Structure, that represents custom error type
#[derive(Debug, PartialEq)]
pub enum BytecodeError {
    /// Used, when some exception, connected with parsing operation occurred
    OperationParsingError(String),
    /// Used, when the exception occurred during bytecode execution
    OperationProcessError(String),
}
