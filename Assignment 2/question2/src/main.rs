fn main() {
    println!("Q2");

    let mut s=String::from("Pakistan");
    complete(&mut s);

    println!("{}",s);
}
fn complete(s :&mut String)
{
    s.push_str(" Zindabad");
}