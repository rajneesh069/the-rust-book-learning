pub fn start() {
    let s = String::from("Hello, World!");
    let (s, len) = calculate_length_of_a_string_without_borrowing(s);
    println!("The size of string '{s}' is: {len}.");
    // ? Now as we can see, we had to actually return the string to the caller function in order for it to be used again(used shadowing for the same name, lol!) but this seems very tedious for a simple task like calculating the length of the string.

    // * For this very reason we'll use the concept of "references" and "borrowing".
    // * So now, let's pass a reference of the string to the calculate length function and see how things unfold:

    let len = calculate_length(&s); // * passing the reference, denoted via '&variable_name'
    println!("The size of '{s}' is: {len}.");

    // change `s`
    // change_the_value(&s); // ! ILLEGAL -> this shows that references are immutable by default just like variables!

    let mut s = String::from("hello");
    change(&mut s); // now this is allowed

    // ? but 2 mutable references at the same time are NOT allowed - this saves us from data races and synchronization issues
    let r1 = &mut s;
    // let r2 = &mut s; // ! ILLEGAL
    println!("{r1}"); // the moment we "use" any of the mutable references (if there are multiple) it gives an error, because unused variables don't throw any errors - thats why declaration won't cause any problem, but usage will and that has been explored below.

    let s = String::from("Hello, World!");
    let r1 = &s; // * NO PROBLEM
    let r2 = &s; // * NO PROBLEM
    let r3 = &s; // * NO PROBLEM
    println!("r1: {r1}, r2: {r2}, r3: {r3}");

    // let r4: &mut String = &mut s; // ! BIG PROBLEM
    // ?^ You can use multiple immutable references at the same time, because they cannot change the value, but you cannot use single and/or multiple mutable and immutable references at the same time!

    // ? Either all(multiple) references are immutable - no problem, or just a single one is mutable and if now we wanna use a immutable one then either it should go out of scope or all the usages of it should end before/after the mutable one is used, i.e., NOT in the overlapping 'lifetime'/NOT at the same time - Why? (Answered below)

    let mut s = String::from("Hello World"); // * This is allowed because of shadowing
    {
        let r4: &mut String = &mut s;
        println!("{r4}");
    } // Here r4 goes out of scope so it causes no problem ofc
    {
        let r5: &mut String = &mut s;
        println!("{r5}");
    } // Here r5 goes out of scope so it causes no problem ofc
    let x1 = &s;
    let x2 = &s;
    let x3 = &s;
    println!("x1 = {x1}, x2 = {x2}, x3 = {x3}");
    let x4 = &mut s;
    println!("x4 = {x4}"); // Till here its fine, all the immutable ones are used above the mutable one and the mutable one is at last
    // println!("x1 = {x1}"); // ! println!("x4 = {x4}, x1={x1}"); mutability signifies change to the compiler even while using it we do not change it actually because that can only be figured out at the runtime(just print it and do nothing lol!) and compiler doesn't have any way to figure it out so that's why it asks for exclusiveness with mutable references just to guarantee runtime safety at all costs, and immutable references guarantee unchangability hence this is ILLEGAL - This is the explanation

    // * This is also allowed!
    {
        let x4 = &mut s;
        println!("x4 = {x4}"); // Its fine, all the immutable ones are used below the mutable one and the mutable one is at top, so no change below!
        let x1 = &s;
        let x2 = &s;
        let x3 = &s;
        println!("x1 = {x1}, x2 = {x2}, x3 = {x3}");
        // println!("x4 = {x4}"); // ! mutability signifies change, and immutable references guarantee unchangability hence this is ILLEGAL as well
    }
}

fn calculate_length_of_a_string_without_borrowing(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length(s: &String) -> usize {
    // `s` is a reference to the string: Checkout image: assets/chap_08_1.png
    s.len()
} // * Here, `s` goes out of scope but since `s` does NOT have ownership of the value it refers to, the String in the caller isn't dropped! - and `references` are the way to `borrow` the value

fn change_the_value(s: &String) {
    // s.push_str(", world!"); // ! ILLEGAL
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// * Rules of borrowing and references in Rust:
// 1. At any point in time, only one mutable (read/write) reference to a value may be in use.
// 2. At any point in time, any number of immutable (read-only) references  may be in use.
// 3. If you're intermixing the mutable and immutable ones keep the following things in mind:
//     a. At any point in time, a value can have either one mutable reference or any number of immutable references, but never both while they're in use.
//     b. And only one mutable reference can exist in this case as well.
