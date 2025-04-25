// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    
    let mut s = String::from("literal");
    s.push_str(", bar.");
    println!("{s}");

    // <- can be mutaded, stored in heap, probably

    // string literal -> contents known in compile time

    // string type is mutable, so we need memory handling
    // memory problems start with the values stored in the heap, aka mutable values
    // memory is allocated during the initialization

    {
        let _s1 = String::from("hello");
        // memory is freed when the scope of the variable ends
    }
}

fn _example_1() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn _example_2() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // what is the difference between len and capacity

    // pointer, len, capacity

    // if we assign something to the existing value
    // like this,
    // x = y;
    // then there are several possibilities
    // it could work like reference type, like classes, when the variables point to one memory simultaneously, and you can access both of the variables

    // it could work as a value type, when memory is copied

    // instead it does something else

    // s1 is moved to s2

    let s3 = String::from("invalidation");
}

fn _example_3() {
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    // something on scope and assignment
    
}