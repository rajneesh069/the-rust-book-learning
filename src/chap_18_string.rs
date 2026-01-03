#[allow(unused)]

pub fn start() {
    let s = String::new();
    // to_string() works on types which implement the "Display" trait
    let data = "I am Rajneesh Mishra"; // string literal
    let s = data.to_string(); // or "I am Rajneesh Mishra".to_string() will also work ;)
    let s = String::from("I am Rajneesh Mishra");
    let hello = String::from("नमस्ते"); // UTF-8 encoded

    let mut str = String::from("Hello");
    str.push_str(", World!"); // takes reference of the literal, so the value is borrowed
    println!("{}", str);

    let mut s1 = String::from("lo");
    s1.push('l'); // for pushing characters

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 has been moved so cannot be used further
    // '+' calls the add method which has the following signature(its actually generic but for this we'll insert the actual types): fn add(self, s:&str) - Notice that there's no '&' with 'self' meaning that the ownership will be moved when add will be called and for 's' there's an 'ampersand' that's why we need to pass the second parameter with the reference, but since s2 is a string and NOT a string slice, Rust automatically performs deref coercion and makes &s2 -> &s2[..]. s2 is still usable after the 'add' operation but s1 is NOT.

    // Concatenating multiple strings together
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = String::from("Let's get");
    let s4 = String::from(" Rusty!");
    let s = s1 + &s2 + &s3 + &s4; // but this looks unwieldy, hence we'll use "format!" macro

    let s1 = String::from("Hello, ");
    let s = format!("{s1}{s2}\n{s3}{s4}");
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // "format!" uses references so strings are still usable after it.
    println!("{s}"); // tic-tac-toe
    println!("{s1}"); // tic

    // Indexing odf Strings in Rust
    let s1 = String::from("Hey");
    // let h = s1[0]; // ILLEGAL, Rust strings don't support indexing

    // UTF-8 encoding
    let hello = String::from("Здравствуйте"); // the length we expect is 12, but actually its 24
    // the main reason why it is 24 and NOT 12 is because it takes 2 bytes to encode all the characters(Unicode scalars) - this proves that NOT always the indices relate to the valid Unicode scalar value. That's why Rust doesn't support indexing.
    // let answer = &hello[0]; // ILLEGAL: When encoded in UTF-8, the first byte of З is 208 and the second is 151, so it would seem that answer should in fact be 208, but 208 is not a valid character on its own. Returning 208 is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index 0. Users generally don’t want the byte value returned, even if the string contains only Latin letters: If &"hi"[0] were valid code that returned the byte value, it would return 104, not 'h'.

    let hello = String::from("नमस्ते");
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // Above is the byte representation of the Hindi word: "नमस्ते"
    // And it is 18 bytes and that's how computers store the data.
    // Unicode scalars: ['न', 'म', 'स', '्', 'त', 'े']
    // The above scalar has 6 values but 4th and 6th values don't make sense because they are diacritics and NOT letters of the Devanagari script. Below are the "Grapheme clusters" which people will call four letters and will make sense to them
    // ["न", "म", "स्", "ते"]

    // Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in. A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // valid slice because each character = 2 bytes
    // If we were to try to slice only part of a character’s bytes with something like `&hello[0..1]`, Rust would panic at runtime in the same way as if an invalid index were accessed in a vector

    // Iterating over strings
    for c in "Зд".chars() {
        // this will separate out "Unicode scalars(characters)" and print them.
        println!("{c}");
    }

    for b in "Зд".bytes() {
        // this will print the literal bytes of each Unicode scalar which are 4 bytes because each character is of 2 bytes
        println!("{b}");
    }

    // Getting grapheme clusters isn't supported by Rust out of the box, crates can be found online to do so.
}
