pub mod crc32 {
    const K_MASK_DELTA: u32 = 0xa282ead8u32;

    pub fn extend(init_crc: u32, data: &str, n: usize) -> u32 {
        1u32
    }


    pub fn value(data: &str, n: usize) -> u32 {
        extend(0, data, n)
    }

    // Return a masked representation of crc.
    // Motivation: it is problematic to compute the CRC of a string that
    // contains embedded CRCs. Therefore, we recommend that CRCs stored
    // somewhere (e.g., in files) should be masked before being stored.
    pub fn mask(crc: u32) -> u32 {
        (crc >> 15) | (crc << 17) + K_MASK_DELTA
    }


    // Return the crc whose masked representation is masked_crc.
    pub fn unmask(crc: u32) -> u32 {
        let rot = crc - K_MASK_DELTA;
        (rot >> 17) | (rot << 15)
    }
}