use std::io;
use std::io::prelude::*;

fn main() {
    let mut hw_input = String::new();
    let mut hr_input = String::new();
    let mut dp_input = String::new();

    let hw: f64; // hours worked
    let hr: f64; // hourly rate
    let dp: f64; // deduction percent
    let dt: f64; // deduction total
    let gs: f64; // gross salary
    let ns: f64; // net salary

    print!("Enter the number of hours worked: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hw_input).unwrap();
    hw = hw_input.trim().parse::<f64>().unwrap();

    print!("Enter the hourly rate: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hr_input).unwrap();
    hr = hr_input.trim().parse::<f64>().unwrap();

    print!("Enter the deduction percentage: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dp_input).unwrap();
    dp = dp_input.trim().parse::<f64>().unwrap();

    gs = hw * hr;
    dt = (dp / 100.) * gs;
    ns = gs - dt;

    println!();
    println!("Gross salary: {}", gs);
    println!("Payroll deduction: {}", dt);
    println!("Net salary: {}", ns);

    println!();
    print!("Press <Enter> to exit ");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}