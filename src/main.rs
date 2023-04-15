use std::env; 
use std::fs; 
use colorized::*; 

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let path = &args[1]; 
    let pattern = &args[2..].join(" "); 
    
    let file_content = fs::read_to_string(path) 
        .expect("An error ocurred trying to read the file"); 
    
    let mut counter = 1; // counter accesing lines
    
    for line in file_content.lines() {
        let match_line = check_string(line, pattern); 

        if !match_line.is_empty() {
            println!("[{}] {}", counter, match_line); 
        } 

        counter += 1; 
    }
}

fn check_string(input: &str, pattern: &str) -> String {
    let offset = 14; 
    let mut matches = String::new(); 

    for i in 0..input.len() {
        // avoid out of bounds
        if i+pattern.len() > input.len() {
            continue;
        }

        let current_slice = &input[i..i+pattern.len()]; 

        if pattern == current_slice {
            // check if the offset is bigger than the 
            // string slice pre-side
            let pre_string = if offset > input[..i].len() {
                &input[..i] 
            } else {
                &input[i-offset..i]
            };
            
            let post_string = if offset > input[i..].len() {
                &input[i..] 
            } else {
                &input[i+pattern.len()..i+offset]
            };

            let center = colorize_this(current_slice, Colors::BlueFg); 

            let formatted_string = format!("…{}{}{}…", pre_string, center, post_string); 
            matches = formatted_string; 
            break; 
        }
    }

    matches
}