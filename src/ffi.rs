use libc::{c_int, c_uint, ssize_t, kevent};

extern "C" {
    pub fn mysocket() -> c_int;
    pub fn serverconnect(sockfd: c_int, portno: c_int) -> c_int;
    pub fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    pub fn myaccept(sockfd: c_int) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn recv(sockfd: c_int, buf: *mut u8, len: c_int, flags: c_uint) -> ssize_t;
    pub fn my_errno() -> c_int;
    pub fn kqueue() -> c_int;
    pub fn my_ev_set(ev: &mut kevent, sockfd: c_int, tag: c_int);
    pub fn keventfn(kqfd: c_int, changelist: *const kevent, nchanges: c_int, eventlist: *mut kevent,
                nevents: c_int) -> c_int;
}
