fn main() {
    //mutability...

   let mut x=5;
   println!("value of x: {x}");
    x=6;
   
   println!("value of x: {x}");
   // Shadowing...

   let new_var = 30;
   println!("the value of new_var: {new_var}");
   let new_var = 34;
   println!("the value of new_var : {new_var}");

   //shadowing in inner scope..
    let something  = 50;
    println!("the initial value of something is : {something}");
    let something = something +1;
    
    //declare a scope
    {
        let something = something * 2;
        println!("the value in the scope is :{something}");
    }
    println!("the value out of the scope is : {something}");

    //compound data types
    
    //tuples
    let tup  = (-500,6.4,76);
   let (x,y,z)=tup;
   println!("{y}");
   
   //arrays
   let _arr = [1,2,3,4,5,6,7,8];
   let months = ["January", "February", "March", "April", "May", "June", "July",
   "August", "September", "October", "November", "December"];
   let first_month = months[0];

   println!("first month is:{first_month}");
}

