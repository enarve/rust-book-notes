struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Couperin")
    };
    let d = CustomSmartPointer {
        data: String::from("Debussy")
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("End of main.")
}
