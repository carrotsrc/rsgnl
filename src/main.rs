#![allow(dead_code)]
extern crate rsnl;
extern crate rsgnl;

fn main() {
    let mut sock = rsnl::socket::alloc();
    let p = rsgnl::socket::connect(&mut sock);

    println!("{}", p);
}
