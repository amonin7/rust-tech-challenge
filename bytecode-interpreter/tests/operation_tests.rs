#[cfg(test)]
mod op_tests {
    use bytecode_interpreter::operation::Operation;

    #[test]
    fn all_instructions_testcase() {
        let res = Operation::parse_operation("LOAD_VAL 7".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::LoadVal(7));

        let res = Operation::parse_operation("LOOP 7".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::Loop(7));

        let res = Operation::parse_operation("READ_VAR x".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::ReadVar('x'.to_string()));

        let res = Operation::parse_operation("WRITE_VAR x".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::WriteVar('x'.to_string()));

        let res = Operation::parse_operation("ADD".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::Add);

        let res = Operation::parse_operation("SUBTRACT".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::Subtract);

        let res = Operation::parse_operation("MULTIPLY".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::Multiply);

        let res = Operation::parse_operation("DIVIDE".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::Divide);

        let res = Operation::parse_operation("END_LOOP".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::EndLoop);

        let res = Operation::parse_operation("RETURN_VALUE".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Operation::ReturnValue);
    }

    #[test]
    #[should_panic(expected = "OperationParsingError(\"Unknown operation provided\")")]
    fn unsupported_op_testcase() {
        let res = Operation::parse_operation("SHALABALA".to_string());
        assert!(res.is_err());
        panic!("{:?}", res.unwrap_err());
    }

}