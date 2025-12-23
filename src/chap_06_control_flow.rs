use rand::Rng;
pub fn control_flow_with_statements_and_expressions_magic() -> i32 {
    // if `expressions` -> focus on the word expressions here - it has a meaning!
    if rand::thread_rng().gen_range(0..10) < 5 {
        10
    } else {
        100
    }
}

// pub fn control_flow_with_statements_and_expressions_magic() {
//     // if 'expressions' -> focus on the word expressions here - it has a meaning!
//     // now this returns a unit type, i.e., nothing!
//     if rand::thread_rng().gen_range(0..10) < 5 {
//         10;
//     } else {
//         100;
//     };
// }
pub fn control_flow() {
    // if expressions
    let number = 3;
    if number > 5 {
        println!("The condn. was true.");
    } else {
        println!("The condn. was false.");
    }

    let number = 10;
    if number % 2 == 0 {
        println!("The number is divisible by 2.");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3.");
    } else if number % 5 == 0 {
        println!("The number is divisible by 5.");
    } else if number % 7 == 0 {
        println!("The number is divisible by 7.");
    }

    // using `if` and `let` together
    let number = if number > 4 { 5 } else { 10 };
    println!("The value of number is: {number}") // 5
    // - Blocks of code evaluate to the "last expression" in them and number by themselves are also expressions, because they obviously evaluate to a resultant value!
}

pub fn control_flow_with_loops() {
    // Infinite loop
    // loop {
    //     println!("again!")
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {result}");
    // loops with labels
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        'remaining_down: loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break 'remaining_down; // or break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count: {count}");

    // while loop
    let mut counter = 3;
    while counter != 0 {
        println!("counter: {counter}");
        counter -= 1;
    }

    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < arr.len() {
        println!("index-{index}: {}", arr[index]);
        index += 1;
    }
    // for loop to iterate over the elements of the array
    for element in arr {
        println!("element: {element}");
    }
    // using `Range`
    for number in (1..4).rev() {
        // (1..4) = 1, 2, 3 => (1..4).rev() = 3, 2, 1
        println!("{number}")
    }
}
