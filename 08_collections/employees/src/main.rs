// Problem
// Use vectors and hashmaps
// Text interface to add employee names: 'Add Sally to Engineering',
// 'Add Amir to Sales'. Let user retrieve a list of people a list of
// all people in a department or all people in a company,
// sorted alphabetically.

// get all
// get from Engineering

use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<String, String> = HashMap::new();

    loop {
    
        let reader = std::io::stdin();
        let mut buf = String::new();
        let quits = ["q", "quit"];
        let add_command = "add";
        let get_command = "get";
        let all_subcommand = "all";
        let from_subcommand = "from";

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

        if !words.is_empty() {
            let command = words[0].trim().to_lowercase();
                
            if command == add_command {
                if words.len() == 4 {
                    let name = words[1].trim();
                    let department = words[3].trim();
                    println!("Adding {name} employee to {department}...");
                    hm.insert(String::from(name), String::from(department));
                }
            } else if command == get_command {
                if words.len() == 2 {
                    let subcommand = words[1].trim();
                    if subcommand == all_subcommand {
                        // print all
                        println!("printing all");
                        for (k, v) in &hm {
                            println!("{} {}", k, v);
                        }
                    }
                } else if words.len() == 3 {
                    let subcommand = words[1].trim();
                    let dep = words[2].trim();
                    if subcommand == from_subcommand {
                        // print some
                        println!("printing some");
                        for (k, v) in &hm {
                            if v == dep {
                            println!("{}", k);
                            }
                        }
                    }
                }
            }
        }
    
    }
}
