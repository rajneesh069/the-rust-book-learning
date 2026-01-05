use std::collections::HashMap;

pub fn start() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); // all keys must have same type and all values must have same type, like in this case the type of keys is String and of values - its i32.

    // getting values from hashmaps
    let team_blue = String::from("Blue");
    let score = scores.get(&team_blue).copied().unwrap_or(0); // unwrap_or eagerly evaluates so if we want to evaluate the internal value lazily, i.e., only when needed then we use unwrap_or_else method
    println!("Blue team score: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Fav color");
    let field_value = String::from("I don't know!");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value have been moved and are unusable now
    // If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

    let mut scores = HashMap::new();
    // "or_insert" method works like this: if the key exists do nothing, if it doesn't then insert the key along with the provided value. It is a method defined on Entry and always provides a mutable reference to the value in the map (existing or newly inserted).
    scores.insert(String::from("Blue"), 10);

    let value: &mut i32 = scores.entry(String::from("Blue")).or_insert(50);
    println!("value: {value:?}"); // value: 10
    *value = 100; // mutable reference of the value is there so we can change it as well
    // INFO: `mut` on a binding means: the variable can be reassigned.`mut` in a reference type (`&mut T`) means: the "value" being pointed to can be mutated. That's why we don't need a `mut` with let value assignment. We will require `mut` if we tried to re-assign the "value" variable and NOT the "*value".

    scores.entry(String::from("Blue")).or_insert(100); // this will do nothing because Blue is already there

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}"); // {"hello": 1, "wonderful": 1, "world": 2}
}
