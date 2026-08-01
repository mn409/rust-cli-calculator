use std::io::stdin;

fn main() {
    let mut operation = String::new();
    println!("Enter operation (add/ sub/ mul/ div): ");
    
    stdin().read_line(&mut operation).unwrap();
    let operation = operation.trim();

    match operation {
        "add" => {
            println!("You choose add");
        }
        "sub" => {
            println!("You choose sub");
        }
        _ => {
            println!("Invalid operation");
        }
    }
}
