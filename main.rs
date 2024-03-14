use std::io;
use std::io::Write;
use std::process::ExitCode;
mod tokenize;
mod infix_to_postfix;
mod evaluate;
mod command;
mod test;
mod complex_evaluate;

// Read operation from user and solve or call command respectively.
fn main() -> ExitCode {
    let mut variables = command::get_variable_map();
    let mut settings = command::get_settings_map();
    println!("Welcome to Rusty Calculator. Enter your equations below:");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();
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
            if let Some(first) = trimmed_eq.chars().nth(0) {
                match first {
                    '/' => command::execute_command(trimmed_eq, &mut variables, &mut settings),
                    '_' => command::change_variable(trimmed_eq, &mut variables),
                    _ => {
                        match tokenize::tokenize(trimmed_eq, &variables) {
                            Ok(tokens) => {
                                let expression = infix_to_postfix::infix_to_postfix(&tokens);
                                if settings["reveal"] {
                                    println!("Tokens: {:?}", tokens);
                                    println!("Expression: {:?}", expression);
                                }

                                match evaluate::evaluate(expression) {
                                    Ok(result) => {
                                        println!("Result: {}", result);
                                        variables.insert('=', result.to_string());
                                    },
                                    Err(e) => println!("Error: {}", e),
                                }
                            },
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                }
            }
        }

        if settings["quit"] {
            println!("Thank you for using Rusty Calculator. Goodbye!");
            return ExitCode::SUCCESS;
        }
    }
}