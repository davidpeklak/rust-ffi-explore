extern crate ffi_explore;
extern crate libc;

use ffi_explore::*;

fn main() {
    let mut kq = KQueueDescr::new()
        .expect("Failed to create kqueue");

    let s = SocketDescr::new()
        .expect("Failed to create socket");
    s.server_connect(3128)
        .expect("Failed to connect to socket");
    s.listen(5)
        .expect("Failed to listen to socket");
    let ac = s.accept()
        .expect("Failed to accept connection");

    println!("Accepted connection");

    kq.register_acc_sock(&ac, Token(1))
        .expect("Failed to register event");

    println!("Registered event");

    match kq.fetch_event()
        .expect("Failed to fetch event") {
        Some(length) => println!("Fetched event. Lenght = {}", length),
        None => println!("Did not fetch event")
    }


    {

        let mut buf = [0u8; 16];

        loop {
            let length = ac.receive(&mut buf)
                .expect("Failed to receive on socket");

            if length == 0 {
                break
            }

            let s = std::str::from_utf8(&buf).unwrap();

            println!("Received: {}", s)
        }
    }
}
