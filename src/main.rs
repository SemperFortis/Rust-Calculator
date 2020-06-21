use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");

    stdin().read_line(input).expect("failed to read");
}

fn calculate(first: f32, second: f32, operator: char) -> f32 {
    if operator == '+' {
        first + second
    } else if operator == '-' {
        first - second
    } else if operator == '/' {
        first / second
    } else if operator == '*' {
        first * second
    } else {
        0.0
    }
}

fn main() {
    println!("Basic Calculator");
    println!("----------------");

    // User inputs are strings
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    // First input
    print!("Please enter the first number: ");
    read(&mut num1);

    // Operator
    print!("Please enter the operation you would like to do: ");
    read(&mut operator);

    // Check if the operator is valid before continuing
    let operator: char = operator.trim().chars().next().unwrap();
    let operators: [char; 4] =  ['+', '-', '/', '*'];

    if !operators.contains(&operator) {
        println!("Error, unknown operator.");
        return;
    }

    // Second number
    print!("Please enter the second number: ");
    read(&mut num2);

    // Convert user input to float 32s
    let num1: f32 = num1.trim().parse().unwrap();
    let num2: f32 = num2.trim().parse().unwrap();

    println!("Your result for the operation {} {} {} is {}", num1, operator, num2, calculate(num1, num2, operator));
}