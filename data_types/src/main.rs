fn main() {

    // MARK: Numbers
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // MARK: Bool and Char
    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = '»';
    let grumpy_cat = '😾'; // <- :(

    // MARK: Compound
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {y}");

    let five_hundred = tup.0;

    // Array
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let b: [i32; 5];
    let first = a[0];
}
