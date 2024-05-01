pub type MacAddress = [u8; 6];

pub fn to_string(mac: &[u8; 6]) -> String {
    // Convert each byte to a two-character hex string and join them with colons
    mac.iter()
        .map(|byte| format!("{:02x}", byte)) // Ensure two digits with padding if necessary, and lowercase hex
        .collect::<Vec<_>>()
        .join(":")
}
