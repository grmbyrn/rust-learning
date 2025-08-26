use std::io;
use crate::operations;
use crate::errors::CalcError;

pub fn run() -> Result<(), CalcError> {
    println!("Simple Calculator");
    println!("Enter an expression (e.g., 2 + 2):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| CalcError::InvalidInput)?;

    let mut input = input.trim().to_string();
    for op in ["+", "-", "*", "/"] {
        input = input.replace(op, &format!(" {} ", op));
    }
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() != 3 {
        return Err(CalcError::InvalidInput);
    }

    let a: f64 = tokens[0].parse().map_err(|_| CalcError::InvalidInput)?;
    let b: f64 = tokens[2].parse().map_err(|_| CalcError::InvalidInput)?;

    let result = match tokens[1] {
        "+" => Ok(operations::add(a, b)),
        "-" => Ok(operations::subtract(a, b)),
        "*" => Ok(operations::multiply(a, b)),
        "/" => operations::divide(a, b).map_err(|_| CalcError::DivideByZero),
        _ => Err(CalcError::UnknownOperator),
    }?;

    println!("Result: {}", result);
    Ok(())
}
