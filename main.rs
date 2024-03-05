use std::io;
use std::process::ExitCode;
use std::collections::HashMap;

const OPERATORS: [&str; 9] = ["+", "-", "*", "/", "^", "%", "#", "(", "\\"];

// Tokenize User Equation into individual Strings
fn tokenize(equation: &str, variable_map: &HashMap<char, String>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut number_buffer = String::new();

    for c in equation.chars() {
        match c {
            '0'..='9' | '.' | 'E' => number_buffer.push(c),
            _ => {
                if let Some(value) = variable_map.get(&c) {
                    if number_buffer != "-" && !number_buffer.is_empty() {
                        tokens.push(number_buffer.clone());
                        number_buffer.clear();
                    }
                    number_buffer.push_str(&value.to_string())
                }
                if !number_buffer.is_empty() {
                    if number_buffer == "-" {
                        number_buffer = "~".to_string();
                    }
                    if let Some(last) = tokens.last() {
                        match last.as_str() {
                            ")" => tokens.push("*".to_string()),
                            _ => {
                                if let Ok(_) = last.as_str().parse::<f64>() {
                                    tokens.push("*".to_string());
                                } else {}
                            },
                        }
                    }
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
                        match &tokens.last() {
                            Some(last) if OPERATORS.contains(&last.as_str()) => {
                                number_buffer.push(c);
                            }
                            None => number_buffer.push(c), 
                            _ => {
                                tokens.push(c.to_string());
                            }
                        }
                    },
                    '*' => tokens.push(c.to_string()),
                    '/' => {
                        if let Some(last) = tokens.last() {
                            match last.as_str() {
                                "/" => {
                                    if let Some(last) = tokens.last_mut() {
                                        *last = "#".to_string();
                                    }
                                },
                                _ => tokens.push(c.to_string()),
                            }
                        } else {
                            tokens.push(c.to_string());
                        }
                    },
                    '%' => {
                        if let Some(last) = tokens.last() {
                            match last.as_str() {
                                "%" => {
                                    if let Some(last) = tokens.last_mut() {
                                        *last = "\\".to_string();
                                    }
                                },
                                _ => tokens.push(c.to_string()),
                            }
                        } else {
                            tokens.push(c.to_string());
                        }
                    },
                    '#' => tokens.push(c.to_string()),
                    '\\' => tokens.push(c.to_string()),
                    '^' => tokens.push(c.to_string()),
                    '(' => {
                        if let Some(last) = tokens.last() {
                            match last.as_str() {
                                "~" => {
                                    if let Some(last) = tokens.last_mut() {
                                        *last = "-1".to_string();
                                        tokens.push("*".to_string());
                                    }
                                },
                                ")" => tokens.push("*".to_string()),
                                _ => {
                                    if let Ok(_) = last.as_str().parse::<f64>() {
                                        tokens.push("*".to_string());
                                    } else {}
                                },
                            }
                        }
                        tokens.push(c.to_string());
                    },
                    ')' => tokens.push(c.to_string()),
                    _ => {
                        if c != ' ' && !variable_map.contains_key(&c) {
                            println!("'{}' is not a valid character. Solving without {}.", c, c);
                        }
                    }
                }
            }
        }
    }
    if !number_buffer.is_empty() {
        if number_buffer == "-" {
            number_buffer = "~".to_string();
        }
        if let Some(last) = tokens.last() {
            match last.as_str() {
                ")" => tokens.push("*".to_string()),
                _ => {
                    if let Ok(_) = last.as_str().parse::<f64>() {
                        tokens.push("*".to_string());
                    } else {}
                },
            }
        }
        tokens.push(number_buffer.clone());
        number_buffer.clear();
    }
    tokens
}

// Order of operations precedence for converting infix to postfix
fn precedence(operator: &str) -> i8 {
    match operator {
        "^" => 3,
        "*" | "/" | "%" | "#" => 2,
        "\\" => 1,
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
            "+" | "-" | "*" | "/" | "^" | "%" | "#" | "\\" => {
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
            "+" | "-" | "*" | "/" | "^" | "%" | "#" | "\\" => {
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
                        "\\" => (a / 100.0) * b,
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
    let mut variables = HashMap::new();
    variables.insert('p', "3.14159265359".to_string());
    variables.insert('e', "2.71828182845".to_string());
    variables.insert('=', "0".to_string());
    variables.insert('i', "1".to_string());
    variables.insert('j', "1".to_string());
    variables.insert('k', "1".to_string());
    variables.insert('l', "1".to_string());
    variables.insert('m', "1".to_string());
    variables.insert('n', "1".to_string());
    variables.insert('o', "1".to_string());
    
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
                        let tokens = tokenize(equation, &variables);
                        println!("Tokens: {:?}", tokens);
                        let expression = infix_to_postfix(tokens);
                        println!("Expression: {:?}", expression);
                        match evaluate(expression) {
                            Ok(result) => {
                                println!("Result: {}", result);
                                let answer = result.to_string();
                                variables.insert('=', answer.clone());
                            },
                            Err(err) => println!("Error: {}", err),
                        }
                    }
                },
                None => {},
            }
        }
    }
}
