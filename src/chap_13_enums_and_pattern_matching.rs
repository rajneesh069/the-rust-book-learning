#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn start() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{four:?}"); //V4, type is IpAddrKind!

    println!("home:{home:?}, loopback: {loopback:?}"); // home:IpAddr { kind: V4, address: "127.0.0.1" }, loopback: IpAddr { kind: V6, address: "::1" }

    // NOTE: replacing struct with enums in this case is much better & concise
    #[derive(Debug)]
    enum IpAddrEnum {
        V4(String), // WARN: its a constructor and NOT a value now!
        V6(String),
    }

    let four = IpAddrEnum::V4(String::from("127.0.0.1"));
    let six = IpAddrEnum::V6(String::from("::1"));

    println!("four: {four:?}, six: {six:?}"); // four: V4("127.0.0.1"), six: V6("::1")

    #[derive(Debug)]
    enum IpAddrEnumWith4Params {
        V4(u8, u8, u8, u8), // doing this with pure structs will take a lot of fields and its not concise
        V6(String),
    }
    let four = IpAddrEnumWith4Params::V4(127, 0, 0, 1);
    let six = IpAddrEnumWith4Params::V6(String::from("::1"));
    println!("four: {four:?}, six: {six:?}"); // four: V4(127, 0, 0, 1), six: V6("::1")

    // So you can put any kind of data inside the `enums`
    #[derive(Debug)]
    enum Message {
        Quit,                       // Has no data associated with it at all
        Move { x: i32, y: i32 },    // Has named fields, like a struct does
        Write(String),              // Includes a single String
        ChangeColor(i32, i32, i32), // Includes three i32 values
    }

    // Above enum variants behave like different kinds of structs wrapped inside a single tagged union type!
    struct Quit; // unit struct - no data =  Constructor with no args!
    struct Move {
        x: i32,
        y: i32,
    } // normal struct syntax with named fields
    struct Write(String); // tuple struct
    struct ChangeColor(i32, i32, i32); // tuple struct = Constructor + Type named Color
    // INFO: Regarding ChangeColor:-
    // Conceptually ChangeColor is the following:
    // struct Color {
    //     0: i32,
    //     1: i32,
    //     2: i32,
    // }

    // plus an implicit constructor
    // fn ChangeColor(a: i32, b: i32, c: i32) -> ChangeColor {
    //     ChangeColor(a, b, c)
    // }

    // But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum defined, which is a single type.

    // INFO: we can also use "impl" keyword with enums just like structs!

    impl Message {
        fn call(&self) {
            println!("self: {self:?}");
        }
    }

    let m = Message::Write(String::from("Hello World"));
    m.call();

    // INFO: Structs vs Enums
    // Structs contains "fields" - which are accessible(using the dot operator).
    // - Single Shape, Exist Together and All the values exist all the time!
    struct User {
        name: String,
        age: u8,
        active: bool,
    }

    let user = User {
        name: String::from("Rajneesh"),
        age: 23,
        active: true,
    };
    println!("{}", user.age);

    impl User {
        fn create_active_user(name: &str, age: u8) -> Self {
            Self {
                name: String::from(name),
                age,
                active: true,
            }
        }
    }

    // Enums: An enum value is exactly one variant at a time. Enumeration literally means listing down things one by one
    // Could be only one of the many variants at a time
    // Each variant may have different data
    // Compiler forces strictness
    enum Direction {
        North, // North is one of the directions - one of the variants of a bigger abstraction called Direction!
        South,
        East,
        West,
    }

    // Namespace - Compile time - Types, Enum Variants, Functions, Modules, Constants
    // Values - Runtime - Variables, Struct "instances", Enum "instances", References

    // `::` -> means look inside it - like use std::any; -> look for "any" inside "std"
    // Usage: Modules(std::any), Types->associated items(String::from("Hello")), Enums->Variants(Direction::North), Inside an impl: [Self(Self::North), Self::create_active_user("Rajneesh", 23)]

    // '.' -> value based access, :: -> namespace based access(thats why self.North is illegal)

    // Summary:
    // Structs store data.
    // Enums represent choices.
    // "." works on values.
    // "::" works on names.

    // INFO: `NULL` in Rust is in the form of Option<T> enum.
    let some_num = Some(5);
    let some_char = Some('e');
    let absent_num: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(10);
    // ERROR: ILLEGAL - This prohibits you from making the billion dollar mistake! => let sum = x + y;
    // INFO:   ^^^  We're trying to add i8 + Option<i8>. Since Option<i8> and i8 are different types compiler will NOT allow you to add them and also Option<i8> could be None and compiler won't let you proceed unless you make sure that 'y' has a valid value! When a value is not of the type Option<T> its safe to assume that its always non-null.

    // INFO: In order to handle the cases of Option<T> enum, we need some construct which runs different code when its None and some other code when Some(T) is valid and could help us with extracting the value from inside the Some variant. This is where the `match` control flow construct comes in!!
}

fn route(ip_kind: IpAddrKind) {}
