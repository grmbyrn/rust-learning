#[derive(Debug)]
pub enum CalcError {
    InvalidInput,
    UnknownOperator,
    DivideByZero,
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::InvalidInput => write!(f, "Invalid input format. Please use: number operator number"),
            CalcError::UnknownOperator => write!(f, "Unknown operator."),
            CalcError::DivideByZero => write!(f, "Cannot divide by zero"),
        }
    }
}

impl std::error::Error for CalcError {}