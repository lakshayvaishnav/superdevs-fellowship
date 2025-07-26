use print_macro::print_input;

#[print_input]
fn hello(name: &str) {
    println!("Hello , {name}!");
}

fn main() {
    hello("Lakshay");
}
