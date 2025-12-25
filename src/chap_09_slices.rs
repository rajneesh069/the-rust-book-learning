pub fn slices() {
    let mut str: String = String::from("This is Rajneesh Mishra.");
    let word = find_first_words_index(&str);
    println!("{}", word);
    str.clear(); // now this clears the 'str' and now if the importance of 'word' is pointless because there's no string to find the first word for

    println!("{:?}", str.chars().nth(word - 1)); // perfectly fine code and this will compile as well, but it could result in unexpected runtime behavior, because there's no string now. Output is None btw.

    // Having to worry about the index in word getting out of sync with the data in `str` is tedious and error-prone! And if we try to find the second word, then we'll have to use 2 variables, one at the start and other at the end, then along with these unnecessary variables the problem of keeping it in sync is another problem.

    // This is where slices come in, they "reference" a contiguous part of a string!
    // * Formal definiton: A string slice is a 'reference' to a contiguous sequence of the elements of a String.
    // * Checkout image: assets/chap_09_1.png

    let s = String::from("hello world");
    let len = s.len();
    let hello = &s[0..5];
    let world = &s[6..len]; // let world = &s[6..] -> both are same
    println!("{hello}, {world}");

    let world = &s[6..];
    println!("{hello}, {world}");

    let s = String::from("hello");
    let len = s.len();
    let hello = &s[0..len];
    println!("{hello}");
    let hello = &s[..]; // same as above
    println!("{hello}");

    // ? Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

    let mut string: String = String::from("Hello, World");
    let word = first_word(&string);
    println!("First word: {word}");
    string.insert_str(0, "I am "); // * it reallocates btw, so yeah it will have runtime performance implications - just used for example purposes
    let word = first_word(&string);
    println!("New first word: {word}");

    let str = String::from("I am hello world");
    let word = second_word(&str);
    println!("Second word: '{word}'");

    // Now if we try to clear the string and access the first word, it'll be illegal/result in a compiler error
    let mut s = String::from("hello"); // `mut` used because we are gonna try and clear it later.
    let word: &str = first_word(&s); // ? Immutable reference of `s` is passed to first_word and then a 'immutable slice' of it is returned and attached to the `word` variable and considered alive from its creation until its last use.
    // s.clear(); // ! illegal: `word` is an active 'immutable slice' (&str) borrowing `s`.
    println!("{word}"); // this would become a dangling pointer if `s.clear()` were allowed
    // Mutating `s` would invalidate the slice, so Rust prevents it.

    // * Core idea above: You cannot mutate a value while immutable references to it are still in use.

    let s: &str = "Hello, World!"; // lives in the ROData section of the code (ROData = Read Only Data)
    // string literals are immutable, because they are literally embedded in the binary
    // '&str' is an immutable reference
    // * '&mut str' is possible, but that is tricky, because we'll need to make sure that they are properly UTF-8 encoded
    println!("The string literal is: {s}");

    // ? first_word_improved uses "&str" as param type hence extending it to string literals as well as to the String type as well.
    let my_string: String = String::from("hello, world!");
    let word = first_word_improved(&my_string);
    let word = first_word_improved(&my_string[0..my_string.len()]);
    let word = first_word_improved(&my_string[..]);

    // send partial parts
    let word = first_word_improved(&my_string[0..6]);

    let my_string_literal: &str = "hello, world!";
    let word = first_word_improved(&my_string_literal);

    // * Other slices
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let slice: &[u8] = &a[1..3]; // slice of the array 'a', This slice has the type &[u8]. It works the same way as string slices do, by storing a reference to the first element and a length.
    assert_eq!(slice, &[2, 3]);
}

pub fn find_first_words_index(s: &String) -> usize {
    for (idx, &el) in s.as_bytes().iter().enumerate() {
        if el == b' ' {
            return idx;
        }
    }
    s.len()
}

pub fn first_word(s: &String) -> &str {
    for (idx, &el) in s.as_bytes().iter().enumerate() {
        if el == b' ' {
            return &s[0..idx + 1];
        }
    }
    return &s[..];
}

pub fn second_word(s: &String) -> &str {
    // ?^^ Why immutable reference of `s`? Because we are changing nothing, just reading it.
    // "I am hello world" [first_space+1.. last_space]
    let mut first_space_idx: usize = 0;
    let mut counter: u8 = 1;
    for (idx, &el) in s.as_bytes().iter().enumerate() {
        if el == b' ' {
            if counter > 0 {
                first_space_idx = idx;
                counter -= 1;
            } else {
                return &s[first_space_idx + 1..idx];
            }
        }
    }
    &s[first_space_idx + 1..s.len()]
}

pub fn first_word_improved(s: &str) -> &str {
    for (idx, &el) in s.as_bytes().iter().enumerate() {
        if el == b' ' {
            return &s[0..idx];
        }
    }
    return &s[..];
}
