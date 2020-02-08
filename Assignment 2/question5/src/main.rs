use std::io;
fn main() {
    println!("Q5");

    println!("Enter Number please");
    let mut a=String::new();
    io::stdin().read_line(&mut a);
    let a:i32=a.trim().parse().unwrap();    
    
    for mut x in (1..=a).rev(){
     
        while x<=a{
            
            print!("*");
            x=x+1;
        }
        println!("\n");
    }
    



}
