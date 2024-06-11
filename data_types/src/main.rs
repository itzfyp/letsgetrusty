fn main() {
    println!("Hello, world!");

    // bool
    // ====================================================================================================
    let isValid: bool = true;

    // bools are one byte in size

    // Integers Types
    // ====================================================================================================

    // Default size of integers : u32

    // Unsigned - It should be positive values only
    // To calulcate the size of each type  : 2^n - 1
    
    let v1: u8 = 8;
    
    let v2: u16 = 16;

    let v3: u32 = 32;
    
    let v4: u64 = 64;
    
    let v5: u128 = 164;
    
    // Signed - It could either contain negative or possitive values
    // To calulcate the size of each type  : -2^(n-1) to 2^(n-1) - 1
    
    let s1: i8 = -1;

    let s2: i16 = 32;

    let s3: i32 = 64;

    let s4: i64 = 150;

    let s5: i128 = 200;


    /* 
        Notes: Integer overflow
            - Integer overflow is being handled differently in two modes
                - debug mode
                    - checker will be included to validate type
                    - rust panic's and throws error at comopile time
                - release mode
                    - checker will not included
                    - overflowing values will be wrapped around 
                
            - There are explicit way to handle the overflow. Check Data Types in rust doc   


     */


    // String Types
    // ====================================================================================================
    
    // chart - four bytes in size and represents unicode scaler value
    let c: char = 'J'; 
    
    // str
    let str: &str = "string literal";
    
    // String
    let string: String =  String::from("String Type")
    
    // float types
    // ====================================================================================================

    // Default float size : f64

    
    let f1: f32 = 1.01;
    
    let f2: f64 = 1.000000001;
    
    // Pointer (system architecture based) size
    // ====================================================================================================
    
    let signed_pointer_size: isize = 100;
    
    let unsigned_pointer_size: usize = 10000;
    
    // Array type
    // ====================================================================================================

    /*
        Notes:
            - Array must have same type of values
            - Arrays have fixed length once declared
            - Array values are stored in stack instead of heap
            - Unlike Vector, it can not grow or shrink dynamically
            - if you are unsure which type to use, prefer to use vector over array 
    
     */

    let a1: [u32; 5] = [1,2,3,4,5];


    // array with same repeated value can be decliared below

    let a2: [i32; 6] = [3,3,3,3,3,3];

    // equalent to let a2 = [3; 6]

    // Accessing array values
    let first = a1[0];
    let second = a2[1];


    // Tuple type
    // ====================================================================================================

    // fixed size. Cant grow or shirnk once declared

    let tup: (u8, char, f64) = (100, 'c', 0.124);

    // Accesssing tuple
    // Destructing:
        let (z,zz,zzz) = tup;

    // Dot notation
        let xx = tup.0;
        let xxx = tup.1;
        let xxxx = tup.2;

    // Unit type
    // ====================================================================================================

    // tuple without any values means empty tuple is called unit. 
    // Type of the unit is alos () empty. 
    // it's kind of equalent to (void 0) in javascipt. returns nothing

    let empty_tup: () = ();




}
