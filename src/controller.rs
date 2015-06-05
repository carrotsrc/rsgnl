extern crate libc;
extern crate rsnl;

use libc::{c_int};
use rsnl::socket::{NetlinkSocket, nl_sock};
use rsnl::socket::expose::{nl_sock_ptr};


#[link(name="nl-genl-3")]
extern "C" {
    fn genl_ctrl_resolve(socket: *const nl_sock, name: *const str) -> i32;
}


pub fn resolve(sock: &NetlinkSocket, name: *const str) -> i32 {
    unsafe { genl_ctrl_resolve(nl_sock_ptr(sock), name) }
}
