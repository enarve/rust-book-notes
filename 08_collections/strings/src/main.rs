fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    let s1 = String::from("hello");
    // let h = s1[0];

    let hello = String::from("Hola");
    let hello = String::from("Здравствуйте");
    // let answer = &hello[0];

    let hello = "Здравствуйте";
    let s = &hello[0..4];

    for c in "Зд".chars() {
        println!("{c}");
    }
}
