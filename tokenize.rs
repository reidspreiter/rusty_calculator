use crate::complex_evaluate::complex_evaluate;
use std::collections::HashMap;

const OPERATORS: [&str; 13] = ["+", "-", "*", "/", "^", "%", "#", "(", "\\", "R", "~", "L", "H"];

// Tokenize User Equation into individual Strings
pub fn tokenize(equation: &str, variable_map: &HashMap<char, String>) -> Result<Vec<String>, String> {
    let mut tokens: Vec<Vec<String>> = Vec::new();
    tokens.push(Vec::new());
    let mut tokens_index = 0;
    let mut number_buffer = String::new();
    //let mut complex_search = false;
    let mut complex_types: Vec<char> = Vec::new();

    for c in equation.chars() {
        match c {
            '0'..='9' | '.' | 'E' => number_buffer.push(c),
            _ => {
                if let Some(value) = variable_map.get(&c) {
                    if!number_buffer.is_empty() {
                        tokens[tokens_index].push(number_buffer.clone());
                        number_buffer.clear();
                    }
                    number_buffer.push_str(&value.to_string())
                }
                if !number_buffer.is_empty() {
                    if let Some(last) = tokens[tokens_index].last() {
                        match last.as_str() {
                            ")" => tokens[tokens_index].push("*".to_string()),
                            _ => {
                                if let Ok(_) = last.as_str().parse::<f64>() {
                                    tokens[tokens_index].push("*".to_string());
                                } else {}
                            },
                        }
                    }
                    tokens[tokens_index].push(number_buffer.clone());
                    number_buffer.clear();
                }
                match c {
                    '+' => {
                        if let Some(last) = tokens[tokens_index].last() {
                            if !OPERATORS.contains(&last.as_str()) {
                                tokens[tokens_index].push(c.to_string());
                            }
                        } else {
                            tokens[tokens_index].push(c.to_string());
                        }
                    },
                    '-' => {
                        match &tokens[tokens_index].last() {
                            Some(last) if OPERATORS.contains(&last.as_str()) => {
                                tokens[tokens_index].push("~".to_string());
                            },
                            None => tokens[tokens_index].push("~".to_string()), 
                            _ => {
                                tokens[tokens_index].push(c.to_string());
                            },
                        }
                    },
                    '*' => tokens[tokens_index].push(c.to_string()),
                    '/' => {
                        if let Some(last) = tokens.last() {
                            match last[tokens_index].as_str() {
                                "/" => {
                                    if let Some(last) = tokens[tokens_index].last_mut() {
                                        *last = "#".to_string();
                                    }
                                },
                                _ => tokens[tokens_index].push(c.to_string()),
                            }
                        } else {
                            tokens[tokens_index].push(c.to_string());
                        }
                    },
                    '%' => {
                        if let Some(last) = tokens[tokens_index].last() {
                            match last.as_str() {
                                "%" => {
                                    if let Some(last) = tokens[tokens_index].last_mut() {
                                        *last = "\\".to_string();
                                    }
                                },
                                _ => tokens[tokens_index].push(c.to_string()),
                            }
                        } else {
                            tokens[tokens_index].push(c.to_string());
                        }
                    },
                    '#' => tokens[tokens_index].push(c.to_string()),
                    '\\' => tokens[tokens_index].push(c.to_string()),
                    '^' => tokens[tokens_index].push(c.to_string()),
                    '(' => {
                        if let Some(last) = tokens[tokens_index].last() {
                            match last.as_str() {
                                ")" => tokens[tokens_index].push("*".to_string()),
                                _ => {
                                    if let Ok(_) = last.as_str().parse::<f64>() {
                                        tokens[tokens_index].push("*".to_string());
                                    } else {}
                                },
                            }
                        }
                        tokens[tokens_index].push(c.to_string());
                    },
                    ')' => tokens[tokens_index].push(c.to_string()),
                    '!' => tokens[tokens_index].push(c.to_string()),
                    'R' => {
                        match &tokens[tokens_index].last() {
                            Some(last) if OPERATORS.contains(&last.as_str()) => {
                                tokens[tokens_index].push("2".to_string());
                            },
                            None => tokens[tokens_index].push("2".to_string()),
                            _ => {},
                        }
                        tokens[tokens_index].push(c.to_string());
                    },
                    'L' => {
                        match &tokens[tokens_index].last() {
                            Some(last) if OPERATORS.contains(&last.as_str()) => {
                                tokens[tokens_index].push("10".to_string());
                            },
                            None => tokens[tokens_index].push("10".to_string()),
                            _ => {},
                        }
                        tokens[tokens_index].push(c.to_string());
                    },
                    'N' => {
                        match &tokens[tokens_index].last() {
                            Some(last) if !OPERATORS.contains(&last.as_str()) => {
                                tokens[tokens_index].push("*".to_string());
                            },
                            _ => {},
                        }
                        if let Some(value) = variable_map.get(&'e') {
                            tokens[tokens_index].push(value.to_string());
                        }
                        tokens[tokens_index].push("L".to_string());
                    },
                    'H' => tokens[tokens_index].push(c.to_string()),
                    'S' | 'P' => {
                        complex_types.push(c);
                    }
                    _ => {
                        if complex_types.is_empty() && c != ' ' && !variable_map.contains_key(&c) {
                            println!("'{}' is not a valid character. Solving without {}.", c, c);
                        } else if complex_types.len() > 0 {
                            match c {
                                '[' => {
                                    tokens_index += 1;
                                    tokens.push(Vec::new());
                                },
                                ']' => {
                                    tokens_index -= 1;
                                    if let Some(complex_tokens) = tokens.pop() {
                                        if let Some(complex_type) = complex_types.pop() {
                                            tokens[tokens_index].push(complex_evaluate(complex_tokens, complex_type));
                                        }
                                    }
                                },
                                ',' => tokens[tokens_index].push(c.to_string()),
                                _ => {
                                    if c == 'x' {
                                        if let Some(last) = complex_types.last() {
                                            if last != &'S' && last != &'P' {
                                                println!("'{}' is not a valid character for complexity type {}. Solving without {}.", c, last, c);
                                            } else {
                                                tokens[tokens_index].push(c.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }

    if !number_buffer.is_empty() {
        if let Some(last) = tokens[tokens_index].last() {
            match last.as_str() {
                ")" => tokens[tokens_index].push("*".to_string()),
                _ => {
                    if let Ok(_) = last.as_str().parse::<f64>() {
                        tokens[tokens_index].push("*".to_string());
                    } else {}
                },
            }
        }
        tokens[tokens_index].push(number_buffer.clone());
        number_buffer.clear();
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