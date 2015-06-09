extern crate libc;
extern crate rsnl;

use libc::{c_int, c_void};
use rsnl::message::{NetlinkMessage, nl_msg, nlmsghdr};
use rsnl::message::expose::{nl_msg_ptr, nlmsghdr_ptr};

#[repr(C)]
struct genlmsghdr;

#[link(name="nl-genl-3")]
extern "C" {
    fn genlmsg_valid_hdr(hdr: *const nlmsghdr, hdrlen: c_int) -> i32;
    fn genlmsg_hdr(hdr: *const nlmsghdr) -> *const genlmsghdr;
    fn genlmsg_put(msg: *const nl_msg, port: u32, seq: u32, family: c_int, hdrlen: c_int, flags: c_int, cmd: u8, version: u8) -> *const c_void;
}

// leaky!
pub struct GenlHeader {
    ptr: *const genlmsghdr,
}

pub fn valid_hdr(msg: &NetlinkMessage, hdrlen: i32) -> i32 {
    let hdrptr = nlmsghdr_ptr(msg);

    unsafe { genlmsg_valid_hdr(hdrptr, hdrlen) }
}

pub fn hdr(msg: &NetlinkMessage) -> GenlHeader {
    GenlHeader {
        ptr: unsafe { genlmsg_hdr(nlmsghdr_ptr(msg)) }
    }
}

pub fn put(msg: &mut NetlinkMessage, port: u32, seq: u32, family: i32, hdrlen: i32, flags: i32, cmd: u8, version: u8) {
    unsafe { genlmsg_put(nl_msg_ptr(msg), port, seq, family, hdrlen, flags, cmd, version); }
}
