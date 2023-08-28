use std::sync::{atomic::AtomicU64, Arc};

use tcp_server::TcpServer;
//extern crate TcpServer;
mod tcp_server;

type AtomicInteger = Arc<AtomicU64>;

fn main() {
    let server = TcpServer::<AtomicInteger>::new("127.0.0.1",3000);
}
