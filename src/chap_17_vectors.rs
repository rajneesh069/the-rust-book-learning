pub fn start() {
    // Ways to define a vector
    let v: Vec<i32> = Vec::new();
    let v: Vec<u8> = Vec::from([1, 2, 3]);
    // v.push(23); // ILLEGAL, because `v` is immutable!
    let v = vec![1, 2, 3];

    let mut v = Vec::new(); // type inferred from usage
    v.push(32);
    v.push(23);
    v.push(22);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is: {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("There's no third element."),
    }

    let el_100 = v.get(100); // This will give us the `None` arm's message/execute the code of that arm. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
    match el_100 {
        Some(el_100) => println!("The 100th element is: {el_100}"),
        None => println!("There's no 100th element."),
    }
    // let el_100 = &v[100]; // this will crash the program because there's no 100th element, it is good when you want to crash the application if out of bounds access is requested.
    // println!("The 100th element is: {el_100}");

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let first = &v[0]; // held an "immutable reference" of the first element
    // v.push(7); // ILLEGAL, because suppose now the vector needs to be allocated somewhere else then "first" will be pointing to a deallocated location!
    // v[0] = 100; // ILLEGAL, because of ownership and borrowing rules, since the immutable reference is being used below!!
    println!("The first element is: {first}");
    println!("0th el: {}", v[0]); // All good to access

    // for i in v {
    //     // without the ampersand(&) the values inside the vector will be moved to "i" in each iteration and will be lost later, i.e., consumed
    //     println!("{i}");
    // }
    // println!("0th el: {}", v[0]); // ILLEGAL: borrow of a moved value

    for i in &v {
        // that's why we take the reference
        println!("{i}");
    }
    let v = vec![10, 11, 12, 13, 14, 15];
    for (idx, el) in v.iter().enumerate() {
        println!("{idx}: {el}")
    }

    let mut v = vec![23, 25, 57, 10, 99];

    for i in &mut v {
        *i += 1; // '*' is the dereference operator.

        // v.push(100); // ILLEGAL: would create overlapping mutable borrows, because v.push(100) is same as (&mut v).push(100); Its just that Rust auto dereferences it.

        // `for i in &mut v` creates ONE mutable borrow of the entire vector `v`
        // that lasts for the "entire duration of the loop".
        //
        // Each `i` is a `&mut T` derived from that borrow, allowing us to safely
        // modify existing elements.
        //
        // However, because `v` is already mutably borrowed, we CANNOT call methods
        // like `v.push()` here. Doing so would require another `&mut v` while the
        // first mutable borrow is still active, which Rust forbids.
        //
        // This restriction prevents structural changes (like reallocation) that
        // could invalidate existing references and cause undefined behavior.
        //
    }

    // Using an Enum to store different kinds of data
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(1000),
        SpreadSheetCell::Float(3.14159),
        SpreadSheetCell::Text(String::from("Rajneesh Mishra")),
        // The `String` is OWNED by the `SpreadSheetCell::Text` enum variant.
        // That enum value is owned by the `Vec<SpreadSheetCell>`, which is owned by `row`.
        // In other words, `row` owns the vector, the vector owns its elements, and the `Text` variant owns the `String` (and thus the heap data).
    ];

    {
        let v = vec![1, 2, 3, 4];
        println!("{v:?}"); // do stuff with `v`
        // beyond this scope the memory is "dropped" and all the integers/elements it holds get cleaned up.
    }
}
