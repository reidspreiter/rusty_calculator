// Evaluate postfix expression
pub fn evaluate(expression: Vec<String>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expression {
        match token.as_str() {
            "+" | "-" | "*" | "/" | "^" | "%" | "#" | "\\" | "R" | "L" | "H" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    let result = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => {
                            if b == 0.0 {
                                let error_message = format!("Divide by Zero {} / {}", a, b);
                                return Err(error_message);
                            }
                            a / b
                        },
                        "^" => a.powf(b),
                        "%" => a % b,
                        "#" => (a / b).trunc(),
                        "\\" => (a / 100.0) * b,
                        "R" => {
                            if a == 0.0 {
                                return Err("Cannot take 0th root of a number".to_string());
                            }
                            if b < 0.0 && a % 2.0 == 0.0 {
                                return Err("Cannot take even root of a negative number".to_string());
                            }
                            b.powf(1.0 / a)
                        },
                        "L" => {
                            if b < 0.0 {
                                return Err("Cannot take log of a negative number".to_string());
                            }
                            if a < 0.0 {
                                return Err("Cannot compute log with a negative base".to_string());
                            }
                            b.log10() / a.log10()
                        },
                        "H" => (a*a + b*b).sqrt(),
                        operator => {
                            let error_message = format!("Invalid Operator {}", operator);
                            return Err(error_message)
                        },
                    };
                    stack.push(result);
                } else {
                    return Err("Not enough operands".to_string());
                }
            },
            "!" | "~" => {
                if let Some(a) = stack.pop() {
                    let result = match token.as_str() {
                        "!" => {
                            if a < 0.0 {
                                return Err("Cannot take negative factorial".to_string());
                            } 
                            if a.fract() != 0.0 {
                                return Err("Cannot evaluate decimal factorial (yet)".to_string());
                            }
                            let a_int = a as i32;
                            let factorial = match a_int {
                                0 | 1 => 1,
                                _ => (2..=a_int).fold(1, |acc, x| acc * x),
                            };
                            factorial as f64
                        },
                        "~" => -a,
                        operator => {
                            let error_message = format!("Invalid Operator {}", operator);
                            return Err(error_message);
                        },
                    };
                    stack.push(result as f64);
                }
            },
            operand => {
                if let Ok(num) = operand.parse::<f64>() {
                    stack.push(num);
                } else {
                    let error_message = format!("Invalid Operand {}", operand);
                    return Err(error_message);
                }
            },
        }
    }

    if let Some(result) = stack.pop() {
        if stack.is_empty() {
            return Ok(result);
        }
        return Err("Too many operands".to_string());
    }
    Err("Empty expression".to_string())
}