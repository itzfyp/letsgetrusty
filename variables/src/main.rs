fn main() {

   // Creation
   // ====================================================================================================


   let a = 10;

   /*
      Notes
         - Vraibles should be delcared using `let` with type annotation
         - By default rust inferes type from the values is assigned
         - 
    */


    println!("Creation : a is : {a}");

    
    // Mutability
    // ====================================================================================================
    
    
    // a = 100; 
    
    /*
    Notes
    - Above statement is invalid.
    - A variable can't be mutated directly by assiging the value to it.
    - To make a vaiable mutable, it should be declared with `mut` keyword
    */
    
    let mut b = 10;
    
    b = 100;
    
    
    println!("Mutability : b is : {b}");
    
    // Shadowing
    // ====================================================================================================
    
    let c = 10;
    let c = 100;
    
    println!("Shadowing : c is : {c}");
    
    /* 
    Notes:
    - Variable with same name will be shadowing by the latter and former will be unused
    */
    
    // Scope
    // ====================================================================================================

    {
      let a = 100;
      println!("Scope : a is in inner block: {a}");
   }


   println!("Scope : a is in outer block: {a}");

   /*
      Notes:
         - Variables are scoped by {}
         - Inner scope variable can access from outer scope
    */

   
}
