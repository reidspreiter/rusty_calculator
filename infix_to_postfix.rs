// Order of operations precedence for converting infix to postfix
fn precedence(operator: &str) -> i8 {
    match operator {
        "!" => 6,
        "^" => 5,
        "R" | "L" | "H" => 4,
        "*" | "/" | "%" | "#" => 3,
        "\\" => 2,
        "~" => 1,
        "+" | "-" => 0,
        _ => -1,
    }
}

// Convert infix to postfix
pub fn infix_to_postfix(tokens: Vec<String>) -> Vec<String> {
    let mut postfix_expression: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" | "-" | "*" | "/" | "^" | "%" | "#" | "\\" | "!" | "R" | "~" | "L" | "H" => {
                while let Some(top) = stack.last() {
                    if top == "(" || precedence(&top) < precedence(token.as_str()) {
                        break;
                    }
                    if let Some(popped) = stack.pop() {
                        postfix_expression.push(popped);
                    }
                }
                stack.push(token);
            },
            "(" => stack.push(token),
            ")" => {
                while let Some(top) = stack.pop() {
                    if top == "(" {
                        break;
                    }
                    postfix_expression.push(top);
                }
            },
            _ => postfix_expression.push(token),
        }
    }

    while let Some(token) = stack.pop() {
        postfix_expression.push(token);
    }
    postfix_expression
}