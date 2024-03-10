use crate::complex_evaluate::complex_evaluate;
use std::collections::HashMap;

const OPERATORS: [&str; 14] = ["+", "-", "*", "/", "^", "%", "#", 
                                "(", "\\", "R", "~", "L", "H", ","];

// Tokenize usser entered equation into individual strings tokens.
// Also evaluate complex functions and push their result as a token.
pub fn tokenize(equation: &str, variable_map: &HashMap<char, String>) -> Result<Vec<String>, String> {
    let mut tokens: Vec<Vec<String>> = vec![Vec::new()];
    let mut index = 0;
    let mut number_buffer = String::new();
    let mut complex_types: Vec<char> = Vec::new();
    let mut complex_tokens = false;
    let mut balanced_parenthesis: Vec<i32> = vec![0];

    for c in equation.chars() {
        match c {
            '0'..='9' | '.' | 'E' => number_buffer.push(c),
            _ => {
                let curr_tokens = &mut tokens[index];
                // Push potential variable values.
                if let Some(value) = variable_map.get(&c) {
                    if !number_buffer.is_empty() {
                        curr_tokens.push(number_buffer.clone());
                        number_buffer.clear();
                    }
                    number_buffer.push_str(&value.to_string())
                }
                // Push number buffer.
                if !number_buffer.is_empty() {
                    // If last is f64 or ')', push '*' before pushing number.
                    if let Some(last) = curr_tokens.last() {
                        if last == ")" || last.parse::<f64>().is_ok() {
                            curr_tokens.push("*".to_string());
                        }
                    }
                    curr_tokens.push(number_buffer.clone());
                    number_buffer.clear();
                }
                match c {
                    '+' => {
                        // If last is None or not an operator, push '+'.
                        if curr_tokens.last()
                        .map_or(true, |last| !OPERATORS.contains(&last.as_str())) {
                            curr_tokens.push(c.to_string());
                        }
                    },
                    '-' => {
                        // If last is an operator or none, push '~'. Otherwise, push '-'.
                        if curr_tokens.last()
                        .map_or(true, |last| OPERATORS.contains(&last.as_str())) {
                            curr_tokens.push("~".to_string());
                        } else {
                            curr_tokens.push(c.to_string());
                        }
                    },
                    '*' => curr_tokens.push(c.to_string()),
                    '/' => {
                        // If last is '/', push '#'. Otherwise, push '/'.
                        if let Some(last) = curr_tokens.last() {
                            match last.as_str() {
                                "/" => {
                                    if let Some(last) = curr_tokens.last_mut() {
                                        *last = "#".to_string();
                                    }
                                },
                                _ => curr_tokens.push(c.to_string()),
                            }
                        } else {
                            curr_tokens.push(c.to_string());
                        }
                    },
                    '%' => {
                        // If last is '%', push '\\'. Otherwise, push '%'.
                        if let Some(last) = curr_tokens.last() {
                            match last.as_str() {
                                "%" => {
                                    if let Some(last) = curr_tokens.last_mut() {
                                        *last = "\\".to_string();
                                    }
                                },
                                _ => curr_tokens.push(c.to_string()),
                            }
                        } else {
                            curr_tokens.push(c.to_string());
                        }
                    },
                    '#' => curr_tokens.push(c.to_string()),
                    '\\' => curr_tokens.push(c.to_string()),
                    '^' => curr_tokens.push(c.to_string()),
                    '(' => {
                        // If last is f64 or ')', push '*' before pushing '('.
                        if let Some(last) = curr_tokens.last() {
                            if last == ")" || last.parse::<f64>().is_ok() {
                                curr_tokens.push("*".to_string());
                            }
                        }
                        curr_tokens.push(c.to_string());
                        balanced_parenthesis[index] += 1;
                    },
                    ')' => {
                        curr_tokens.push(c.to_string());
                        balanced_parenthesis[index] -= 1;
                    }
                    '!' => curr_tokens.push(c.to_string()),
                    'R' => {
                        // Push default root value of 2 if none is provided.
                        if curr_tokens.last()
                        .map_or(true, |last| OPERATORS.contains(&last.as_str())) {
                            curr_tokens.push("2".to_string());
                        }
                        curr_tokens.push(c.to_string());
                    },
                    'L' => {
                        // Push default log base value of 10 if none is provided.
                        if curr_tokens.last()
                        .map_or(true, |last| OPERATORS.contains(&last.as_str())) {
                            curr_tokens.push("10".to_string());
                        }
                        curr_tokens.push(c.to_string());
                    },
                    'N' => {
                        // Push e and L to simulate ln functionality.
                        if let Some(last) = curr_tokens.last() {
                            if !OPERATORS.contains(&last.as_str()) {
                                curr_tokens.push("*".to_string());
                            }
                        }
                        curr_tokens.push("2.71828182845".to_string());
                        curr_tokens.push("L".to_string());
                    },
                    'H' => curr_tokens.push(c.to_string()),
                    'S' | 'P' => {
                        // Store complex type and push '*' if needed
                        complex_tokens = true;
                        complex_types.push(c);
                        if let Some(last) = curr_tokens.last() {
                            if last == ")" || last.parse::<f64>().is_ok() {
                                curr_tokens.push("*".to_string());
                            }
                        }
                    },
                    '[' if complex_tokens => {
                        index += 1;
                        tokens.push(Vec::new());
                        balanced_parenthesis.push(0);
                    },
                    ']' if complex_tokens => {
                        if index == 0 {
                            return Err("Unable to tokenize complexities. 
                                        Dumping equation.".to_string());
                        }
                        if let Some(parenthesis) = balanced_parenthesis.pop() {
                            for _ in 0..parenthesis {
                                curr_tokens.push(')'.to_string());
                            }
                        }
                        index -= 1;
                        if index == 0 {
                            complex_tokens = false;
                        }
                        if let Some(ctokens) = tokens.pop() {
                            if let Some(ctype) = complex_types.pop() {
                                tokens[index].push(complex_evaluate(ctokens, ctype));
                            }
                        }
                    },
                    ',' if complex_tokens => {
                        if let Some(parenthesis) = balanced_parenthesis.pop() {
                            for _ in 0..parenthesis {
                                curr_tokens.push(')'.to_string());
                            }
                        }
                        balanced_parenthesis.push(0);
                        curr_tokens.push(c.to_string());
                    },
                    _ => {
                        if c == 'x' {
                            if let Some(last) = complex_types.last() {
                                match last {
                                    'S' | 'P' => curr_tokens.push(c.to_string()),
                                    _ => println!("'x' is not a valid character for complexity 
                                                    type {}. Solving without x.", last),
                                }
                            }
                        } else if complex_types.is_empty() && c != ' ' 
                            && !variable_map.contains_key(&c) {
                            println!("'{}' is not a valid character. Solving without {}.", c, c);
                        }
                    },
                }
            },
        }
    }

    let curr_tokens = &mut tokens[index];
    if !number_buffer.is_empty() {
        // If last is f64 or ')', push '*' before pushing number.
        if let Some(last) = curr_tokens.last() {
            if last == ")" || last.parse::<f64>().is_ok() {
                curr_tokens.push("*".to_string());
            }
        }
        curr_tokens.push(number_buffer.clone());
        number_buffer.clear();
    }
    if let Some(parenthesis) = balanced_parenthesis.pop() {
        for _ in 0..parenthesis {
            curr_tokens.push(')'.to_string());
        }
    }
    if let Some(final_tokens) = tokens.pop() {
        if tokens.is_empty() {
            Ok(final_tokens)
        } else {
            Err("Unable to tokenize complexities. Dumping equation.".to_string())
        }
    } else {
        Err("Unable to tokenize complexities. Dumping equation.".to_string())
    }
}