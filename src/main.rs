use std::io;
use std::io::prelude::*;

fn main() {
let mut name = String::new();


    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    if let Some('\n') = name.chars().next_back() {
        name.pop();

    }

    if let Some('\r') = name.chars().next_back() {
        name.pop();
    }

    println!("Hello, {}!", name);
    print!("Type <Enter> for exit...");
    io::stdin().read(&mut [0u8]).unwrap();
}
