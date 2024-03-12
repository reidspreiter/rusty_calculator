use crate::infix_to_postfix::infix_to_postfix;
use crate::evaluate::evaluate;

// Calls correct complexity function.
pub fn complex_evaluate(tokens: &Vec<String>, complexity_type: &char) -> Result<String, String> {
    match complexity_type {
        'S' => summation(&tokens),
        'P' => product(&tokens),
        'A' => average(&tokens),
        _ => Err("Unknown complexity type".to_string()),
    }
}

// Separates a vector into multiple vectors based on commas.
// Length_limit of 0 will separate indefinitely.
fn separate_vector(tokens: &Vec<String>, length_limit: usize) -> Vec<Vec<String>> {
    let mut separated_tokens: Vec<Vec<String>> = vec![Vec::new()];

    for token in tokens {
        match token.as_str() {
            "," => {
                if let Some(last) = separated_tokens.last() {
                    if !last.is_empty() {
                        if last.len() == length_limit {
                            break;
                        }
                        separated_tokens.push(Vec::new());
                    }
                }
            },
            _ => {
                if let Some(last) = separated_tokens.last_mut() {
                    last.push(token.to_string());
                }
            },
        }
    }
    if let Some(last) = separated_tokens.last() {
        if last.is_empty() {
            separated_tokens.pop();
        }
    }
    separated_tokens
}

// Compute the summation of given tokens as [start,upper limit,equation]
fn summation(tokens: &Vec<String>) -> Result<String, String> {
    let separated_tokens = separate_vector(&tokens, 3);
    
    match separated_tokens.len() {
        0 => return Err("Missing summation start, upper limit, and equation".to_string()),
        1 => return Err("Missing summation upper limit and equation".to_string()),
        2 => return Err("Missing summation equation".to_string()),
        _ => {}
    }
    let start = match evaluate(infix_to_postfix(separated_tokens[0].clone())) {
        Ok(result) => result as i64,
        Err(_) => return Err("Could not evaluate summation start".to_string()),
    };

    let upper_limit = match evaluate(infix_to_postfix(separated_tokens[1].clone())) {
        Ok(result) => result as i64,
        Err(_) => return Err("Could not evaluate summation upper limit".to_string()),
    };

    let equation = separated_tokens[2].clone();

    let mut summation_result = 0.0;
    for i in start..=upper_limit {
        let mut equation_tokens = equation.clone();
        for token in equation_tokens.iter_mut() {
            if *token == "x" {
                *token = i.to_string();
            }
        }
        match evaluate(infix_to_postfix(equation_tokens)) {
            Ok(result) => summation_result += result,
            Err(_) => return Err("Could not evaluate summation equation".to_string()),
        }
    }
    Ok(summation_result.to_string())
}

// Compute the product of given tokens as [start,upper limit,equation]
fn product(tokens: &Vec<String>) -> Result<String, String> {
    let separated_tokens = separate_vector(&tokens, 3);

    match separated_tokens.len() {
        0 => return Err("Missing product start, upper limit, and equation".to_string()),
        1 => return Err("Missing product upper limit and equation".to_string()),
        2 => return Err("Missing product equation".to_string()),
        _ => {}
    }

    let start = match evaluate(infix_to_postfix(separated_tokens[0].clone())) {
        Ok(result) => result as i64,
        Err(_) => return Err("Could not evaluate product start".to_string()),
    };

    let upper_limit = match evaluate(infix_to_postfix(separated_tokens[1].clone())) {
        Ok(result) => result as i64,
        Err(_) => return Err("Could not evaluate product upper limit".to_string()),
    };

    let equation = separated_tokens[2].clone();

    let mut product_result = 1.0;
    for i in start..=upper_limit {
        let mut equation_tokens = equation.clone();
        for token in equation_tokens.iter_mut() {
            if *token == "x" {
                *token = i.to_string();
            }
        }
        match evaluate(infix_to_postfix(equation_tokens)) {
            Ok(result) => product_result *= result,
            Err(_) => return Err("Could not evaluate product equation".to_string()),
        }
    }
    Ok(product_result.to_string())
}

fn average(tokens: &Vec<String>) -> Result<String, String> {
    let separated_tokens = separate_vector(&tokens, 0);

    let total_values = separated_tokens.len();
    if total_values == 0 {
        return Err("Missing average values".to_string());
    }

    let mut sum = 0.0;
    for equation_tokens in separated_tokens {
        match evaluate(infix_to_postfix(equation_tokens)) {
            Ok(result) => sum += result,
            Err(_) => return Err("Could not evaluate average value equation".to_string()),
        }
    }
    let average = sum / total_values as f64;
    Ok(average.to_string())
}