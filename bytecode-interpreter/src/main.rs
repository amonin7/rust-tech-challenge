use std::collections::HashMap;
use std::io;
use crate::bytecode::ByteCode;
use crate::interpreter::Interpreter;

mod bytecode;
mod operation;
mod errors;
mod interpreter;

fn main() {

    let mut bc = ByteCode { operations: Vec::new() };
    read_input(&mut bc);

    let mut interpreter = Interpreter {
        bc,
        stack: Vec::new(),
        variables: HashMap::new()
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