pub mod Calculator_functions{
    pub mod basic_functions{
        pub fn add(a:i32,b:i32)->i32
        {
            a+b
        }
        pub fn subtract(a:i32,b:i32)->i32
        {
            a-b
        }
        pub fn multiply(a:i32,b:i32)->i32
        {
            a*b
        }
        pub fn divide(a:i32,b:i32)->i32
        {
            a/b
        }
        
    }
    pub mod Power_functions{
        pub fn Square_function(a:i32)->i32
        {
            a*a
        }
        pub fn Cube_function(a:i32)->i32
        {
            a*a*a
        }
        pub fn power_function(a:i32,b:i32)->i32
        {
            let mut c=1;
            for x in 0..b{
                c=c*a;
            }
            c
        }
        
    }

}