#[derive(Debug)]
enum Vehicle
{
    Cars,
    Trucks,
    Bikes
}
#[derive(Debug)]
struct Vehicles
{
    veh_info : String,
    veh_type : Vehicle,
}

fn main() {
    println!("Q2");

    let veh1=Vehicles
    {
        veh_info:String::from("Red Mercedies"),
        veh_type:Vehicle::Cars
    };

    let veh2=Vehicles
    {
        veh_info:String::from("Oil Trucks"),
        veh_type:Vehicle::Trucks,
    };

    let veh3=Vehicles
    {
        veh_info:String::from("Suzuki"),
        veh_type:Vehicle::Bikes,
    };

    println!("{:#?}",veh1);
    println!("{:#?}",veh2);
    println!("{:#?}",veh3);
}
