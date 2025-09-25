use std::io;
use std::io::prelude::*;

fn main() {
    let mut base  = String::new();
    let mut index  = String::new();

    let bas: f64;
    let ind: f64;


    print!("Enter the value of base: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).unwrap();
    bas = base.trim().parse::<f64>().unwrap();

    print!("Enter the value of index: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut index).unwrap();
    ind = index.trim().parse::<f64>().unwrap();


    println!();
    println!("Exponentiation: {:8.2}", bas.powf(ind));
    println!("Square root: {:8.2}", bas.sqrt());
    println!("Cube root: {:8.2}", bas.cbrt());




    println!();
    print!("Press <Enter> to exit");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();


}
