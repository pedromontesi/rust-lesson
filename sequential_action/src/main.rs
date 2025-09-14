use std::io;
use std::io::prelude::*;

fn main() {
    let mut value = String::new();
    let vlr: i64;

    print!("Enter a integer value: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut value).unwrap();

    vlr = value.trim().parse::<i64>().unwrap();

    println!("value: {}", vlr);

    println!();
    print!("Press <Enter> to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}