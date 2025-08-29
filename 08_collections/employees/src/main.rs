// Problem
// Use vectors and hashmaps
// Text interface to add employee names: 'Add Sally to Engineering',
// 'Add Amir to Sales'. Let user retrieve a list of people a list of
// all people in a department or all people in a company,
// sorted alphabetically.

use std::collections::HashMap;

fn main() {
    let hm: HashMap<String, String> = HashMap::new();

    loop {
    
        let reader = std::io::stdin();
        let mut buf = String::new();
        let quits = ["q", "quit"];

        match reader.read_line(&mut buf) {
            Ok(_) => (),
            Err(_) => ()
        }

        // break from the loop of specific command
        if quits.contains(&buf.trim()) {
            break;
        }
        
        // Parse else and decide what to do
        let words: Vec<&str> = buf.split(" ").collect();
        if words.len() == 4 {
            // Either add to hashmap or print some results
            println!("To do...");
        } else {
            println!("Wrong command.");
        }
    
    }
}
