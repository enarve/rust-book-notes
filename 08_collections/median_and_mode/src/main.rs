use std::collections::HashMap;

fn main() {
    let mut integers = vec![6, 4, 3, 2, 5, 5, 1];
    // sort list
    integers.sort();
    
    // create hashmap for getting mode
    let mut map = HashMap::new();

    for i in &integers {
        println!("{i}");
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let middle_position = integers.len() / 2;
    let median = integers[middle_position];
    println!("Median: {}", median);

    let mut mode: i32 = 0;
    let mut max_key: &i32;
    let mut flag: bool = false;
    for (k, v) in map.iter() {
        if !flag {
            mode = *v;
            max_key = k;
        } else {
            if *v > mode {
                max_key = k;
                mode = *v;
            }
        }
    }

    println!("Mode: {}", mode);

}
