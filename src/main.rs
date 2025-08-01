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
}

