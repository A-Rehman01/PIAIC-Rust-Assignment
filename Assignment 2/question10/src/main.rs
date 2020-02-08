use std::io;
#[derive(Debug)]
struct Person
{
    name : String,
    age : u8,
    country : String,
}

fn main() {
    println!("Q10");

    println!("Enter your NAme");
    let mut n=String::new();
    io::stdin().read_line(&mut n);
    if let Some('\n')=n.chars().next_back()
    {
        n.pop();
    }
    if let Some('\t') =n.chars().next_back()
    {
        n.pop();
    }

    println!("\nEnter your Age");
    let mut a=String::new();
    io::stdin().read_line(&mut a);
    let mut a:u8=a.trim().parse().unwrap();

    println!("\nEnter your Country");
    let mut c= String::new();
    io::stdin().read_line(& mut c);
    if let Some('\n') =c.chars().next_back()
    {
        c.pop();
    }
    if let Some('\t') =c.chars().next_back()
    {
        c.pop();
    }

    let per1=Person
    {
        name:n,
        age:a,
        country:c,
    };

    
    let array=[(per1.name,per1.age,per1.country)];                         //touple in array because different data ypes
    println!("\n{}  {}  {}",array[0].0,array[0].1,array[0].2);
    
}
