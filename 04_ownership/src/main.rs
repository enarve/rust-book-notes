// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    
    let mut s = String::from("hello");
    s.push_str(", world.");
    println!("{s}");

    // <- can be mutaded, stored in heap

    // string literal -> contents known in compile time

    // string type is mutable, so we need memory handling
    // memory problems start with the values stored in the heap, aka mutable values
    // memory is allocated during the initialization

    {
        let _s1 = String::from("hello");
        // memory is freed when the scope of the variable ends
    }

    // _example_4();
    _example_5();
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

fn _example_4() {
    // Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn _example_5() {
    // Stack-only data: copy
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    // Types that implement Copy Trait and types that not...
}

fn _example_6() {
    let s = String::from("hello");
    takes_ownership(s);
    // s's value moves into the function...
    // ... and so is no longer valid here

    // x comes into scope
    let x = 5;

    // because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
}
// Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}
// Here, some_integer goes out of scope. Nothing special happens.

fn _example_7() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// References...