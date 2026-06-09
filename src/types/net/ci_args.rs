use std::net::Ipv4Addr;

const PORT: u16 = 8080;

#[derive(Debug)]
pub struct CiArgs {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl CiArgs {
    pub fn new() -> Self {
        CiArgs {
            ip: Ipv4Addr::new(127, 0, 0, 1),
            port: PORT,
        }
    }
}
