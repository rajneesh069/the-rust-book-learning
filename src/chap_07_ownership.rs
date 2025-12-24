// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
// ! Borrowing and Lifetimes will be discussed later....

pub fn ownership() {
    let s: &str = "hello";
    // * The string literal `"hello"` is stored in the program's read-only binary (RODATA)
    // *`s` is a reference (pointer + length) stored on the stack

    // ? -- DON'T FRET ABOUT IT JUST NOW ---
    // `s` is of type `&'static str`(coerced from just 'str' to `&'static str` because of the context)
    // It is borrowed from a string literal with static lifetime
    // The string data has static lifetime and is never dropped
    // Only the reference `s` goes out of scope here
    // ---------

    {
        // `k` is NOT valid here
        let k: &str = "hello, world!"; // `k` is valid from this point forward
        // do stuff with `k`, like print it or something
        println!("{k}");
    } // `k` goes out of scope and now if we try to access it from this point forward, it will cause compile errors
    // println!("{k}"); // ! ILLEGAL

    let string = String::from("hello");
    // * The `String` owns heap-allocated string data("hello" in this case)
    // * The `String` value{`string``} (ptr, len, cap) lives on the stack

    // ? -- DON'T FRET ABOUT IT JUST NOW ---
    let s: &String = &string;
    // `s` is a non-`'static` `&str`
    // It is borrowed from a `String`
    // The lifetime of `s` is tied to the lifetime of `str`
    // -------
}

pub fn data_movement() {
    let x = 10;
    let y = x; //* x's is copied into `y`
    println!("The value of x and y are: {x}, {y}.");

    let s1: String = String::from("hello"); // * Checkout image: assets/chap_07_1.png
    let s2: String = s1; // * Checkout Possibility 1: assets/chap_07_2.png 

    // * Checkout Possibility 2: assets/chap_07_3.png - this is what will happen if Rust actually copied the string, but it will cause runtime issues if the data on the heap is large!

    // Poss. 1 creates a `double free error`, because if `s1` and `s2` go out of scope they'll try to `free` the same memory location together causing an error
    // Freeing memory twice can lead to memory corruption.

    // ? This is why Rust NO LONGER considers `s1` valid after the line: let s2 = s1, so in a way the value got "moved".

    // println!("{s1}"); // ! ILLEGAL

    // copying the pointer, length, and capacity without the data - shallow copy
    // copying the pointer, length, capacity and data - deep copy
    // ? Since, Rust invalidates the first variable as well therefore it's called a "move"! -> Checkout image: assets/chap_07_4.png

    // ? Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    let mut s: String = String::from("hello");
    s = String::from("ahoy");
    // "hello" is removed from memory, as nothing points to it. -> Checkout: assets/chap_07_5.png
}

pub fn clone() {
    // For deep copies, we can clone them then

    let s1 = String::from("hello");
    let s2 = s1.clone(); // this creates the deep copy, ofc there are runtime performance overheads/implications to it, this is same as: assets/chap_07_3.png

    println!("s1: {s1}, s2: {s2}");
}

pub fn stack_data_copy() {
    let x = 10;
    let y = x; // now this creates copy(and doesn't get moved) without any clone method! Why? Because we know the size of integers at compile time that's why there won't be any benefit to stop it from being copied, hence NO difference between a shallow or a deep copy here anyway, as there are NO performance implications, since copying data in stack is fast anyways.
    println!("x = {x}, y = {y}");

    // ? Also the types which implement the `Copy trait`(traits will be discussed later), they directly get copied, i.e., they don't get moved

    // ! Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. `drop` function is what's called to free the memory and give it back to the allocator.

    // * Copy trait list:
    // ? All the integer types, such as u32.
    // ? The Boolean type, bool, with values true and false.
    // ? All the floating-point types, such as f64.
    // ? The character type, char.
    // ? Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
}

pub fn some_fn() {
    // the mechanics of passing a value to a function is similar to assigning a value to a variable.
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    // println!("{s}"); // ! ILLEGAL
    let s1 = gives_ownership(); // gives the ownership by returning a string and moves it to s1
    println!("{s1}");
    let s2 = String::from("mine!");
    let s3 = takes_ownership_and_gives_back(s2);
    println!("{}", s3);
} // * at this point when s1 and s3 go out of scope, they are dropped, and s2 was already invalidated the moment it went to takes_ownership_and_gives_back function, so nothing happens for it.

fn takes_ownership(str: String) {
    println!("I own: {str}");
}

fn makes_copy(x: u32) {
    println!("I made the copy of x: {x}");
}

// let's try returning the value from takes_ownership though
fn takes_ownership_and_gives_back(s: String) -> String {
    println!("{s}");
    s
}

fn gives_ownership() -> String {
    let s: String = String::from("hello");
    s
}

fn multiple_value_returns_using_a_tuple(s: String) -> (usize, String) {
    let s = String::from("Hello, World!");
    return (s.len(), s);
}
