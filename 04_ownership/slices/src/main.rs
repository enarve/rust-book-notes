fn main() {
    // let s = String::from("hello, world!");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    let my_string = String::from("hello world");

    // first_word_3 works on slices of String whether partial or whole
    let word = first_word_3(&my_string[0..6]);
    let word = first_word_3(&my_string[..]);
    // also works on references to String
    // which are same as whole slices of String
    let word = first_word_3(&my_string);

    let my_string_literal = "hello world";

    // first word works on slices of string literals
    // parial or whole
    let word = first_word_3(&my_string_literal[0..6]);
    let word = first_word_3(&my_string_literal[..]);

    let word = first_word_3(my_string_literal);

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    s.len()
}