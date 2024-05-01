use std::net::Ipv4Addr;

pub fn same_subnet(src: Ipv4Addr, dest: Ipv4Addr, subnet: String) -> bool {
    let subnet_parsed: Ipv4Addr = subnet.parse().unwrap();

    let src_network = ipv4_to_u32(src.octets()) & ipv4_to_u32(subnet_parsed.octets());
    let dest_network = ipv4_to_u32(dest.octets()) & ipv4_to_u32(subnet_parsed.octets());

    src_network == dest_network
}

// to_bits is nightly experimental on Ipv4Addr so we have to do it ourselves
fn ipv4_to_u32(octets: [u8; 4]) -> u32 {
    ((octets[0] as u32) << 24)
        | ((octets[1] as u32) << 16)
        | ((octets[2] as u32) << 8)
        | (octets[3] as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subnet_routing() {
        let src = Ipv4Addr::new(192, 168, 0, 1);
        let dest = Ipv4Addr::new(192, 168, 0, 2);
        let subnet = "255.255.0.0".to_owned();

        let result = same_subnet(src, dest, subnet);
        assert!(result);
    }
}
