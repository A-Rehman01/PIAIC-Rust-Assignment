fn main() {
    println!("Q9\nCalculate percentage in one Subject ");
    mark_sheet(92);
    mark_sheet(32);
    mark_sheet(62);
}
fn mark_sheet( x:i32){

    if x>=80{
        println!("Grade A+");
    }
    else if x>=70 && x<80{
        println!("Grade A");
    }
    else if x>=60 && x<70{
         println!("Grade B");
    }
    else if x>=50 && x<60{
         println!("Grade C");
    }
    else if x>=40 && x<50{
         println!("Grade D");
    }
    else {
         println!("Grade F");
    }
}
