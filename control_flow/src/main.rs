fn main() {
    // if
    // ====================================================================================================

    /*
    Notes:
           - if can handle bool value
           - unlike js, it expects only bool in comparition block
           - if else can return value, if last value of each block returns same type of value expression
           - if values are not equal, throws mimatch error
           */

    let number = 5;

    if number < 10 {
        println!("if block");
    } else if number == 5 {
        println!("else if block");
    }

    // example with let
    let outout = if number < 10 { 5 } else { 2 };

    // Example of mimatched return type from if blocks
    // let outout1 = if number < 10 { 5 } else { "2" };

    // Rust has three kinds of loops: loop, while, and for
    // ====================================================================================================

    // loop
    // ====================================================================================================

    // infinite loop
    /* loop {
        println!("looping infinitly")
    } */

    // loop returns value

    let mut count = 0;

    let is_max_reched = loop {
        count += 1;

        if count > 100 {
            break true;
        }
    };

    println!("is max reached: {is_max_reched}");

    // loop with label

    let mut start = 0;

    'outer: loop {
        println!("outer loop : {}", start);

        let mut next = 0;

        'inner: loop {
            println!("-- inner loop : {}", start);

            if next == start {
                break 'inner;
            }

            next += 1;

            if start == 5 {
                break 'outer;
            }
        }

        start += 1;
    }

    // while
    // ====================================================================================================

    // standard while loop is used like JS

    // for
    // ====================================================================================================

    // In Rust, for..in loop is available to iterate over array values
    // To reverse array value, use array.rev() method
}
