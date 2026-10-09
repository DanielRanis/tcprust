use std::io;

enum State {
    Closed,
    Listen,
    //SynRecv,
    //Estab,
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: ReceiveSequenceSpace,
}

// Struct for the State of the Send Sequence Space (See: RFC 793 S3.2 F4)
pub struct SendSequenceSpace {
    // send unacknowledged
    una: u32,
    // send next
    nxt: u32,
    // send window
    wnd: u16,
    // send urgent pointer
    up: bool,
    // segment sequence number used for last window update
    wl1: bool,
    // segment acknowledgement number used for last window update
    wl2: bool,
    // initial send sequence number
    iss: u32,
}
// Struct for the State of the Send Sequence Space (See: RFC 793 S3.2 F5)
pub struct ReceiveSequenceSpace {
    // receive next
    nxt: u32,
    // receive window
    wnd: u16,
    // receive urgent pointer
    up: bool,
    // initial receive sequence number
    irs: u32,
}

impl Default for Connection{
    fn default() -> Self{
        //Connection::Closed
        Connection {
            state: State::Listen,
        }
    }
}

impl Connection{
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a> ,
        data: &'a[u8]) -> io::Result<usize>
    {
        let mut buf = [0u8; 1500];
        match self.state {
                State::Closed => {
                    return Ok(0);
                }
                State::Listen => {
                    if !tcp_header.syn() {
                        // Only expected SYN packet
                        return Ok(0);
                    }
                    // keep track of sender info
                    self.recv.irs = tcp_header.sequence_number();
                    self.recv.nxt = tcp_header.sequence_number() + 1;
                    self.recv.wnd = tcp_header.window_size();

                    // decide on stuff we're sending them
                    self.send.iss = 0;
                    self.send.una = self.send.iss;
                    self.send.nxt = self.send.una + 1;
                    self.send.wnd = 10;

                    // Establish a connection
                    let mut syn_ack = etherparse::TcpHeader::new(
                        tcp_header.destination_port(),
                        tcp_header.source_port(),
                        self.send.iss,
                        self.send.wnd
                    );
                    syn_ack.acknowledgment_number = self.recv.nxt as u32;
                    syn_ack.syn = true;
                    syn_ack.ack = true;
                    let ip = etherparse::Ipv4Header::new(
                        syn_ack.header_len(),
                        64,
                        etherparse::IpTrafficClass::Tcp,
                        [
                            ip_header.destination()[0],
                            ip_header.destination()[1],
                            ip_header.destination()[2],
                            ip_header.destination()[3],
                            ],
                        [
                            ip_header.source()[0],
                            ip_header.source()[1],
                            ip_header.source()[2],
                            ip_header.source()[3],
                        ],
                    );
                    let unwritten ={
                        let mut unwritten = &mut buf[..];
                        ip.write(&mut unwritten);
                        syn_ack.write(&mut unwritten);
                        unwritten.len()
                    };
                    nic.send(&buf[..unwritten])


                }
                //State::SynRecv => {}
                //State::Estab => {}
        }
        /*
        eprintln!("{}:{} → {}:{} {} bytes of TCP packet",
                      ip_header.source_addr(),
                      tcp_header.source_port(),
                      ip_header.destination_addr(),
                      tcp_header.destination_port(),
                      data.len()
        );
        */
    }
}
