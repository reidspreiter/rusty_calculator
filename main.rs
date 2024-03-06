use std::io;
use std::process::ExitCode;
use std::collections::HashMap;
mod tokenize;
mod infix_to_postfix;
mod evaluate;
mod command;
mod test;

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
                        let result = command::execute_command(equation);
                        if result == 1 {
                            println!("Thank you for using Rusty Calculator. Goodbye!");
                            return ExitCode::SUCCESS;
                        }
                    } else {
                        let tokens = tokenize::tokenize(equation, &variables);
                        println!("Tokens: {:?}", tokens);
                        let expression = infix_to_postfix::infix_to_postfix(tokens);
                        println!("Expression: {:?}", expression);
                        match evaluate::evaluate(expression) {
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
