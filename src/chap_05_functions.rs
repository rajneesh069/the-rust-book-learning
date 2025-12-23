pub fn functions() {
    println!("This is a function");
}

pub fn print_labeled_measurement(x: i32, unit_label: &str) {
    println!("The value is: {x}{unit_label}");
}

pub fn statements_and_expressions() {
    // STATEMENTS:
    // - Do NOT evaluate to a value
    // - Usually end with `;`
    // - Cannot be assigned

    let a = 6; // `let` bindings are statements
    println!("{a}"); // expression `println!(...)` + `;` => statement

    // let x = (let y = 10); // invalid: statements produce no value

    // EXPRESSIONS:
    // - Evaluate to a value
    // - Can be assigned
    // - Semicolon turns an expression into a statement

    let c = {
        let x = 3; // statement inside a block
        x + 1 // last expression => block value
    };

    // If you add `;` here, the block evaluates to `()`
    // let c = { let x = 3; x + 1; }; // c = ()

    // FUNCTION CALLS:
    // - Are expressions
    // - Become statements when followed by `;`

    foo(); // expression statement
}

// ITEMS:
// - Define things
// - Are NOT expressions
// - Do NOT evaluate to values

fn foo() {}

// let → statement

// expr → value

// expr; → statement

// { ... } → block expression

// fn / struct / enum → items

// Rust is expression-oriented, not expression-only

fn five() -> i32 {
    5 // tail expression
}

fn plus_one(x: i32) {
    // doesn't return i32 -> it actually returns `()` unit type as we made x+1 into a statement!
    x + 1;
}
// that's why below's function definition produces an error
// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }
