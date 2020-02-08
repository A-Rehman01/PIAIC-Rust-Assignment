#[derive(Debug)]
struct Rectangle
{
    width :u32,
    height :u32,
}

fn main() {
    println!("Q7");
    
    let mut rect = Rectangle
    {
        width:50,
        height:100,
    };

    println!("{:#?}",rect);
    change_width(&mut rect);
    println!("\n After change width \n");
    println!("{:#?}",rect);
}
fn change_width(rect : &mut Rectangle)
{
    rect.width=100;
}
