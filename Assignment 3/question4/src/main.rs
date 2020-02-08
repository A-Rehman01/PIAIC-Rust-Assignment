//use std::io;
#[derive(Debug)]
enum Shape
{
    Circle(String),
    Triangle(String),
    Rectangle(String),
    Square(String),
}



fn main() {
    println!("Q4");

    let cir=Shape::Circle(String::from("Radius for Circle"));
    let tri=Shape::Triangle(String::from("Lenght of three sodes for Triangle"));
    let rec=Shape::Rectangle(String::from("Lenght of four sodes for Rectangle"));
    let squ=Shape::Square(String::from("Lenght of four sodes for Square"));

    cir.call();
    tri.call();
    rec.call();
    squ.call();
}
impl Shape{
    
    fn call(&self){
        println!("{:?}",self)
    }    

}