extern crate libc;
extern crate rsnl;

use libc::{c_int};
use rsnl::socket::{NetlinkSocket, nl_sock};
use rsnl::socket::expose::{nl_sock_ptr};


#[link(name="nl-genl-3")]
extern "C" {
	// Exposed socket functions
	fn genl_connect(socket: *const nl_sock) -> i32;
	fn genl_send_simple(socket: *const nl_sock, family: c_int, cmd: c_int, version: c_int, flags: c_int ) -> i32;
}

pub fn connect(sock: &mut NetlinkSocket) -> i32 {
    unsafe { genl_connect(nl_sock_ptr(sock)) }
}

pub fn send_simple(sock: &NetlinkSocket, family: i32, cmd: i32, version: i32, flags: i32) -> i32 {
    unsafe { genl_send_simple(nl_sock_ptr(sock), family, cmd, version, flags) }
}

