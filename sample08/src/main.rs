mod mods;
mod common;

fn main() {
    let greet = "Hello!! たぢさん。";
    mods::my_module(greet);

    println!("{}",common::my_func(2,3));
}
