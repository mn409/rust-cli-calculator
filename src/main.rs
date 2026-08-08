use std::io::stdin;
use std::collections::HashMap;

mod input;
mod operations;

use input::get_number;
use operations::math::*;
use operations::types::OperationType;

fn main() {

    let mut operations: HashMap<&str, OperationType> = HashMap::new();

    operations.insert("add", OperationType::Binary(add));
    operations.insert("sub", OperationType::Binary(sub));
    operations.insert("mul", OperationType::Binary(mul));
    operations.insert("div", OperationType::Binary(div));
    operations.insert("square", OperationType::Unary(square));
    operations.insert("sqrt", OperationType::Unary(sqrt));


    loop {
        let mut operation = String::new();

        println!("Enter operation (add / sub / mul / div / square / sqrt / exit):");
        stdin().read_line(&mut operation).unwrap();
        let operation = operation.trim();

        if operation == "exit" {
            println!("Exiting...");
            break;
        }

        let op = operations.get(operation);

        match op {
            Some(operation_type) => {
                match operation_type {

                    OperationType::Unary(f) => {
                        let a = get_number("Enter number:");
                        match f(a) {
                            Ok(value) => println!("Result: {}", value),
                            Err(e) => println!("Error: {}", e),
                        }
                    }

                    OperationType::Binary(f) => {
                        let a = get_number("Enter first number:");
                        let b = get_number("Enter second number:");

                        if operation == "div" && b == 0 {
                            println!("Cannot divide by zero");
                            continue;
                        }

                        match f(a, b) {
                            Ok(value) => println!("Result: {}", value),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                }
            }

            None => {
                println!("Invalid operation");
            }
        }
    }
}