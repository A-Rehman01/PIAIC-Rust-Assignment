#[derive(Debug)]
struct Triangle
{
    lenght1:u32,
    lenght2:u32,
    lenght3:u32,
}

impl Triangle
{
    fn sum_of_all_sides(&self)->u32
    {
        self.lenght1+self.lenght2+self.lenght3
    }

    fn largest_side(&self)->u32
    {
        let mut l=self.lenght1;
        if(l<self.lenght1){
            l=self.lenght1;
        }
        else if(l<self.lenght2)
        {
            l=self.lenght2;
        }
        else if (l<self.lenght3)
        {
            l=self.lenght3;
        }
        l
    }
}
fn main() {
    println!("Q9");

    let tri=Triangle
    {
        lenght1:25,
        lenght2:80,
        lenght3:60,
    };
 
    println!("Sum of all sides of Triangle {}",tri.sum_of_all_sides());
    println!("Largest side {}",tri.largest_side());
}
