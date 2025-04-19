use std::io;

fn main() {
    let mut input = String::new();
    let mut number: i32;
    
    println!("Enter 'n' to get nth Fibonacci number.");

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Error reading line.");

        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let fib = fib(number);
        println!("{fib}");
        println!();
    }
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        return fib(n-1) + fib(n-2);
    }
}
