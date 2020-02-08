extern crate calculator;
use
calculator::Calculator_functions::basic_functions;
use
calculator::Calculator_functions::Power_functions;
pub fn main()
{
    let mut a;
    let b=10;
    let c=5;
    a=basic_functions::add(b,c);
    println!("{}+{}={}",b,c,a );
    a=basic_functions::subtract(b,c);
    println!("{}-{}={}",b,c,a );
    a=basic_functions::multiply(b,c);
    println!("{}*{}={}",b,c,a );
    a=basic_functions::divide(b,c);
    println!("{}/{}={}",b,c,a );
    a=Power_functions::Square_function(b);
    println!("{} square={}",b,a );
    a=Power_functions::Cube_function(b);
    println!("{} cube ={}",b,a );
     a=Power_functions::power_function(b,c);
    println!("{} ^ {} ={}",b,c,a );

}