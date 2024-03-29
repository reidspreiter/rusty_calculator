use crate::infix_to_postfix::infix_to_postfix;
use crate::evaluate::evaluate;

// Calls correct complexity function.
pub fn complex_evaluate<'a>(tokens: &'a Vec<String>, complexity_type: &char) -> Result<String, &'a str> {
    match complexity_type {
        'S' => summation(&tokens),
        'P' => product(&tokens),
        'M' => mean(&tokens),
        'O' => std_deviation(&tokens),
        'Q' => quadratic(&tokens),
        _ => Err("Unknown complexity type"),
    }
}

// Separates a vector into multiple vectors based on commas.
// length_limit of 0 will separate indefinitely.
fn separate_vector(tokens: &Vec<String>, length_limit: usize) -> Vec<Vec<String>> {
    let mut separated_tokens: Vec<Vec<String>> = vec![Vec::new()];

    for token in tokens {
        match token.as_str() {
            "," => {
                if let Some(last) = separated_tokens.last() {
                    if !last.is_empty() {
                        if separated_tokens.len() == length_limit {
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

// Compute the summation of given tokens as [start,upper limit,equation].
fn summation(tokens: &Vec<String>) -> Result<String, &str> {
    let separated_tokens = separate_vector(&tokens, 3);
    
    match separated_tokens.len() {
        0 => return Err("Missing summation start, upper limit, and equation"),
        1 => return Err("Missing summation upper limit and equation"),
        2 => return Err("Missing summation equation"),
        _ => {}
    }

    let start = match evaluate(infix_to_postfix(&separated_tokens[0])) {
        Ok(result) => result as i64,
        Err(_) => return Err("Could not evaluate summation start"),
    };

    let upper_limit = match evaluate(infix_to_postfix(&separated_tokens[1])) {
        Ok(result) => result as i64,
        Err(_) => return Err("Could not evaluate summation upper limit"),
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

        match evaluate(infix_to_postfix(&equation_tokens)) {
            Ok(result) => summation_result += result,
            Err(_) => return Err("Could not evaluate summation equation"),
        }
    }
    Ok(summation_result.to_string())
}

// Compute the product of given tokens as [start,upper limit,equation].
fn product(tokens: &Vec<String>) -> Result<String, &str> {
    let separated_tokens = separate_vector(&tokens, 3);

    match separated_tokens.len() {
        0 => return Err("Missing product start, upper limit, and equation"),
        1 => return Err("Missing product upper limit and equation"),
        2 => return Err("Missing product equation"),
        _ => {}
    }

    let start = match evaluate(infix_to_postfix(&separated_tokens[0])) {
        Ok(result) => result as i64,
        Err(_) => return Err("Could not evaluate product start"),
    };

    let upper_limit = match evaluate(infix_to_postfix(&separated_tokens[1])) {
        Ok(result) => result as i64,
        Err(_) => return Err("Could not evaluate product upper limit"),
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

        match evaluate(infix_to_postfix(&equation_tokens)) {
            Ok(result) => product_result *= result,
            Err(_) => return Err("Could not evaluate product equation"),
        }
    }
    Ok(product_result.to_string())
}

// Compute the average of given tokens as [value,value,...].
fn mean(tokens: &Vec<String>) -> Result<String, &str> {
    let separated_tokens = separate_vector(&tokens, 0);

    let total_values = separated_tokens.len();
    if total_values == 0 {
        return Err("Missing average values");
    }

    let sum = separated_tokens.iter().try_fold(0.0, |acc, equation_tokens| {
        match evaluate(infix_to_postfix(&equation_tokens)) {
            Ok(result) => Ok(acc + result),
            Err(_) => Err("Could not evaluate average value equation"),
        }
    })?;

    let average = sum / total_values as f64;
    Ok(average.to_string())
}

// Compute the standard deviation of given tokens as [value,value,...].
fn std_deviation(tokens: &Vec<String>) -> Result<String, &str> {
    let separated_tokens = separate_vector(&tokens, 0);

    let total_values = separated_tokens.len() as f64;
    if total_values == 0.0 {
        return Err("Missing standard deviation values");
    }

    let mut values: Vec<f64> = Vec::new();
    let sum = separated_tokens.iter().try_fold(0.0, |acc, equation_tokens| {
        match evaluate(infix_to_postfix(&equation_tokens)) {
            Ok(result) => {
                values.push(result);
                Ok(acc + result)
            }
            Err(_) => Err("Could not evaluate average value equation"),
        }
    })?;

    let mean = sum / total_values;
    let sum_of_squared_differences = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
    let standard_deviation = (sum_of_squared_differences / total_values).sqrt();
    Ok(standard_deviation.to_string())
}

// Computes quadratic formula of given tokens as [a,b,c].
fn quadratic(tokens: &Vec<String>) -> Result<String, &str> {
    let separated_tokens = separate_vector(&tokens, 3);

    match separated_tokens.len() {
        0 => return Err("Missing quadratic a, b, and c values"),
        1 => return Err("Missing quadratic b and c values."),
        2 => return Err("Missing quadratic c value"),
        _ => {}
    }

    let a = match evaluate(infix_to_postfix(&separated_tokens[0])) {
        Ok(result) => result as f64,
        Err(_) => return Err("Could not evaluate quadratic a value"),
    };

    let b = match evaluate(infix_to_postfix(&separated_tokens[1])) {
        Ok(result) => result as f64,
        Err(_) => return Err("Could not evaluate quadratic b value"),
    };

    let c = match evaluate(infix_to_postfix(&separated_tokens[2])) {
        Ok(result) => result as f64,
        Err(_) => return Err("Could not evaluate quadratic c value"),
    };

    let discriminant = b.powi(2) - (4.0 * a * c);
    if discriminant < 0.0 {
        return Err("No real quadratic solutions");
    }
    
    let first_solution = (-b + discriminant) / 2.0 * a;
    let second_solution = (-b - discriminant) / 2.0 * a;
    let results = format!("{},{}", first_solution, second_solution);
    Ok(results)
}