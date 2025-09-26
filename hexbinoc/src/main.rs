use std::io;
use std::io::prelude::*;

fn main() {

    let value = 10;

    println!("Binary: {:b}", value);
    println!("Octal: {:o}", value);
    println!("Hexadecimal: {:x}", value);
    println!();
    println!("ou");
    println!();
    println!("BIN {:b} OCT: {:o} HEX: {:x}", value, value, value);

    println!();
    print!("Type <Enter> for exit...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();




}
