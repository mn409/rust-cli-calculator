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

                let mut a = String::new();
                println!("Enter first number:");
                stdin().read_line(&mut a).unwrap();

                let a: i32 = match a.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };

                let mut b = String::new();
                println!("Enter second number:");
                stdin().read_line(&mut b).unwrap();

                let b: i32 = match b.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };

                println!("Result: {}", a + b);
            }

            "sub" => {
                println!("You chose sub");

                let mut a = String::new();
                println!("Enter first number:");
                stdin().read_line(&mut a).unwrap();

                let a: i32 = match a.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };

                let mut b = String::new();
                println!("Enter second number:");
                stdin().read_line(&mut b).unwrap();

                let b: i32 = match b.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };

                println!("Result: {}", a - b);
            }

            _ => {
                println!("Invalid operation");
            }
        }
    }
}