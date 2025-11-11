type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| {
        println!("hi!");
    });

    // type Result<T>

    // never type
    // bar();
    print!("forever ");
    loop {
        println!("and ever ");
        break;
    }

    // dynamically sized types
    // str

    // let s1: str = "Hello there";
    // let s2: str = "How's it going?";
}

fn takes_long_type(f: Thunk) {

}

fn returns_long_type() -> Thunk {
    todo!("...")
}

// fn bar() -> ! {
    // never type
    // let guess = String::from("...");
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };
// }