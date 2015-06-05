extern crate rsnl;
use rsnl::socket::{NetlinkSocket, nl_sock};
//#[repr(C)]

#[link(name="gennl-3")]
extern "C" {
	// Exposed socket functions
	fn genl_connect(socket: *const nl_sock) -> i32;
}

fn connect(sock: &mut NetlinkSocket) -> i32 {

    unsafe { genl_connect(rsnl::socket::expose::nl_sock_ptr(sock)) }
    
}
