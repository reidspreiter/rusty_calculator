use crate::test;
use crate::tokenize::tokenize;
use crate::infix_to_postfix::infix_to_postfix;
use crate::evaluate::evaluate;
use std::collections::HashMap;

pub fn get_variable_map() -> HashMap<char, String> {
    let variables: HashMap<char, String> = [
        ('p', "3.14159265359".to_string()),
        ('e', "2.71828182845".to_string()),
        ('=', "0".to_string()),
        ('i', "1".to_string()),
        ('j', "1".to_string()),
        ('k', "1".to_string()),
        ('l', "1".to_string()),
        ('m', "1".to_string()),
        ('n', "1".to_string()),
        ('o', "1".to_string()),
    ].iter().cloned().collect();
    variables
}

pub fn get_settings_map() -> HashMap<String, bool> {
    let settings: HashMap<String, bool> = [
        ("reveal".to_string(), false),
        ("quit".to_string(), false),
    ].iter().cloned().collect();
    settings
}


// Execute commands
pub fn execute_command(command: &str, variables: &mut HashMap<char, String>, settings: &mut HashMap<String, bool>) {
    match command {
        "/help" => help(),
        "/reset" => {
            *variables = get_variable_map();
            *settings = get_settings_map();
            println!("Variables and settings reset successfully")
        },
        "/variables" => {
            for (key, value) in variables {
                println!("{}: {}", key, value);
            }
        },
        "/x" => { settings.insert("quit".to_string(), true); },
        "/test" => { test::test(); },
        "/reveal" => { 
            settings.insert("reveal".to_string(), !settings["reveal"]); 
            println!("Reveal changed to {}", settings["reveal"]);
        },
        "/op" => {},
        "/varop" => {},
        "/complexop" => {},
        _ => println!("{} is not a valid command. Try /help.", command)
    }
}

pub fn change_variable(command: &str, variables: &mut HashMap<char, String>) {
    if let Some(variable) = command.chars().nth(1) {
        match variable {
            'p' | 'e' | '=' => println!("{} is not a mutable variable", variable),
            'i'..='o' => {
                let equation = &command[2..];
                match tokenize(equation, variables) {
                    Ok(result) => {
                        match evaluate(infix_to_postfix(&result)) {
                            Ok(result) => { 
                                variables.insert(variable, result.to_string());
                                println!("'{}' value changed to {}", variable, result);
                            },
                            Err(e) => { println!("Unable to evaluate new value: {}", e); },
                        }
                    },
                    Err(e) => println!("Unable to tokenize new value: {}", e),
                }
            },
            _ => println!("{} is not a valid variable", variable)
        }
    }
}

fn help() {
    println!("HELP:");
    println!("/x         -> Quit calculator");
    println!("/reset     -> Reset variable and setting values to default");
    println!("/variables -> Print all current variable values");
    println!("/test      -> Run all test cases");
    println!("/reveal    -> Toggle reveal on and off. When reveal is on, separate tokens and postfix");
    println!("              expressions will be printed alongside the result for each user entered equation");
    println!("/op        -> Prints basic operator usage information");
    println!("/varop     -> Prints variable usage information");
    println!("/complexop -> Prints complex operator usage information");
}