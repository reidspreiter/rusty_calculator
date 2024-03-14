use crate::test;
use crate::tokenize::tokenize;
use crate::infix_to_postfix::infix_to_postfix;
use crate::evaluate::evaluate;
use std::collections::HashMap;

pub fn get_variable_map() -> HashMap<char, String> {
    let variables: HashMap<char, String> = [
        ('p', "3.14159265359".to_string()),
        ('e', "2.71828182845".to_string()),
        ('=', "0".to_string()),
        ('i', "1".to_string()),
        ('j', "1".to_string()),
        ('k', "1".to_string()),
        ('l', "1".to_string()),
        ('m', "1".to_string()),
        ('n', "1".to_string()),
        ('o', "1".to_string()),
    ].iter().cloned().collect();
    variables
}

pub fn get_settings_map() -> HashMap<String, bool> {
    let settings: HashMap<String, bool> = [
        ("reveal".to_string(), false),
        ("quit".to_string(), false),
    ].iter().cloned().collect();
    settings
}

// Execute commands
pub fn execute_command(command: &str, variables: &mut HashMap<char, String>, settings: &mut HashMap<String, bool>) {
    match command {
        "/help" => help(),
        "/reset" => reset(variables, settings),
        "/variables" => print_variables(variables),
        "/x" => set_quit(settings),
        "/test" => test::test(),
        "/reveal" => set_reveal(settings),
        "/info" => print_info(),
        "/op" => print_operators(),
        "/varop" => print_variable_info(),
        "/complexop" => print_complex_operators(),
        "/oporder" => print_operation_order(),
        _ => println!("{} is not a valid command. Try /help.", command),
    }
}

pub fn change_variable(command: &str, variables: &mut HashMap<char, String>) {
    if let Some(variable) = command.chars().nth(1) {
        match variable {
            'p' | 'e' | '=' => println!("{} is not a mutable variable", variable),
            'i'..='o' => {
                let equation = &command[2..];
                match tokenize(equation, variables) {
                    Ok(result) => {
                        match evaluate(infix_to_postfix(&result)) {
                            Ok(result) => { 
                                variables.insert(variable, result.to_string());
                                println!("'{}' value changed to {}", variable, result);
                            },
                            Err(e) => { println!("Unable to evaluate new value: {}", e); },
                        }
                    },
                    Err(e) => println!("Unable to tokenize new value: {}", e),
                }
            },
            _ => println!("{} is not a valid variable", variable)
        }
    }
}

