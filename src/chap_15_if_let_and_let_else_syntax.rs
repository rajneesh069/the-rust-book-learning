#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

impl UsState {
    //INFO: If a function inside impl uses self, it’s a method → self must be first. If it doesn’t use self, it’s just an associated function.
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::California => year >= 1850,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,            // 1
    Nickel,           // 5
    Dime,             // 10
    Quarter(UsState), // 25
}

pub fn start() {
    //INFO: if let and let else use pattern matching, which is different from normal if/else expressions that evaluate boolean conditions.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum value is configured to be {max}"),
        _ => (), // annoying repition/boilerplate if we don't wanna execute if the value is None
    }

    // Concise way to write the same thing using "if let" syntax
    if let Some(max) = config_max {
        //INFO: The syntax if let takes a "pattern" and an expression separated by an equal sign. It works the same way as a match, where the expression is given to the match and the pattern is its first arm. In this case, the pattern is Some(max), and the max binds to the value inside the Some. We can then use max in the body of the if let block in the same way we used max in the corresponding match arm. The code in the if let block only runs if the value matches the pattern. You lose the exhaustiveness of match pattern matching though.
        println!("The maximum value is configured to be {max}");
    }

    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;
    // I want to run code just for the "North" dirn. and ignore the rest.
    if let Direction::North = dir {
        println!("The direction is North.");
    }

    // Using `else` with `if let`

    let dir = Direction::South;
    if let Direction::North = dir {
        println!("The direction is North.");
    } else {
        // this is equivalent to the `_` case of the match syntax, basically a catch-all.
        println!("The direction is {dir:?}.");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::California);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}.");
    } else {
        count += 1;
    }

    println!("The count is: {count}");

    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = &coin {
        // INFO: We pattern-match on `&coin` to avoid moving it. If we matched on `coin` directly, extracting data from a variant containing non-Copy values (like `Quarter(state)`) would move that data and potentially leave `coin` in a partially moved state. By matching on a reference (`&coin`), the pattern only borrows the inner data, so `coin` remains fully usable after the `if let`.
        println!("State quarter from {state:?}");
    } else {
        count += 1;
    }
    println!("The count is: {count}");

    // INFO: `Quarter(state)` is a pattern that destructures the `Quarter` enum variant and binds its inner value to `state`.

    // WARN: Whether this binding moves or borrows the inner value depends on what is being matched on.

    // 1) Matching on an owned value → MOVE (partial move):
    //    if let Coin::Quarter(state) = coin {
    //        // `state: State` (moved)
    //    }
    //    // `coin` is now partially moved and cannot be used as a whole.
    //
    // 2) Matching on a reference → BORROW (no move):
    //
    //    if let Coin::Quarter(state) = &coin {
    //        // `state: &State` (borrowed)
    //    }
    //    // `coin` remains fully usable.
    //
    // WARN:
    // - `if let` itself does NOT cause moves.
    // - Moves happen only when pattern matching on owned values.
    // - Matching on references borrows the inner data instead of moving it.

    // INFO: `match` follows the same ownership rules as `if let` and `let ... else`.
    // WARN: Whether a move occurs depends on what is being matched on:
    //
    // 1) Matching on an owned value (`coin`) moves inner non-Copy data:
    //
    //    match coin {
    //        Coin::Quarter(state) => {
    //            // `state: State` (moved)
    //        }
    //        _ => {}
    //    }
    //    // `coin` is partially moved and cannot be used as a whole.
    //
    // 2) Matching on a reference (`&coin`) borrows the inner data:
    //
    //    match &coin {
    //        Coin::Quarter(state) => {
    //            // `state: &State` (borrowed)
    //        }
    //        _ => {}
    //    }
    //    // `coin` remains fully usable.
    //
    //INFO: Ownership behavior comes from pattern matching on owned values vs references, not from `match` itself.

    let str = describe_quarter_state_1(&coin);
    println!("Case 1: {str:?}");

    let coin = Coin::Quarter(UsState::Alaska);
    let str = describe_quarter_state_2(&coin);
    println!("Case 2: {str:?}");

    let coin = Coin::Quarter(UsState::California);
    let str = describe_quarter_state_3(&coin);
    println!("Case 3: {str:?}");
}

fn describe_quarter_state_1(coin: &Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None // eventually we return None at last if the Coin isn't a Quarter and this work can be done early in the function as well in another way
    }
}
fn describe_quarter_state_2(coin: &Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None; // Notice the "return" keyword, it will return from the function itself, not just merely evaluate to a value
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }

    // The above seems tedious as well.
}

fn describe_quarter_state_3(coin: &Coin) -> Option<String> {
    // Using the `let ... else` syntax.

    // INFO: The `let ... else` syntax takes a pattern on the left-hand side and an expression on the right-hand side, similar to `if let`, but it does not have an `if` branch — only an `else` branch. If the pattern matches, the bindings introduced by the pattern are available in the outer scope. If the pattern does not match, control flow enters the `else` block, which must be a diverging expression (`!`), such as `return`, `break`, `continue`, or `panic!`.

    // INFO: As a result, execution cannot continue past the `else` block in `let ... else`. In contrast, with `if let`, control flow "can" continue after the conditional.

    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
