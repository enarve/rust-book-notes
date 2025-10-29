fn main() {
    // match arms
    let x: Option<i32> = Some(0);

    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // if let
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using your favourite, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // for
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // let
    let (x, y, z) = (1, 2, 3);

    // function parameters
    fn foo(x: i32) {
        // code goes here
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: {x}, {y}");
    }

    let point = (3, 5);
    print_coordinates(&point);
    
}
