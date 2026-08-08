use std::io::stdin;

pub fn get_number(message: &str) -> i32 {
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