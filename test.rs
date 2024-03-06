use std::collections::HashMap;
use crate::tokenize::tokenize;
use crate::infix_to_postfix::infix_to_postfix;
use crate::evaluate::evaluate;

fn start_to_finish(text: &str) -> String {
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

    let equations: Vec<&str> = text.split(";").collect();
    for equation in equations {
        let trimmed_eq = equation.trim();
        let first_char = trimmed_eq.chars().nth(0);
        match first_char {
            Some(_) => {
                let tokens = tokenize(equation, &variables);
                let expression = infix_to_postfix(tokens);
                match evaluate(expression) {
                    Ok(result) => {
                        let answer = result.to_string();
                        variables.insert('=', answer.clone());
                    },
                    Err(err) => println!("Error: {}", err),
                }
            },
            None => {},
        }
    }
    let result = &variables.get(&'=').unwrap();
    result.to_string()
}

pub fn test() {
    println!("Starting tests...");
    let mut success = 0;
    let mut failure = 0;

    let mut tests: HashMap<&str, &str> = HashMap::new();
    tests.insert("2+2", "4");
    tests.insert("2-+2", "0");
    tests.insert("2--2", "4");
    tests.insert("2++2", "4");
    tests.insert("-4+4", "0");
    tests.insert("-(4+4)", "-8");
    tests.insert("5!^2+3", "14403");
    tests.insert("-2+10/2*40+5!", "318");
    tests.insert("4 4", "16");
    tests.insert("(5+5)(5+5)", "100");
    tests.insert("(5+5)5+5", "55");
    tests.insert("5+5(5+5)", "55");
    tests.insert("(-4+4)", "0");
    tests.insert("-4//3+4#3", "0");
    tests.insert("30%%100 + 30\\100", "60");
    tests.insert("=5;=+5;=5;=^2;=/2", "312.5");
    tests.insert("-e", "-2.71828182845");
    tests.insert("pe", "8.539734222645713");
    tests.insert("p e", "8.539734222645713");
    tests.insert("1;p=e=======;", "8.539734222645713");

    for (&equation, &expected) in tests.iter() {
        let result = start_to_finish(equation);
        if result != expected {
            println!("{} expected {} but returned {}", equation, expected, result);
            failure += 1;
        } else {
            success += 1;
        }
    }

    let total_tests = failure + success;
    let percent_successful = success as f32 / total_tests as f32 * 100.0;
    println!("Testing Complete. {}% successful.", percent_successful);
}