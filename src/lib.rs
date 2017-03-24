extern crate libc;


mod ffi;
mod socket_descr;
mod kqueue_descr;

pub use socket_descr::{New, Listen, Accept, SocketDescr};
pub use kqueue_descr::{KQueueDescr, Token};
pub use ffi::keventfn;
