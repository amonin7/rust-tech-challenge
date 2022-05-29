#[cfg(test)]
mod e2e_tests {
    use std::collections::HashMap;
    use bytecode_interpreter::bytecode::ByteCode;
    use bytecode_interpreter::interpreter::Interpreter;
    use bytecode_interpreter::loop_info::LoopInfo;

    #[test]
    fn provided_example_testcase() {
        let program: Vec<&str> = vec![
            "LOAD_VAL 1",
            "WRITE_VAR ‘x’",
            "LOAD_VAL 2",
            "WRITE_VAR ‘y’",
            "READ_VAR ‘x’",
            "LOAD_VAL 1",
            "ADD",
            "READ_VAR ‘y’",
            "MULTIPLY",
            "RETURN_VALUE",
        ];
        let mut interpreter = initialize_interpreter(program);
        let result = interpreter.execute_code();
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn loop_testcase() {
        let program: Vec<&str> = vec![
            "LOAD_VAL 1",
            "WRITE_VAR ‘x’",
            "LOAD_VAL 2",
            "WRITE_VAR ‘y’",
            "LOOP 6",
            "READ_VAR ‘x’",
            "LOAD_VAL 1",
            "ADD",
            "WRITE_VAR ‘x’",
            "END_LOOP",
            "READ_VAR ‘x’",
            "READ_VAR ‘y’",
            "MULTIPLY",
            "RETURN_VALUE",
        ];
        let mut interpreter = initialize_interpreter(program);
        let result = interpreter.execute_code();
        assert_eq!(result.unwrap(), 14);
    }

    #[test]
    #[should_panic(expected = "OperationProcessError(\"There is already initialized loop\")")]
    fn double_loop_panic_testcase() {
        let program: Vec<&str> = vec![
            "LOAD_VAL 1",
            "WRITE_VAR ‘x’",
            "LOOP 6",
            "READ_VAR ‘x’",
            "LOOP 2",
            "LOAD_VAL 1",
            "ADD",
            "WRITE_VAR ‘x’",
            "END_LOOP",
            "READ_VAR ‘x’",
            "RETURN_VALUE",
        ];
        let mut interpreter = initialize_interpreter(program);
        let result = interpreter.execute_code();
        assert!(result.is_err());
        panic!("{:?}", result.unwrap_err())
    }

    fn initialize_interpreter(program: Vec<&str>) -> Interpreter {
        let mut bc = ByteCode { operations: Vec::new() };
        for op in program {
            let operation_result = bc.add_operation(op.to_string());
            if operation_result.unwrap() {
                break;
            }
        }
        let interpreter = Interpreter {
            bc,
            stack: Vec::new(),
            variables: HashMap::new(),
            loop_info: LoopInfo { start_op_index: 0, end_op_index: 0, iterations_num: 0, has_loop: false }
        };
        return interpreter;
    }
}