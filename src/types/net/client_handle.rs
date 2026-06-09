use std::net::SocketAddr;

#[allow(unused)]
pub struct ClientHandle {
    pub id: u64,
    name: String,
    sock_addr: SocketAddr,
}

impl ClientHandle {
    pub fn from(id: u64, name: String, sock_addr: SocketAddr) -> Self {
        Self { id, name, sock_addr }
    }
}
