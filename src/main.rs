pub mod hello_world; 

fn main() {
    first_chapter();
}


fn first_chapter() {
    println!("============================================================================");
    println!("========================= 1. Hello World ===================================");
    println!("============================================================================");
    hello_world::hello_rustaceans();
    println!("****1.1. Formatted print******");
    hello_world::formatted_print();
}