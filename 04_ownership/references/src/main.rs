// Either one mutable reference or any number of immutable references
// References must always be valid

fn main() {
    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{s1}' is {len}...");

    // let mut s = String::from("hello");

    // change(&mut s);

    // {
    //     let r1 = &mut s;
    // }

    // let r2 = &mut s;

    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     println!("{}", s);
//     s.len()
// }


