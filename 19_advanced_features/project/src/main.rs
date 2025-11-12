use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// Attribute-like macros
// #[route(GET, "/")]
// fn index() {}

// #[proc_macro_attribute]
// pub fn route(_attr: TokenStream, _item: TokenStream) -> TokenStream {}

// Function-like macros
// let sql = sql!(SELECT * FROM posts WHERE id=1);

// #[proc_macro]
// pub fn sql(_input: TokenStream) -> TokenStream {}