//calculator functions

//Add two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

//Subtract two numbers
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

//Multiply two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

//Divide two numbers
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

pub fn power(a: i32, b: i32) -> i32 {
    a.pow(b.try_into().unwrap())
}

pub fn squareroot(a: f64) -> f64 {
    a.sqrt()
}