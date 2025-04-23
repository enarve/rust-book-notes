// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
    
}