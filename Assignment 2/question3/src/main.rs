use std::io;
fn main() {
    println!("Q3");

    println!("Enter first number");
    let mut num1=String::new();
    io::stdin().read_line(&mut num1);
    let mut num1:f64=num1.trim().parse().unwrap();

    println!("Enter second number");
    let mut num2=String::new();
    io::stdin().read_line(&mut num2);
    let mut num2:f64=num2.trim().parse().unwrap();

    println!("Enter third number");
    let mut num3=String::new();
    io::stdin().read_line(&mut num3);
    let mut num3:f64=num3.trim().parse().unwrap();

    let av:f64=(num1+num2+num3)/3.0;
    println!("Your averge  number is {}",av);
}
