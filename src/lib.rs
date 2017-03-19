extern crate libc;


mod ffi;
mod socket_descr;
mod acc_socket_descr;
mod kqueue_descr;

pub use socket_descr::SocketDescr;
pub use acc_socket_descr::AccSocketDescr;
pub use kqueue_descr::KQueueDescr;
pub use kqueue_descr::KEventParam;
pub use ffi::keventfn;