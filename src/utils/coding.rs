pub fn decoded_fixed_32(data: &str) -> u32 {
    let bytes = data.as_bytes();
    (bytes[0] as u32) |
        ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16)
        | ((bytes[3] as u32) << 24)
}
