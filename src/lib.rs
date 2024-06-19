


#[derive(Debug)]
pub struct CustomError {
    message: String,
}


impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CustomError {}

// Function that divides two numbers, returning a Result
pub fn safe_divide(a: i32, b: i32) -> Result<i32, CustomError> {
    if b == 0 {
        return Err(CustomError {
            message: "Division by zero".to_string(),
        });
    }

    Ok(a / b)
}
