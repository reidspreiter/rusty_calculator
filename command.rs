use crate::test;
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
        ("reset".to_string(), false),
    ].iter().cloned().collect();
    settings
}


// Execute commands
pub fn execute_command(command: &str, variables: &HashMap<char, String>, settings: &mut HashMap<String, bool>) {
    match command {
        "/help" => help(),
        "/reset" => {},
        "/variables" => {},
        "/quit" | "/exit" | "/x" => { settings.insert("quit".to_string(), true); },
        "/test" => { test::test(); },
        "/reveal" => { 
            settings.insert("reveal".to_string(), !settings["reveal"]); 
            println!("Reveal changed to {}", settings["reveal"]);
        },
        _ => println!("{} is not a valid command.", command)
    }
}

fn help() {

}