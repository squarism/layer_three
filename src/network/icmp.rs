use etherparse::PacketBuilder;

use crate::mac::MacAddress;

pub fn packet(src_mac: MacAddress, dest_mac: MacAddress, payload: &str) -> Vec<u8> {
    let builder = PacketBuilder::ethernet2(src_mac, dest_mac)
        .ipv4([192, 168, 6, 10], [192, 168, 6, 20], 42)
        .icmpv4_echo_request(123, 456);

    let payload = payload.as_bytes();

    let mut result = Vec::<u8>::with_capacity(builder.size(payload.len()));

    builder.write(&mut result, payload).unwrap();
    result
}
