use std::io;
use std::io::prelude::*;

fn main() {
    const PI: f64 = std::f64::consts::PI;

    let mut height = String::new();
    let mut radius = String::new();


    let h: f64;
    let r: f64;
    let a: f64;

    print!("Enter the height: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut height).unwrap();
    h = height.trim().parse::<f64>().unwrap();

    print!("Enter the radius: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut radius).unwrap();
    r = radius.trim().parse::<f64>().unwrap();

    a = 2.0 * PI * r / (r + h);

    println!();
    println!("Cylinder area: {:8.2}", a );

    println!();
    print!("Type <Enter> to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();







}
