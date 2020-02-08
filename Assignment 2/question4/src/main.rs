use std::io;
fn main() {
    println!("Q4");

    println!("Enter Any String");
    let mut a=String::new();
    io::stdin().read_line(&mut a);
    if let Some('\n') =a.chars().next_back()
    {
        a.pop();
    }
   
    
    println!("Lenght of {} is {}",a,lenght(&a));

}
fn lenght(a:&String)->usize
{
    
    a.len()    //a.chars().count()
}