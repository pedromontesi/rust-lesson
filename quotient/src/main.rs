use std::io;
use std::io::prelude::*;

fn main() {
    let mut dividend = String::new();
    let mut divisor = String::new();

    let dvd: i64;
    let dvr: i64;
    let rem: i64;
    let quo: i64;

    print!("Please enter the dividend: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dividend).unwrap();
    dvd = dividend.trim().parse::<i64>().unwrap();

    print!("Please enter the value of divisor ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut divisor).unwrap();
    dvr = divisor.trim().parse::<i64>().unwrap();

    quo = dvd / dvr;
    rem = dvd % dvr;


    println!();
    println!("Quotient: {}", quo);
    println!("Remainder: {}", rem);

    println!();
    print!("Press <Enter> to exit: ");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();




}
