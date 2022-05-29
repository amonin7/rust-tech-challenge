use std::collections::HashMap;
use std::io;
use bytecode_interpreter::bytecode::ByteCode;
use bytecode_interpreter::interpreter::Interpreter;
use bytecode_interpreter::loop_info::LoopInfo;

fn main() {

    let mut bc = ByteCode { operations: Vec::new() };
    read_input(&mut bc);

    let mut interpreter = Interpreter {
        bc,
        stack: Vec::new(),
        variables: HashMap::new(),
        loop_info: LoopInfo { start_op_index: 0, end_op_index: 0, iterations_num: 0, has_loop: false }
    };

    let result = interpreter.execute_code();
    if result.is_err() {
        panic!("Failed to process instructions: {:?}", result.unwrap_err())
    }
    println!("Result is {}", result.unwrap())
}

fn read_input(bc: &mut ByteCode) {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let operation_result = bc.add_operation(input);
                if operation_result.is_err() {
                    println!("failed to process operation - {:?}", operation_result.unwrap_err());
                } else if operation_result.unwrap() {
                    break;
                }
            }
            Err(e) => panic!("Failed to read input {:?}", e)
        }
    }
}