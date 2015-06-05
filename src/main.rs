#![allow(dead_code)]
extern crate rsnl;
extern crate rsgnl;

fn main() {
    let mut nls = rsnl::socket::alloc().unwrap();

    let p = rsgnl::socket::connect(&mut nls);

    let s=  "nl80211";

    let family = rsgnl::controller::resolve(&nls, s);
    println!("Family Index: {}", family);

    // connect the socket to generic netlink
    
}
