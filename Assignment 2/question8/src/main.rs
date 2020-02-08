#[derive(Debug)]
struct Rectangle
{
    width : u32,
    height : u32,
}
fn main() {
    println!("Q8");

    let  rect = Rectangle
    {
        width:50,
        height:100,
    };

    println!("sum of your first instance {}",sum_rect(& rect));
}
fn sum_rect(rect:& Rectangle)->u32
{
    rect.width+rect.height
}
