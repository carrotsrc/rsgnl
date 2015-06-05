#![allow(dead_code)]
extern crate rsnl;
extern crate rsgnl;

fn main() {
    let sock = rsnl::socket::alloc();

    let p = rsgnl::socket::connect(&mut sock.unwrap());

    println!("{}", p);
}
