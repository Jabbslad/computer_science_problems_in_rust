mod fib;
mod compression;
mod encryption;
mod dnasearch;

use fib::fib;

fn main() {
    println!("{}", fib(1));
}
