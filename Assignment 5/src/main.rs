use std::io;
#[derive(Debug)]
struct NewArticle
{
    auther:String,
    content:String
}
#[derive(Debug)]
struct Tweet
{
    username:String,
    content:String
}

pub trait Summary
{
    fn summarize(&self)->String;
}

impl Summary for NewArticle
{
    fn summarize(&self)->String{
        String::from("Auther :This Article is for new youth who demotivated:The auther in this Article wants the new Generation youth dont be affraid and try/give his/her best for their Destination")
    }
}

impl Summary for Tweet
{
    fn summarize(&self)->String
    {
        String::from("Tweeter: I really appricaiated your tweet and konw i give my best and try to reach destination at  any cost")
    }
}

fn main() {
    println!("Question1");

    println!("Enter Auther Name");
    let mut aath=String::new();
    io::stdin().read_line(& mut aath);
    let aath: String=aath.trim().parse().unwrap();
    println!("Enter Auther Content");
    let mut acon=String::new();
    io::stdin().read_line(& mut acon);
    let acon: String=acon.trim().parse().unwrap();
    println!("Enter username of tweeter");
    let mut tuser=String::new();
    io::stdin().read_line(& mut tuser);
    let tuser: String=tuser.trim().parse().unwrap();
    println!("Enter Tweet Content");
    let mut tcon=String::new();
    io::stdin().read_line(& mut tcon);
    let tcon: String=tcon.trim().parse().unwrap();
    let article1=NewArticle
    {
        auther:aath,
        content:acon
    };

    let tweet1=Tweet{
        username:tuser,
        content:tcon
    };
    println!("{:?}",article1);
    println!("{:?}",tweet1);
    println!("{:?}",article1.summarize());
    println!("{:?}",tweet1.summarize());
}
