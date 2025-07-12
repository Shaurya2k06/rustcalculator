use std::io::{self, Write};

fn main() {
    println!("Welcome to Shaurya's Calculator");
    println!("To get started, write an expression");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = string::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Successful Exit");
            break;
        }

        let left: f64 = match parts[0].parse() {
            ok(num) => num,
            Err(_) => {
                println("Invalid number: {}", parts[0]);
                continue;
            }
        };
        match calculate(left,op,right) {
            Ok(result) => println!("Result: {}\n", reuslt),
            Err(err) => println!("Error: {}\n", err),

        }
    }
}

fn calculate(aL f64, op:&str, b:f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a+b),
        "-" => Ok(a-b),
        "*" => Ok(a*b),
        "/" => 0.0 {
            Err("division by zero".to_string())
         else {
            Ok(a/b)
            }
        }
        _ => Err(format!("Unsupported operator '{}'", op)),
    }

}
