#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn width(self: &Self) -> bool {
        // INFO: &self is shorthand for self: &Self
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn print(&self) {
        println!("Width: {}, Height: {}", self.width, self.height);
    }
}

pub fn methods() {
    let rect = Rectangle {
        width: 21,
        height: 12,
    };

    let a = rect.area();
    println!("The value of a is: {a}");

    let is_width_greater_than_0 = rect.width(); // INFO: `width` property and the method name are the same but Rust figures it out based on the parentheses.
    println!("The rectangle's width is greater than 0? : {is_width_greater_than_0}");

    /* INFO :
     * In C and C++, two different operators are used for calling methods: You use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something().
     * Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust with this behavior.
     * Here’s how it works: When you call a method with object.something(), Rust automatically adds in &, &mut, or * so that object matches the signature of the method. In other words, the following are the same:
     *  p1.distance(&p2);
     * (&p1).distance(&p2);
     * The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
     *  We know we want to define a method, so it will be within the impl Rectangle block. The method name will be "can_hold", and it will take an immutable borrow of another Rectangle as a parameter. We can tell what the type of the parameter will be by looking at the code that calls the method: rect1.can_hold(&rect2) passes in &rect2, which is an immutable borrow to rect2, an instance of Rectangle. This makes sense because we only need to read rect2 (rather than write, which would mean we’d need a mutable borrow), and we want main to retain ownership of rect2 so that we can use it again after calling the can_hold method. The return value of can_hold will be a Boolean, and the implementation will check whether the width and height of self are greater than the width and height of the other Rectangle, respectively. */

    let rect1 = Rectangle {
        width: 32,
        height: 23,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 15,
    };

    let rect3 = Rectangle {
        width: 33,
        height: 24,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(22);
    println!("square: {square:#?}");

    // Multiple `impl` blocks are allowed
    let rect = Rectangle {
        width: 100,
        height: 200,
    };
    rect.print();
}
