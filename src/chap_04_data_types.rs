// Total 2 types: Scalar and Compound
// Scalar: Integers, Floating-Point numbers, Booleans, Characters
// Compound: Tuples, Arrays

pub fn scalar_types() {
    let x = 100_000; // defaults to i32 (i32 => 32 in i32 = 32 bits = -2^31 to (2^31)-1)
    // signed integer types: i8, i16, i32, i64, i128, isize(Architecture Dependent)
    // unsigned integer types: u8, u16, u32, u64, u128, usize(Architecture Dependent)
    println!("The value of x is: {x}");

    let hex = 0xf; // hex literal -> i32 by default - same as other integers
    println!("The value of hex in decimal is: {hex}");

    let octal = 0o77; // octal literal -> i32 by default - same as other integers
    println!("The value of octal in decimal is: {octal}");

    let binary = 0b111_000; // binary literal -> i32 by default - same as other integers
    println!("The value of binary in decimal is: {binary}");

    let byte = b'A'; // byte literal -> u8 'always' -> 1 byte = 8 bits (u8 = 8 unsigned bits)
    println!("The value of byte in decimal is: {byte}");

    let y = 3.0; // defaults to f64
    // floating point types: f32 and f64 - all are signed - roughly operate at same speed in the CPU, but precision of f64 is more because of more bits(64)
    println!("The value of y is: {y}");

    let boolean = true; // bool type
    println!("The boolean value is: {boolean}");

    let c = 'Z'; // 'char' type
    println!("The value of c is: {c}");
    let c: char = 'K';
    println!("The value of c is: {c}");

    let emoji: char = '❤'; // heart emoji
    println!("emoji: {emoji}")
}

pub fn numeric_operations() {
    let add = 3 + 2; // 5
    println!("Add: {add}");
    let diff = 2 - 3; // -1 
    println!("Diff: {diff}");
    let mul = 2 * 3; // 6
    println!("Mul: {mul}");
    let quotient = 2 / 3; // 0, integer/integer = integer 
    println!(
        "The value of quotient is truncated towards 0 for integer/integer expressions: {quotient}"
    );

    let truncated = -5 / 2; // -2
    println!("Truncated: {truncated}");

    let remainder = 3 % 2; // 1
    println!("Remainder: {remainder}");
}

pub fn compound_types() {
    let tup: (i32, &str, f64) = (500, "Rajneesh", 32.32); // fixed size, different types allowed, allocated in stack
    let (x, y, z) = tup;
    println!("x:{x}, y: {y}, z: {z}"); // destructuring a tuple
    println!("tup.0: {}", tup.0); // member access via '.' operator
    println!("tup.1: {}", tup.1);
    println!("tup.2: {}", tup.2);

    let unit: () = (); // () -> unit type
    // - represents an empty value or an empty return type.
    // - expressions implicitly return the unit value if they don’t return any other value.
    // - in rust almost everything is an expression, functions are declarations/items and their bodies are blocks/expressions, if, else, match, blocks, loops, assignments -> all these are expressions.
    // - Rust is an expression oriented language.

    let a = [1, 2, 3, 4]; // arrays -> fixed size, same type, allocated on stack
    // a[0] = 100; // error as 'a' is immutable
    // println!("Out of bounds access isn't allowed: {}", a[5]); // results in a compile time/build error, for out of bounds access
    println!("3rd index element in a: {}", a[3]);
    let typed_array: [i32; 5] = [1, 2, 3, 4, 5]; // [i32; 5] => 5 items of type i32

    let mut mutable_array = [1, 2, 3, 4, 5];
    mutable_array[0] = 100;
    println!(
        "Zeroth index element of mutable array: {}",
        mutable_array[0]
    );

    let k = [3; 5]; // [3, 3, 3, 3, 3] -> 3 five times
    println!("Zeroth index element of k: {}", k[0])
}
