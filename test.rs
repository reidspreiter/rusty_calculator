use std::collections::HashMap;
use crate::tokenize::tokenize;
use crate::infix_to_postfix::infix_to_postfix;
use crate::evaluate::evaluate;

// Executes a test equation and returns the final result.
fn compute_result(test: &str) -> String {
    let mut variables: HashMap<char, String> = [
        ('p', "3.14159265359".to_string()),
        ('e', "2.71828182845".to_string()),
        ('=', "0".to_string()),
    ].iter().cloned().collect();

    println!("Test: {}", test);
    let equations: Vec<&str> = test.split(";").collect();
    for equation in equations {
        match tokenize(equation, &variables) {
            Ok(tokens) => {
                match evaluate(infix_to_postfix(&tokens)) {
                    Ok(result) => {
                        let answer = result.to_string();
                        variables.insert('=', answer.clone());
                    },
                    Err(e) => println!("Error: {}", e),
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    let result = variables.get(&'=').unwrap();
    result.to_string()
}

// Runs all tests.
pub fn run_tests() {
    println!("Starting tests...");
    let mut success = 0;
    let mut failure = 0;

    let tests: HashMap<&str, &str> = [
        ("2+2", "4"),
        ("2-+2", "0"),
        ("2--2", "4"),
        ("2++2", "4"),
        ("-4+4", "0"),
        ("-(4+4)", "-8"),
        ("-4*-4", "16"),
        ("-4/-4", "1"),
        ("4*-4", "-16"),
        ("-4/4", "-1"),
        ("5!^2+3", "14403"),
        ("-2+10/2*40+5!", "318"),
        ("4 4", "16"),
        ("(5+5)(5+5)", "100"),
        ("(5+5)5+5", "55"),
        ("5+5(5+5)", "55"),
        ("(-4+4)", "0"),
        ("-4//3+4#3", "0"),
        ("30%%100 + 30\\100", "60"),
        ("=5;=+5;=5;=^2;=/2", "312.5"),
        ("-e", "-2.71828182845"),
        ("pe", "8.539734222645713"),
        ("p e", "8.539734222645713"),
        ("1;p=e=======", "8.539734222645713"),
        ("R2^2", "2"),
        ("-2^3;-=", "8"),
        ("-2^2;(-=)^2", "16"),
        ("-3R2^3", "0.5"),
        ("2*1R4", "8"),
        ("(R(R(R(R4))))^2^2^2^2", "3.9999999999999982"),
        ("(-3)R8+1", "1.5"),
        ("RL10", "2"),
        ("R(L10)", "1"),
        ("5Ne", "5"),
        ("Ne-eLe", "0"),
        ("3H4-R(4^2+3^2)", "0"),
        ("-3H-4-R((-4)^2+(-3)^2)", "0"),
        ("S[0, 4, 4] - (4+4+4+4+4)", "0"),
        ("4S[0, 3, 3]", "48"),
        ("P[1, 3, 3]4", "108"),
        ("S[0, 9, x^2]-(1+4+9+16+25+36+49+64+81)", "0"),
        ("P[S[P[1, 3, x+1], P[1, 3, x+2], x+3], 1668, R(x-50)]", "2613070.9999998086"),
        ("2(2(2(2(2", "32"),
        ("S[0, 2(2(2+2, 2]", "34"),
        ("M[4E10,2,3.4]", "13333333335.133333"),
        ("M[1+2,3+4,5/6,5^3,-5,43*2]", "36.13888888888889"),
        ("O[1, 2, 3, 4, 5]", "1.4142135623730951"),
        ("O[1+3, 4+7, 5/10, R25,4^2,-50]","21.938835429438818"),
        ("Q[3+3,-17,R144]", "48"),
        ("5A-1", "5"),
        ("A(5-6+A(5-50)", "44"),
    ].iter().cloned().collect();

    for (&equation, &expected) in tests.iter() {
        let result = compute_result(equation);
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