#![allow(dead_code)]

use std::io::{self, Write};
// 일반적인 구조체
struct Foo {
    test: i32,
    id: i32,
    name: String,
}

// unit struct
struct Calculator;

// tuple struct
struct Point(i32, i32);

impl Calculator {
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    fn mutiply(a: f64, b: f64) -> f64 {
        a * b
    }

    fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
        if b == 0.0 {
            Err("Cannot divide by zero")
        } else {
            Ok(a / b)
        }
    }
}

fn main() {
    let mut input = String::new();

    print!("Enter first number:");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let a: f64 = input.trim().parse().expect("숫자가 들어가지 않았습니다.");
    input.clear();

    print!("Enter second number:");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("숫자가 들어가지 않았습니다.");
    input.clear();

    print!("Enter operation (+,-,*,/):");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let operation = input.trim();

    let result = match operation {
        "+" => Ok(Calculator::add(a, b)),
        "-" => Ok(Calculator::subtract(a, b)),
        "*" => Ok(Calculator::mutiply(a, b)),
        "/" => Calculator::divide(a, b),
        _ => Err("Invalid operation"),
    };

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => eprintln!("Error: {}", e),
    }
}
