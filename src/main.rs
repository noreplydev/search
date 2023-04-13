use std::io::stdin; 
use colorized::*; 

fn main() {
    let stdin = stdin();
    let mut input = String::new(); 

    stdin.read_line(&mut input).unwrap(); 

    let matches = check_string(&input); 
    
    if matches.is_empty() {
        println!("The regex was not found in {}", input); 
        std::process::exit(0)
    }  

    println!("> {}", matches)
}

fn check_string(input: &String) -> String {
    let to_match = "hola";

    let mut matches = String::from("Not found"); 

    for i in 0..input.len() {
        // avoid out of bounds
        if i+to_match.len() > input.len() {
            continue;
        }

        let current_slice = &input[i..i+to_match.len()]; 

        if to_match == current_slice {
            let pre_string = &input[..i]; 
            let post_string = &input[i+to_match.len()..]; 
            let center = colorize_this(current_slice, Colors::BlueFg); 

            let formatted_string = format!("{}{}{}", pre_string, center, post_string); 
            matches = formatted_string; 
            break; 
        }
    }

    matches
}