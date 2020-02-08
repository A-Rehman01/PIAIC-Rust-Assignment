#[derive(Debug)]
enum Vehicle
{
    Cars(String),
    Truck(String),
    Bikes(String)
}
fn main() {
    println!("Q3");

    let vech1=Vehicle::Cars(String::from("Red Murcedies"));
    let vech2=Vehicle::Truck(String::from("Oil Truck"));
    let vech3=Vehicle::Bikes(String::from("Suzuki"));

    println!("{:#?}",vech1);
    println!("{:#?}",vech2);
    println!("{:#?}",vech3);
}
