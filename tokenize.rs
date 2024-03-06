use std::collections::HashMap;

const OPERATORS: [&str; 11] = ["+", "-", "*", "/", "^", "%", "#", "(", "\\", "R", "~"];

// Tokenize User Equation into individual Strings
pub fn tokenize(equation: &str, variable_map: &HashMap<char, String>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut number_buffer = String::new();

    for c in equation.chars() {
        match c {
            '0'..='9' | '.' | 'E' => number_buffer.push(c),
            _ => {
                if let Some(value) = variable_map.get(&c) {
                    if!number_buffer.is_empty() {
                        tokens.push(number_buffer.clone());
                        number_buffer.clear();
                    }
                    number_buffer.push_str(&value.to_string())
                }
                if !number_buffer.is_empty() {
                    if let Some(last) = tokens.last() {
                        match last.as_str() {
                            ")" => tokens.push("*".to_string()),
                            _ => {
                                if let Ok(_) = last.as_str().parse::<f64>() {
                                    tokens.push("*".to_string());
                                } else {}
                            },
                        }
                    }
                    tokens.push(number_buffer.clone());
                    number_buffer.clear();
                }
                match c {
                    '+' => {
                        if let Some(last) = tokens.last() {
                            if !OPERATORS.contains(&last.as_str()) {
                                tokens.push(c.to_string());
                            }
                        } else {
                            tokens.push(c.to_string());
                        }
                    },
                    '-' => {
                        match &tokens.last() {
                            Some(last) if OPERATORS.contains(&last.as_str()) => {
                                tokens.push("~".to_string());
                            },
                            None => tokens.push("~".to_string()), 
                            _ => {
                                tokens.push(c.to_string());
                            },
                        }
                    },
                    '*' => tokens.push(c.to_string()),
                    '/' => {
                        if let Some(last) = tokens.last() {
                            match last.as_str() {
                                "/" => {
                                    if let Some(last) = tokens.last_mut() {
                                        *last = "#".to_string();
                                    }
                                },
                                _ => tokens.push(c.to_string()),
                            }
                        } else {
                            tokens.push(c.to_string());
                        }
                    },
                    '%' => {
                        if let Some(last) = tokens.last() {
                            match last.as_str() {
                                "%" => {
                                    if let Some(last) = tokens.last_mut() {
                                        *last = "\\".to_string();
                                    }
                                },
                                _ => tokens.push(c.to_string()),
                            }
                        } else {
                            tokens.push(c.to_string());
                        }
                    },
                    '#' => tokens.push(c.to_string()),
                    '\\' => tokens.push(c.to_string()),
                    '^' => tokens.push(c.to_string()),
                    '(' => {
                        if let Some(last) = tokens.last() {
                            match last.as_str() {
                                ")" => tokens.push("*".to_string()),
                                _ => {
                                    if let Ok(_) = last.as_str().parse::<f64>() {
                                        tokens.push("*".to_string());
                                    } else {}
                                },
                            }
                        }
                        tokens.push(c.to_string());
                    },
                    ')' => tokens.push(c.to_string()),
                    '!' => tokens.push(c.to_string()),
                    'R' => {
                        match &tokens.last() {
                            Some(last) if OPERATORS.contains(&last.as_str()) => {
                                tokens.push("2".to_string());
                            },
                            Some(last) if last.as_str() == "~" => {
                                tokens.push("2".to_string());
                            },
                            None => tokens.push("2".to_string()),
                            _ => {},
                        }
                        tokens.push(c.to_string());
                    },
                    _ => {
                        if c != ' ' && !variable_map.contains_key(&c) {
                            println!("'{}' is not a valid character. Solving without {}.", c, c);
                        }
                    },
                }
            }
        }
    }
    if !number_buffer.is_empty() {
        if let Some(last) = tokens.last() {
            match last.as_str() {
                ")" => tokens.push("*".to_string()),
                _ => {
                    if let Ok(_) = last.as_str().parse::<f64>() {
                        tokens.push("*".to_string());
                    } else {}
                },
            }
        }
        tokens.push(number_buffer.clone());
        number_buffer.clear();
    }
    tokens
}