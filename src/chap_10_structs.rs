struct User {
    active: bool,
    name: String,
    age: u8,
    address: String,
    email: String,
    username: String,
    password: String,
    sign_in_count: u64,
}

pub fn start() {
    // In tuples, we can store multiple types of data but the problem there is you'll have to rely on the order of the data.

    // In structs we have named/annotated fields which give us more accessibility and flexibility over tuples.

    let mut user = User {
        active: true,
        name: String::from("Rajneesh Mishra"),
        age: 23,
        address: String::from("Gayatri Puram Colony, Civil Lines, Gonda, 271001"),
        email: String::from("rajneesh.mishra9616@gmail.com"),
        password: String::from("rajneesh123"),
        username: String::from("rajneeshmishra6969"),
        sign_in_count: 10,
    };

    user.email = String::from("rajneesh.mishra9616@outlook.com"); // to do this, the entire instance of user should be mutable

    let user2 = create_user(
        String::from("rajneesh.mishra9616@gmail.com"),
        user.name, // passed the name from `user` variable -> this value has been moved!
        String::from("rajneesh123"),
        String::from("rajneesh6969"),
        23,
        String::from("Gayatri Puram Colony"),
    );

    println!("{:?}", user.email); // * Okay
    // println!("{:?}", user.name); // ! ILLEGAL
    let user3 = User {
        email: String::from("rajneesh.mishra9616@outlook.com"),
        ..user2 // struct update syntax, this updates the `email` field but uses all the other fields from `user2` -> user2 has been moved!!!
    };

    // println!("{:?}", user2); // ! ILLEGAL
    // Because `String` does not implement `Copy` trait, hence value is moved instead of being copied like integers, floats, etc...

    // If we only use the update syntax for the values that implement the `Copy` trait then it won't be moved, following is the example of it.

    struct MiniUser {
        active: bool,
        email: String,
        username: String,
        sign_in_count: u32,
    }

    let u1 = MiniUser {
        active: true,
        email: String::from("rajneesh.mishra9616@gmail.com"),
        username: String::from("rajneesh6969"),
        sign_in_count: 12,
    };

    let u2 = MiniUser {
        email: String::from("rajneesh.mishra9616@outlook.com"),
        username: String::from("sachin6969"),
        ..u1 // ? u1 NOT moved, because all the values which will use the update syntax implement the `Copy` trait.
    };

    println!(
        "{}, {}, {}, {}",
        u1.email, u1.active, u1.username, u1.sign_in_count
    ); // * Perfectly fine

    // Basically, whichever data implements the Copy trait, they are copied or else they are moved. Generally, "Stack only" data is copied.

    // Tuple Structs
    struct Color(i32, i32, i32); // rgb
    struct Point(i32, i32, i32); // (x,y,z) co-ordinates
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Even though Point and Color look the same, but if you pass them as a parameter, a Point param won't accept Color and vice versa.

    println!("{}, {}, {}", origin.0, origin.1, origin.2);
    let Color(r, g, b) = black; // destructuring of Tuple structs, it requires explicit naming of the Tuple Struct.
    println!("r = {r}, g = {g}, b = {b}");

    // Unit Structs
    struct AlwaysEqual;
    let subject = AlwaysEqual; // Unit struct, no fields, but traits can be implemented on it.

    // struct L {
    //     active: bool,
    //     email: &str, // ! ILLEGAL, lifetime parameter is required for it, we'll study it later, for now we'll go for String
    // }
}

fn create_user(
    email: String,
    name: String,
    password: String,
    username: String,
    age: u8,
    address: String,
) -> User {
    User {
        active: true,
        name, // using field init shorthand
        age,
        address,
        email,
        username,
        password,
        sign_in_count: 0,
    }
}
