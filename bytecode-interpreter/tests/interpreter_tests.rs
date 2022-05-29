#[cfg(test)]
mod op_tests {
    use std::collections::HashMap;
    use bytecode_interpreter::bytecode::ByteCode;
    use bytecode_interpreter::errors::BytecodeError;
    use bytecode_interpreter::interpreter::Interpreter;
    use bytecode_interpreter::loop_info::LoopInfo;

    #[test]
    fn simple_correct_testcase() {
        let mut bc = ByteCode { operations: Vec::new() };
        let _ = bc.add_operation("LOAD_VAL 7".to_string());
        let _ = bc.add_operation("RETURN_VALUE".to_string());
        let res = get_result(bc);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 7);
    }

    #[test]
    fn empty_stack_testcase() {
        let bc = ByteCode { operations: Vec::new() };
        let res = get_result(bc);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), BytecodeError::OperationProcessError("There is nothing to return - the stack is empty.".to_string()));
    }

    #[test]
    fn nested_loops_testcase() {
        let mut bc = ByteCode { operations: Vec::new() };
        let _ = bc.add_operation("LOOP 7".to_string());
        let _ = bc.add_operation("LOOP 7".to_string());
        let res = get_result(bc);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), BytecodeError::OperationProcessError("There is already initialized loop".to_string()));
    }

    #[test]
    fn no_loop_testcase() {
        let mut bc = ByteCode { operations: Vec::new() };
        let _ = bc.add_operation("END_LOOP".to_string());
        let res = get_result(bc);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), BytecodeError::OperationProcessError("There is no initialized loop yet".to_string()));
    }

    #[test]
    fn write_var_failure_testcase() {
        let mut bc = ByteCode { operations: Vec::new() };
        let _ = bc.add_operation("WRITE_VAR x".to_string());
        let res = get_result(bc);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), BytecodeError::OperationProcessError("There is no variable in the stack".to_string()));
    }

    #[test]
    fn arithmetic_failure_not_enough_vars_testcase0() {
        let mut bc = ByteCode { operations: Vec::new() };
        let _ = bc.add_operation("ADD".to_string());
        let res = get_result(bc);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), BytecodeError::OperationProcessError("There stack size is less than 2 and op could not be performed".to_string()));
    }

    #[test]
    fn arithmetic_failure_not_enough_vars_testcase1() {
        let mut bc = ByteCode { operations: Vec::new() };
        let _ = bc.add_operation("LOAD_VAL 1".to_string());
        let _ = bc.add_operation("ADD".to_string());
        let res = get_result(bc);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), BytecodeError::OperationProcessError("There stack size is less than 2 and op could not be performed".to_string()));
    }

    fn get_result(bc: ByteCode) -> Result<i32, BytecodeError> {
        let mut interpreter = Interpreter {
            bc,
            stack: Vec::new(),
            variables: HashMap::new(),
            loop_info: LoopInfo { start_op_index: 0, end_op_index: 0, iterations_num: 0, has_loop: false }
        };
        return interpreter.execute_code();
    }
}