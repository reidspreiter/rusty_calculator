use std::collections::HashMap;
use crate::tokenize::tokenize;
use crate::infix_to_postfix::infix_to_postfix;
use crate::evaluate::evaluate;

// Executes the text equation and returns the final result
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
                match tokenize(equation, &variables) {
                    Ok(tokens) => {
                        let expression = infix_to_postfix(tokens);
                        match evaluate(expression) {
                            Ok(result) => {
                                let answer = result.to_string();
                                variables.insert('=', answer.clone());
                            },
                            Err(err) => println!("Error: {}", err),
                        }
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

// Runs all tests
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
    tests.insert("-4*-4", "16");
    tests.insert("-4/-4", "1");
    tests.insert("4*-4", "-16");
    tests.insert("-4/4", "-1");
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
    tests.insert("R2^2", "2");
    tests.insert("-2^3;-=", "8");
    tests.insert("-2^2;(-=)^2", "16");
    tests.insert("-3R2^3", "0.5");
    tests.insert("2*1R4", "8");
    tests.insert("(R(R(R(R4))))^2^2^2^2", "3.9999999999999982"); 
    tests.insert("(-3)R8+1", "1.5");
    tests.insert("RL10", "2");
    tests.insert("R(L10)", "1");
    tests.insert("5Ne", "5");
    tests.insert("Ne-eLe", "0");
    tests.insert("3H4-R(4^2+3^2)", "0");
    tests.insert("-3H-4-R((-4)^2+(-3)^2)", "0");
    tests.insert("S[0, 4, 4] - (4+4+4+4+4)", "0");
    tests.insert("4S[0, 3, 3]", "48");
    tests.insert("P[1, 3, 3]4", "108");
    tests.insert("S[0, 9, x^2]-(1+4+9+16+25+36+49+64+81)", "0");
    tests.insert("P[S[P[1, 3, x+1], P[1, 3, x+2], x+3], 1668, R(x-50)]", "2613070.9999998086");
    tests.insert("2(2(2(2(2", "32");
    tests.insert("S[0, 2(2(2+2, 2]", "34");
    tests.insert("A[4E10,2,3.4]", "13333333335.133333");
    tests.insert("A[1+2,3+4,5/6,5^3,-5,43*2]", "36.13888888888889");
    tests.insert("O[1, 2, 3, 4, 5]", "1.4142135623730951");
    tests.insert("O[1+3, 4+7, 5/10, R25,4^2,-50]","21.938835429438818");
    tests.insert("Q[3+3,-17,R144]", "48");

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
    println!("Testing Complete!\n{} tests ran. {} tests failed. {}% successful.", 
            total_tests, failure, percent_successful);
}