fn help() {
    println!("Help:\n\
    /x         -> Quit calculator\n\
    /reset     -> Reset variable and setting values to default\n\
    /variables -> Print all current variable values\n\
    /test      -> Run all test cases\n\
    /reveal    -> Toggle reveal on and off. When reveal is on, separate tokens and postfix
              expressions will be printed alongside the result for each user entered equation\n\
    /info      -> Prints behavior and general usage info\n\
    /op        -> Prints basic operator usage information\n\
    /varop     -> Prints variable usage information\n\
    /complexop -> Prints complex operator usage information\n\
    /oporder   -> Prints order of operations");
}

fn reset(variables: &mut HashMap<char, String>, settings: &mut HashMap<String, bool>) {
    *variables = get_variable_map();
    *settings = get_settings_map();
    println!("Variables and settings reset successfully");
}

fn print_variables(variables: &mut HashMap<char, String>) {
    println!("Variables:");
    for (key, value) in variables {
        println!("| {}: {}", key, value);
    }
}

fn set_quit(settings: &mut HashMap<String, bool>) {
    settings.insert("quit".to_string(), true);
}

fn set_reveal(settings: &mut HashMap<String, bool>) {
    settings.insert("reveal".to_string(), !settings["reveal"]); 
    println!("Reveal changed to {}", settings["reveal"]);
}

fn print_info() {
    println!("General Information:\n\n\
    Numbers:\n\
    | Numbers can be entered in integer and floating point format.\n\
    | Use 'E' for scientific notation.\n\n\
    Multiple Operations:\n\
    | Multiple operations can be queued at once with ';'.\n\
    | '4+4;2+2;4*5' will evaluate each operation separately.\n\
    | This also works with commands 'R16;/help;5!'.\n\n\
    Whitespace:\n\
    | Whitespace will not affect the results of any operation.\n\
    | Commands do depend on whitespace.\n\
    | Mutating variables must not have whitespace between the\n\
    |    underscore and the variable name (_p) but may have\n\
    |    whitespace anywhere thereafter.\n\n\
    Parenthesis Balancing:\n\
    | Operations with unbalanced parenthesis will become balanced.\n\
    |    '4(4(4(4' will be automatically interpreted as '4(4(4(4))).\n\
    | This also applies to values entered within complex operators.\n\
    |    'S[1,4(4(4,5(5+6]' will be automatically interpreted\n\
    |    as 'S[1,4(4(4)),5(5+6)]'.");
}

fn print_operators() {
    println!("Operators:\n\n\
    Addition: '+'\n\
    | Add two numbers (x+y).\n\n\
    Subtraction: '-'\n\
    | Subtract two numbers (x-y).\n\
    | Negation (-x*-y).\n\n\
    Multiplication: '*'\n\
    | Multiply two numbers (x*y).\n\
    | (x y), (x)y, x(y), xy are all interpreted as (x*y).\n\n\
    Division: '/'\n\
    | Divide two numbers (x/y).\n\n\
    Integer Division: '//' or '#'\n\
    | Computes divison result without remainder (x//y) (x#y).\n\n\
    Modulo: '%'\n\
    | Computes division remainder (x%y).\n\n\
    Percent Of: '%%' or '\\'\n\
    | Computes x percent of y (x%%y) (x\\y).\n\n\
    Exponent: '^'\n\
    | Computes x to the power of y (x^y).\n\n\
    Factorial: '!'\n\
    | Computes factorial (x!).\n\
    | Currently only works on positive integers.\n\n\
    Root: 'R'\n\
    | Computes xth root of y (xRy).\n\
    | Default root value is 2 (Rx = 2Rx).\n\n\
    Logarithm: 'L'\n\
    | Computes log base x of y (xLy).\n\
    | Default base value is 10 (Lx = 10Lx).\n\n\
    Natural Log: 'N'\n\
    | Computes log base e of x (Nx).\n\n\
    Pythagorean Theorem: 'H'\n\
    | Computes hypotenuse length of right triangle with\n\
    |    side lengths x and y (xHy).\n\
    | Always returns a posotive length value.\n\n\
    Absolute Value: 'A'\n\
    | Computes absolute value of x (Ax).");
}

fn print_variable_info() {
    println!("Variable Info:\n\n\
    Pi: 'p'\n\
    | Interpreted as 3.14159265359.\n\
    | Cannot be mutated.\n\n\
    Euler's Number: 'e'\n\
    | Interpreted as 2.71828182845.\n\
    | Cannot be mutated.\n\n\
    Answer: '='\n\
    | Interpreted as the most recent successful result value.\n\
    | Has a value of 0 if no previous answer exists or if /reset has been called.\n\
    | '=+1;=*5;=^2' will calculate a result of 25.\n\
    | Cannot be mutated.\n\n\
    Mutable Variables: 'i', 'j', 'k', 'l', 'm', 'n', 'o'\n\
    | These variables can be changed to any value specified by the user\n\
    | Their default value is 1.\n\
    | To change values, use the following command:\n\
    |    '_variablename(any value or computable operation)\n\
    | To change i to -41, enter:\n\
    |    '_i(-41)'\n\
    | To change n to whatever '5R41' evaluates to, enter:\n\
    |    '_n(5R41)'");
}

fn print_complex_operators() {
    println!("Complex Operators:\n\n\
    General Info:\n\
    | Complex operators are entered in the form:\n\
    |    'capital letter[comma separated values]'\n\
    | Operations can be entered as values:\n\
    |    'P[5+6, R5, 6!]\n\
    | They can be embedded within one another indefinitelly:\n\
    |    'P[1, P[1, 3, 4], 5]'\n\n\
    Summation: 'S'\n\
    | Computes summation from start value to upper limit of equation:\n\
    |    'S[start, upper limit, equation]'\n\
    | Equation can include the variable 'x'.\n\n\
    Product: 'P'\n\
    | Computes the product from start value to upper limit of equation:\n\
    |    'P[start, upper limit, equation]'\n\
    | Equation can include the variable 'x'.\n\n\
    Mean: 'M'\n\
    | Computes the mean of entered values:\n\
    |    'M[value, value, value, value, ...]'\n\n\
    Standard Deviation: 'O'\n\
    | Computes the standard deviation of entered values:\n\
    |    'O[value, value, value, value, ...]'\n\n\
    Quadratic Formula: 'Q'\n\
    | Computes the quadratic formula results of entered a, b, and c values:\n\
    |    'Q[a, b, c]'\n\
    | If two real roots exist, both will be printed to the screen, but only \n\
    |    the second root will be treated as the result.");
}

fn print_operation_order() {
    println!("Rusty Calculator's order of operations (Operators\n\
        in between [] have the same precedence and will be evaluated\n\
        from left to right):\n\n\
        !, ^, ~ (negation), [R, L, H, A], [*, /, %, #, //], [%%, \\], [+, -]");
}