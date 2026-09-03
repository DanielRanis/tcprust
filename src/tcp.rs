pub enum State {
    Closed,
    Listen,
    SynRecv,
    Estab,
}

impl Default for State{
    fn default() -> Self{
        //State::Closed
        State::Listen
    }
}

impl State{
    pub fn on_packet<'a>(&mut self, ip_header: etherparse::Ipv4HeaderSlice<'a>, tcp_header: etherparse::TcpHeaderSlice<'a> , data: &'a[u8]){
        #[cfg(any())] // ToDo: Will implement all TCP states
        {
            match *self {
                State::Closed => {
                    return;
                }
                State::Listen => {
                    if !tcp_header.syn() {
                        // Only expected SYN packet
                        return;
                    }
                    // Establish a connection
                    let syn_ack = etherparse::TcpHeader::new(tcp_header.destination_port(), tcp_header.source_port(), 0, 0);
                    //let syn_ack.syn = true;
                    //let syn_ack.ack = true;
                    let mut ip = etherparse::Ipv4Header::new(
                        syn_ack.slice().len(),
                        64,
                        etherparse::IpTrafficClass::Tcp,
                        ip_header.destination_addr(),
                        ip_header.source_addr()
                    );
                }
                State::SynRecv => {}
                State::Estab => {}
            }
            eprintln!("{}:{} → {}:{} {} bytes of TCP packet",
                      ip_header.source_addr(),
                      tcp_header.source_port(),
                      ip_header.destination_addr(),
                      tcp_header.destination_port(),
                      data.len()
            );
        }
    }
}