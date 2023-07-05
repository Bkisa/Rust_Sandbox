use std::io;

fn main() {
    println!("Enter an operator (+, -, *, /):");

    let mut operator_input = String::new();

    match io::stdin().read_line(&mut operator_input) {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    let operator = match operator_input.trim() {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Enter the first number:");
    let num1_input = match read_line().ok().and_then(|v| v.trim().parse().ok()) {
        Some(value) => value,
        None => {
            println!("Invalid input!");
            return;
        }
    };

    println!("Enter the second number:");
    let num2_input = match read_line().ok().and_then(|v| v.trim().parse().ok()) {
        Some(value) => value,
        None => {
            println!("Invalid input!");
            return;
        }
    };

    if let Some(result) = calculate(operator, num1_input, num2_input) {
        println!("Result: {}", result);
    } else {
        println!("Cannot divide by zero!");
    }
}

fn read_line() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input)
}

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operator: Operator, num1: f64, num2: f64) -> Option<f64> {
    match operator {
        Operator::Add => Some(num1 + num2),
        Operator::Subtract => Some(num1 - num2),
        Operator::Multiply => Some(num1 * num2),
        Operator::Divide => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                None
            }
        }
    }
}
