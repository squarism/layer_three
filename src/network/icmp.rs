use etherparse::PacketBuilder;
use std::net::Ipv4Addr;

// ICMP is at layer 3 along with IP so we can do both at the same time here.  This is
// what the etherparse library can work with later this will be wrapped into an ethernet frame
pub fn packet(src_ip: String, dest_ip: String, payload: &str) -> Vec<u8> {
    let source: Ipv4Addr = src_ip.parse().unwrap();
    let destination: Ipv4Addr = dest_ip.parse().unwrap();

    let mut buffer = Vec::<u8>::new();
    let icmp =
        PacketBuilder::ipv4(source.octets(), destination.octets(), 20).icmpv4_echo_request(42, 1);
    icmp.write(&mut buffer, payload.as_bytes()).unwrap();
    buffer
}
