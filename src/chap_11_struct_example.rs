// Using proper struct to do the same thing
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn example() {
    let w1: u32 = 32;
    let h1: u32 = 23;
    let a = calculate_area(w1, h1);
    println!("The area of the rectangle is: {a}");

    // Using the tuple type
    let rect: (u32, u32) = (23, 32);
    let a = area(rect);
    println!("The area of the rectangle is: {a}");

    let rect = Rectangle {
        width: 32,
        height: 23,
    };

    let a = calculate_area_using_struct(&rect); // used `&` so that the value doesn't get moved!
    println!("The area of the rectangle is: {a}");

    // println!("rect is {rect}"); // gives an error that Rectangle doesn't implement "std::fmt::Display". To show formatted outputs using `{}`, println! macro uses Display trait. For primitives its there but for structs there's not, because of multiple possibilities, like do we want curly brackets or commas or what!

    // println!("rect is {rect:?}"); // now it says `Debug` trait isn't implemented. So let us `#[derive(Debug)]` it for the Rectangle and see what happens.

    println!("rect is {rect:?}");
    println!("rect is {rect:#?}"); // for pretty printing the info

    // we can use the dbg! macro to print the line numbers along with the info! although, it takes the ownership of the value and then returns it to the caller and it prints to the `stderr` unlike println! which prints to the console: `stdout` - more on it later!
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 32,
    };

    println!("rect is {rect:#?}"); // Output of it is below:
    // [src/chap_11_struct_example.rs:37:16] 30 * scale = 60
    // rect is Rectangle {
    // width: 60,
    // height: 32,
    // }
}

fn calculate_area_using_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}
