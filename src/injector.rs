use socket2::{Domain, Protocol, SockAddr, Socket, Type}; 
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4};

struct Injector {
    socket: Socket,   
}

impl Injector {
    fn new() -> io::Result<Self> {
        let socket = Socket::new(
            Domain::IPV4,
            Type::RAW,
            Some(Protocol::TCP),
            )?;

        socket.set_header_included(true)?;

        Ok(Self {socket})
    }

    fn shoot(&self, target_ip: Ipv4Addr, raw_packet: &[u8]) -> io::Result<usize> {
        let dest_addr = SocketAddrV4::new(target_ip, 0);
        let addr = SockAddr::from(dest_addr);

        self.socket.send_to(raw_packet, &addr)
    }
}
