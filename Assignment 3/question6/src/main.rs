#[derive(Debug)]
enum Laptops
{
   Hp,
   Dell(Dell),
   Asus,
   Lenovo
}

#[derive(Debug)]
enum Dell
{
    Series1000,
    Series2000,
    Series3000,
    Series4000,
    Series5000,
    Series6000
    
}
fn call(laptop:Laptops)
{
    match laptop
    {
        Laptops::Lenovo=>
        {
            println!("Lenovo Laptop")
        },
        Laptops::Asus=>
        {
            println!("Asus Laptop")
        },
        Laptops::Hp=>
        {
            println!("hp LAptop")
        },
        Laptops::Dell(Dell::Series1000)=>
        {
            println!("Dell series 10000 Laptop")
        },
        Laptops::Dell(Dell::Series2000)=>
        {
            println!("Dell series 20000 Laptop")
        },
        Laptops::Dell(Dell::Series3000)=>
        {
            println!("Dell series 30000 Laptop")
        },
        Laptops::Dell(Dell::Series4000)=>
        {
            println!("Dell series 40000 Laptop")
        },
        Laptops::Dell(Dell::Series5000)=>
        {
            println!("Dell series 50000 Laptop")
        },
        Laptops::Dell(Dell::Series6000)=>
        {
            println!("Dell series 60000 Laptop")
        },
        _ =>println!(" IN-VALID "),

    }
}

 
fn main() {
    println!("Q6");


    call(Laptops::Lenovo);
    call(Laptops::Dell(Dell::Series3000));
    call(Laptops::Asus);
    call(Laptops::Dell(Dell::Series5000));
}
