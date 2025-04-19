use std::io;

fn main() {
    let mut temp: i32;
    let mut mode: String = "f".to_string();
    let mut input: String;

    println!("Enter temperature to convert. To convert from fahrenheit enter 'f'. To convert from celsium enter 'c'. To quit enter 'q'.");

    loop {
        input = "".to_string();

        // get user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let text: String = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => "".to_string(),
        };

        if text == "f" || text == "c" {
            mode = text;
        } else if text == "q" {
            break
        }

        temp = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // convert
        temp = if mode == "f" { (temp - 32) * 5/9 } else { (9/5 * temp) + 32 };

        // print
        println!("{} {}", temp, if mode == "f" {"c"} else {"f"});
        println!();

    }
}
