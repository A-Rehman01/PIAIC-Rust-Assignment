#[derive(Debug)]
struct Student
{
    name : String,
    email : String,
    phone_number : u32,
    gender : String,
}

fn main() {
    println!("Q6\n ");

    let student1=Student
    {
        name:String::from("Hamza"),
        email:String::from("hamza123@gamil.com"),
        phone_number : 03234401623,
        gender:String::from("Male"),
    };
    
    let student2=Student
    {
        name:String::from("Ali"),
        email:String::from("ali125@hotmail.com"),
        phone_number : 0569874525,
        gender:String::from("female"),
    };

    println!("Email of Student_1 is {}",student1.email);
    println!("Whole Data of Student2 is \n` {:#?}",student2);
}
