#[cfg(test)]
mod bc_tests {
    use bytecode_interpreter::bytecode::ByteCode;

    #[test]
    fn correct_non_return_testcase() {
        let mut bc = initialize_bc();
        let result = bc.add_operation("LOAD_VAL 7".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn correct_return_testcase() {
        let mut bc = initialize_bc();
        let result = bc.add_operation("RETURN_VALUE".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    #[should_panic(expected = "OperationParsingError(\"Unknown operation provided\")")]
    fn incorrect_testcase() {
        let mut bc = initialize_bc();
        let result = bc.add_operation("SHALABALA".to_string());
        assert!(result.is_err());
        panic!("{:?}", result.unwrap_err())
    }

    fn initialize_bc() -> ByteCode {
        return ByteCode { operations: Vec::new() }
    }
}