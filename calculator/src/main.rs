use std::io;
mod operations;

fn main() {
    println!("Simple Calculator");
    println!("Enter an expression (e.g., 2 + 2):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    if tokens.len() != 3 {
        println!("Invalid input format. Please use: number operator number");
        return;
    }

    let a: f64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid first number.");
            return;
        }
    };

    let b: f64 = match tokens[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid second number.");
            return;
        }
    };

    let result = match tokens[1] {
        "+" => Ok(operations::add(a, b)),
        "-" => Ok(operations::subtract(a, b)),
        "*" => Ok(operations::multiply(a, b)),
        "/" => operations::divide(a, b),
        _ => {
            println!("Unknown operator.");
            return;
        }
    };

    match result {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
