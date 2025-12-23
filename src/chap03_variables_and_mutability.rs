pub fn var() {
    // not allowed, as x is immutable
    // let x = 10;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // mutable - so value can be changed
    // it signals the programmer that the value of 'x' has been changed later
    let mut x = 10;
    println!("The value of x is: {x}");
    x = 11;
    println!("The value of x is: {x}");
}

pub fn constants() {
    const SPEED_OF_LIGHT: u32 = 3 * 100_000_000;
    println!("This is the speed of light and cannot be changed(pun intended): {SPEED_OF_LIGHT}")
}

pub fn shadowing_vs_mutability() {
    let x = 10; // x is immutable, so we 'cannot' assign a new value to it
    println!("1st value of x is: {x}"); // 10
    let x = x + 2; // however, we can shadow the previous value by essentially re-creating 'x'
    println!("2nd value of x is: {x}"); // 12
    {
        let x = x * 2;
        println!("3rd value of x inside a scope is: {x}"); // 24
    }
    println!("4th value of x outside the inner scope is: {x}"); // 12

    // shadowing also lets you change the type of the variable
    let x = "Rajneesh Mishra"; // a 'string literal'
    println!("5th value of x is: {x}"); // Rajneesh Mishra

    // however the following is NOT allowed
    let mut spaces = "    ";
    println!("spaces: {spaces} - its literally spaces!");
    // spaces = spaces.len(); // this will result in an error saying that mutating a variable's type is NOT allowed!!

    let spaces = spaces.len(); // this is allowed, as we are just shadowing the variable
    println!("Shadowed spaces: {spaces}");
    let mut spaces = "  ".len(); // this is allowed, as we are just shadowing the variable
    println!("Shadowed spaces: {spaces}");
}

pub fn check() {
    // let x: i8 = 3 * 100_000; // this will result in an error saying that the value is out of range
    let x: i32 = 100_000_000;
    println!("The value of x is: {x}");
    println!("This is how you can calculate expressions: {}", x - x);
    // println!("This is how you can calculate expressions: {x-x}"); // and NOT like this
}
