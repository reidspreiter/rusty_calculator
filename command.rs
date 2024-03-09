use crate::test;

// Execute commands
pub fn execute_command(command: &str) -> i8 {
    match command {
        "/help" => println!("Help"),
        "/reset" => println!("Reset"),
        "/quit" => return 1,
        "/test" => test::test(),
        _ => println!("{} is not a valid command.", command)
    }
    0
}