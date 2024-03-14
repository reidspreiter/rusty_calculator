// Order of operations precedence for converting infix to postfix.
fn precedence(operator: &str) -> i8 {
    match operator {
        "!" => 6,
        "^" => 5,
        "~" => 4,
        "R" | "L" | "H" | "A" => 3,
        "*" | "/" | "%" | "#" => 2,
        "\\" => 1,
        "+" | "-" => 0,
        _ => -1,
    }
}

// Converts an infix equation to a postfix equation.
pub fn infix_to_postfix(tokens: &Vec<String>) -> Vec<String> {
    let mut postfix_expression: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" | "^" | "%" | "#" | "\\" | "!" | "R" | "~" | "L" | "H" | "A" => {
                while let Some(top) = stack.last() {
                    if top == "(" || precedence(&top) < precedence(token.as_str()) {
                        break;
                    }
                    postfix_expression.push(stack.pop().unwrap());
                }
                stack.push(token.to_string());
            },
            "(" => stack.push(token.to_string()),
            ")" => {
                while let Some(top) = stack.pop() {
                    if top == "(" {
                        break;
                    }
                    postfix_expression.push(top);
                }
            },
            _ => postfix_expression.push(token.to_string()),
        }
    }
    postfix_expression.extend(stack.into_iter().rev());
    postfix_expression
}