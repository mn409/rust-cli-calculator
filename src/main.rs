use std::io::stdin;
fn main() {
    loop {
        let mut operation = String::new();
        println!("Enter operation (add / sub / mul / div / exit):");

        stdin().read_line(&mut operation).unwrap();
        let operation = operation.trim();

        if operation == "exit" {
            println!("Exiting...");
            break;
        }

        match operation {
            "add" => {
                println!("You chose add");

                let a = get_number("Enter first number:");
                let b = get_number("Enter second number:");

                println!("Result: {}", a + b);
            }

            "sub" => {
                println!("You chose sub");

                let a = get_number("Enter first number:");
                let b = get_number("Enter second number:");

                println!("Result: {}", a - b);
            }

            "mul" => {
                println!("You chose mul");
                
                let a = get_number("Enter first number:");
                let b = get_number("Enter second number:");

                println!("Result: {}", a * b);
            }

            "div" => {
                println!("You chose div");
                
                let a = get_number("Enter first number:");
                loop {
                    let b = get_number("Enter second number:");
                    if b == 0 {
                        println!("Not divisible by 0, provide another number");
                        continue;
                    } else {
                        println!("Result: {}", (a as f32) / (b as f32));
                        break;
                    }
                }
            }
                
            _ => {
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
                continue;
            }
        };
    }
}