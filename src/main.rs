use std::io::stdin;
use std::collections::HashMap;

fn main() {

    let mut operations: HashMap<&str, OperationType> = HashMap::new();

    operations.insert("add", OperationType::Binary(add));
    operations.insert("sub", OperationType::Binary(sub));
    operations.insert("mul", OperationType::Binary(mul));
    operations.insert("div", OperationType::Binary(div));
    operations.insert("square", OperationType::Unary(square));

    loop {
        let mut operation = String::new();

        println!("Enter operation (add / sub / mul / div / square / exit):");
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

                    // 🔥 Unary case
                    OperationType::Unary(f) => {
                        let a = get_number("Enter number:");
                        let result = f(a);
                        println!("Result: {}", result);
                    }

                    OperationType::Binary(f) => {
                        let a = get_number("Enter first number:");
                        let b = get_number("Enter second number:");

                        // Special handling for division
                        if operation == "div" && b == 0 {
                            println!("Cannot divide by zero");
                            continue;
                        }

                        let result = f(a, b);
                        println!("Result: {}", result);
                    }
                }
            }

            None => {
                println!("Invalid operation");
            }
        }
    }
}

fn get_number(message: &str) -> i32 {
    loop {
        let mut input = String::new();

        println!("{}", message);
        stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("Invalid input");
            }
        }
    }
}


fn add(a: i32, b: i32) -> f32 {
    (a + b) as f32
}

fn sub(a: i32, b: i32) -> f32 {
    (a - b) as f32
}

fn mul(a: i32, b: i32) -> f32 {
    (a * b) as f32
}

fn div(a: i32, b: i32) -> f32 {
    (a as f32) / (b as f32)
}

fn square(a: i32) -> f32 {
    (a * a) as f32
}


enum OperationType {
    Unary(fn(i32) -> f32),
    Binary(fn(i32, i32) -> f32),
}