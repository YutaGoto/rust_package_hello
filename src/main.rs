extern crate rust_package_hello;
use rust_package_hello::plus;

fn main() {
    let x = 1;
    println!("{}", plus::plus_one(x));
    println!("{}", plus::plus_n(x, 3));
}
