use std::sync::atomic::{AtomicI64, AtomicI32};

mod TcpServer;

fn main() {
    let server = TcpServer::<AtomicI32>::new("localhost",3000);
}
