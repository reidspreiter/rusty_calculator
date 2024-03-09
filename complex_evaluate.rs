use crate::infix_to_postfix::infix_to_postfix;
use crate::evaluate::evaluate;

pub fn complex_evaluate(tokens: Vec<String>, complexity_type: char) -> String {
    match complexity_type {
        'S' => summation(tokens),
        'P' => product(tokens),
        _ => "".to_string(),
    }
}

fn summation(tokens: Vec<String>) -> String {
    let mut separated_tokens: Vec<Vec<String>> = vec![Vec::new(); 3];
    let mut token_index = 0;

    for token in tokens {
        match token.as_str() {
            "," => {
                if token_index == 2 {
                    break;
                }
                token_index += 1;
            },
            _ => separated_tokens[token_index].push(token.to_string()),
        }
    }

    let start = match evaluate(infix_to_postfix(separated_tokens[0].clone())) {
        Ok(result) => result as i64,
        Err(_) => 0,
    };

    let upper_limit = match evaluate(infix_to_postfix(separated_tokens[1].clone())) {
        Ok(result) => result as i64,
        Err(_) => 0,
    };

    let mut summation_result = 0.0;
    for i in start..=upper_limit {
        let mut equation_tokens = separated_tokens[2].clone();
        for token in equation_tokens.iter_mut() {
            if *token == "x" {
                *token = i.to_string();
            }
        }
        match evaluate(infix_to_postfix(equation_tokens)) {
            Ok(result) => summation_result += result,
            Err(_) => {},
        }
    }
    summation_result.to_string()
}

fn product(tokens: Vec<String>) -> String {
    let mut separated_tokens: Vec<Vec<String>> = vec![Vec::new(); 3];
    let mut token_index = 0;

    for token in tokens {
        match token.as_str() {
            "," => {
                if token_index == 2 {
                    break;
                }
                token_index += 1;
            },
            _ => separated_tokens[token_index].push(token.to_string()),
        }
    }

    let start = match evaluate(infix_to_postfix(separated_tokens[0].clone())) {
        Ok(result) => result as i64,
        Err(_) => 0,
    };

    let upper_limit = match evaluate(infix_to_postfix(separated_tokens[1].clone())) {
        Ok(result) => result as i64,
        Err(_) => 0,
    };

    let mut product_result = 1.0;
    for i in start..=upper_limit {
        let mut equation_tokens = separated_tokens[2].clone();
        for token in equation_tokens.iter_mut() {
            if *token == "x" {
                *token = i.to_string();
            }
        }
        match evaluate(infix_to_postfix(equation_tokens)) {
            Ok(result) => product_result *= result,
            Err(_) => {},
        }
    }
    product_result.to_string()
}