use std::{env, process}; 
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("query: {}", config.query);
    println!("file_path: {}", config.file_path);
    
    if let Err(e) = minigrep::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}



