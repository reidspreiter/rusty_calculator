use std::io;
use std::process::ExitCode;
mod tokenize;
mod infix_to_postfix;
mod evaluate;
mod command;
mod test;
mod complex_evaluate;

fn main() -> ExitCode {
    let mut variables = command::get_variable_map();
    let mut settings = command::get_settings_map();
    
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
                    if char == '/' || char == '_' {
                        command::execute_command(equation, &variables, &mut settings);
                        if settings["quit"] {
                            println!("Thank you for using Rusty Calculator. Goodbye!");
                            return ExitCode::SUCCESS;
                        }
                    } else {
                        match tokenize::tokenize(equation, &variables) {
                            Ok(tokens) => {
                                if settings["reveal"] {
                                    println!("Tokens: {:?}", tokens);
                                }
                                let expression = infix_to_postfix::infix_to_postfix(tokens);
                                if settings["reveal"] {
                                    println!("Expression: {:?}", expression);
                                }
                                match evaluate::evaluate(expression) {
                                    Ok(result) => {
                                        println!("Result: {}", result);
                                        let answer = result.to_string();
                                        variables.insert('=', answer.clone());
                                    },
                                    Err(err) => println!("Error: {}", err),
                                }
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
