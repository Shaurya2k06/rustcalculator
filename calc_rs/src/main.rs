use std::io::{self, Write};

fn main() {
    println!("Welcome to Shaurya's Calculator");
    println!("To get started, write an expression (e.g., '5 + 3')");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Successful Exit");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Please enter in format: number operator number");
            continue;
        }

        let left: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", parts[0]);
                continue;
            }
        };

        let op = parts[1];

        let right: f64 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: {}", parts[2]);
                continue;
            }
        };

        match calculate(left, op, right) {
            Ok(result) => println!("Result: {}\n", result),
            Err(err) => println!("Error: {}\n", err),
        }
    }
}

fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Unsupported operator '{}'", op)),
    }
}