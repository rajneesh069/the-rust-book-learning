enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    NewYork,
    California,
    Texas,
    Florida,
    Pennsylvania,
    Illinois,
    Ohio,
    Georgia,
    NorthCarolina,
    Michigan,
}

enum Coin_ {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn start() {
    // `match` is a control flow construct to compare a value against a series of `patterns` and then execute code based on which pattern matches.

    let coin = Coin::Nickel;
    println!("Value in cents: {}", value_in_cents(coin));

    let coin = Coin_::Quarter(UsState::California);
    println!("Value in cents: {}", value_in_cents_updgraded(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match six {
        None => println!("None"),
        Some(i) => println!("{i}"),
    }

    match none {
        None => {
            println!("None");
        }
        Some(i) => {
            println!("{i}");
        }
    }

    // INFO: Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.
}

fn value_in_cents(coin: Coin) -> u8 {
    // similar to "if", though the expression there has to evaluate to a Boolean but here it can be any type.
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        } // a "match" arm
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }

    //INFO: A useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.
}

fn value_in_cents_updgraded(coin: Coin_) -> u8 {
    match coin {
        Coin_::Penny => 1,
        Coin_::Nickel => 5,
        Coin_::Dime => 10,
        Coin_::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // matches are exhaustive, i.e., it must cover all possibilites!
    // match x {
    //     // ERROR: ILLEGAL
    //     Some(i) => Some(i + 1),
    // }
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn catch_all_pattern_demo(dice_roll: u8) -> u8 {
    // Using enums, we can also take special actions for a few particular values, but for all other values take one default action.
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) -> u8 {
        num_spaces
    }
    match dice_roll {
        3 => {
            add_fancy_hat();
            println!("You got a fancy hat!");
            0
        }
        7 => {
            remove_fancy_hat();
            println!("Your fancy hat is removed...");
            0
        }
        other => move_player(other), //INFO: Catches all the other possibilities through this arm hence making it exhaustive completely. Also this should be at last because if we put the "catch all" at the first place the other cases won't even run. If we didn't want to use the value in the catch_all arm, then we can use an underscore(_) instead of a variable name. Example: _ => reroll(). If you don't wanna do anything when the user rolls anything other than 3 and 7 then you can write it like this: _ => () [usage of unit type - empty value]
    }
}
