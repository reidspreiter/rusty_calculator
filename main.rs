use std::io;
use std::process::ExitCode;

const OPERATORS: [&str; 7] = ["+", "-", "*", "/", "^", "%", "#"];

// Tokenize User Equation into individual Strings
fn tokenize(equation: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut number_buffer = String::new();

    for c in equation.chars() {
        match c {
            '0'..='9' | '.' | 'E' => number_buffer.push(c),
            _ => {
                if !number_buffer.is_empty() {
                    tokens.push(number_buffer.clone());
                    number_buffer.clear();
                }
                match c {
                    '+' => {
                        if let Some(last) = tokens.last() {
                            if !OPERATORS.contains(&last.as_str()) {
                                tokens.push(c.to_string());
                            }
                        } else {
                            tokens.push(c.to_string());
                        }
                    },
                    '-' => {
                        // FIXME: -8+8 and -(8+8)
                        if let Some(last) = tokens.last() {
                            if OPERATORS.contains(&last.as_str()) {
                                number_buffer.push(c);
                            } else {
                                tokens.push(c.to_string())
                            }
                        } else {
                            tokens.push(c.to_string());
                        }
                    },
                    '*' => tokens.push(c.to_string()), // FIXME: add all the correct multiplication scenarios
                    '/' => tokens.push(c.to_string()), // FIXME: // = #
                    '%' => tokens.push(c.to_string()),
                    '#' => tokens.push(c.to_string()),
                    '^' => tokens.push(c.to_string()),
                    '(' => tokens.push(c.to_string()),
                    ')' => tokens.push(c.to_string()),
                    _ => {
                        println!("'{}' is not a valid character. Solving without {}.", c, c);
                    }
                }
            }
        }
    }
    if !number_buffer.is_empty() {
        tokens.push(number_buffer.clone());
        number_buffer.clear();
    }
    tokens
}

// Order of operations precedence for converting infix to postfix
fn precedence(operator: &str) -> i8 {
    match operator {
        "^" => 3,
        "~" => 2,
        "*" | "/" | "%" | "#" => 1,
        "+" | "-" => 0,
        _ => -1,
    }
}

// Convert infix to postfix
fn infix_to_postfix(tokens: Vec<String>) -> Vec<String> {
    let mut postfix_expression: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" | "^" | "%" | "#" => {
                while let Some(top) = stack.last() {
                    if top == "(" || precedence(&top) < precedence(token.as_str()) {
                        break;
                    }
                    if let Some(popped) = stack.pop() {
                        postfix_expression.push(popped);
                    }
                }
                stack.push(token);
            }
            "(" => stack.push(token),
            ")" => {
                while let Some(top) = stack.pop() {
                    if top == "(" {
                        break;
                    }
                    postfix_expression.push(top);
                }
            }
            _ => postfix_expression.push(token),
        }
    }

    while let Some(token) = stack.pop() {
        postfix_expression.push(token);
    }
    postfix_expression
}


// Evaluate postfix expression
fn evaluate(expression: Vec<String>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expression {
        match token.as_str() {
            "+" | "-" | "*" | "/" | "^" | "%" | "#" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let result = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => {
                            if b == 0.0 {
                                let error_message = format!("Divide by Zero {} / {}", a, b);
                                return Err(error_message);
                            }
                            a / b
                        },
                        "^" => a.powf(b),
                        "%" => a % b,
                        "#" => (a / b).floor(),
                        operator => {
                            let error_message = format!("Invalid Operator {}", operator);
                            return Err(error_message);
                        }
                    };
                    stack.push(result);
                } else {
                    return Err("Not enough operands".to_string());
                }
            }
            operand => {
                if let Ok(num) = operand.parse::<f64>() {
                    stack.push(num);
                } else {
                    let error_message = format!("Invalid Operator {}", operand);
                    return Err(error_message);
                }
            }
        }
    }

    if let Some(result) = stack.pop() {
        if stack.is_empty() {
            Ok(result)
        } else {
            Err("Too many operands".to_string())
        }
    } else {
        Err("Empty expression".to_string())
    }
}

// Execute commands
fn execute_command(command: &str) -> i8 {
    println!("matching {}", command);
    match command {
        "/help" => println!("Help"),
        "/reset" => println!("Reset"),
        "/quit" => return 1,
        _ => println!("{} is not a valid command.", command)
    }
    0
}

fn main() -> ExitCode {
    println!("Welcome to Rusty Calculator. Enter your equations below:");
    loop {
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("Failed to read line");
        if text.ends_with('\n') {
            text.pop();
            if text.ends_with('\r') {
                text.pop();
            }
        }
        let equations: Vec<&str> = text.split(";").collect();

        for equation in equations {
            let trimmed_eq = equation.trim();
            let first_char = trimmed_eq.chars().nth(0);
            match first_char {
                Some(char) => {
                    if char == '/' {
                        let result = execute_command(equation);
                        if result == 1 {
                            println!("Thank you for using Rusty Calculator. Goodbye!");
                            return ExitCode::SUCCESS;
                        }
                    } else {
                        let tokens = tokenize(equation);
                        println!("Tokens: {:?}", tokens);
                        let expression = infix_to_postfix(tokens);
                        println!("Expression: {:?}", expression);
                        match evaluate(expression) {
                            Ok(result) => println!("Result: {}", result),
                            Err(err) => println!("Error: {}", err),
                        }
                    }
                },
                None => {},
            }
        }
    }
}