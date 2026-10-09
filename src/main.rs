use std::io;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use tcp::Connection;

mod tcp;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, Connection> = Default::default();
    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1500];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0],buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2],buf[3]]);
        // Catch only IPv4
        if eth_proto != 0x0800 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]){
            Ok(ip_header) => {
                let src = ip_header.source_addr();
                let dst = ip_header.destination_addr();
                // Catch only TCP packets
                if ip_header.protocol() != 0x06 {
                    continue;
                }
                let ip_header_size = ip_header.slice().len();
                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + ip_header.slice().len()..nbytes]){
                    Ok(tcp_header) => {
                        let data = 4 + ip_header_size + tcp_header.slice().len();
                       connections.entry(Quad {
                            src: (src, tcp_header.source_port()),
                            dst: (dst, tcp_header.destination_port()),
                        })
                           .or_default()
                           .on_packet(&mut nic, ip_header, tcp_header, &buf[data..nbytes]);
                        /*eprintln!("{} → {} {} bytes of TCP packet to port {}",
                                  src,
                                  dst,
                                  tcp_header.slice().len(),
                                  tcp_header.destination_port()
                        );*/
                    }
                    Err(e) => {
                        eprintln!("Ignoring invalid packet {:?}",e);
                    }

                }
            }
            Err(e) => {
                eprintln!("Ignoring invalid packet {:?}",e);
            }
        }
    }
}
