use crate::utils::coding::decoded_fixed_32;

pub fn hash(data: &str, n: usize, seed: u32) -> u32 {
    //similar to murmur hash
    const M: u32 = 0xc6a4a793u32;
    const R: u16 = 24u16;
    let mut h = (u128::from(seed)  ^ (u128::from(n as u32) * u128::from(M))) as u32;
    let rounds = n / 4;
    for i in 0..rounds {
        let w = decoded_fixed_32(&data[(i * 4)..(i * 4 + 4)]);
        h += w;
        h  = (u128::from(h) * u128::from(M)) as u32;
        h ^= h >> 16;
    }
    let nums = n % 4;
    let next_idx = rounds * 4;
    //pickup remaining bytes
    let bytes = data.as_bytes();
    match &nums {
        3 => {
            h += (bytes[next_idx + 2] as u32) << 16;
            h += (bytes[next_idx + 1] as u32) << 8;
            h += bytes[next_idx] as u32;
            h  = (u128::from(h) * u128::from(M)) as u32;
            h ^= h >> R;
        }
        2 => {
            h += (bytes[next_idx + 1] as u32) << 8;
            h += bytes[next_idx] as u32;
            h  = (u128::from(h) * u128::from(M)) as u32;
            h ^= h >> R;
        }
        1 => {
            h += bytes[next_idx] as u32;
            h = (u128::from(h) * u128::from(M)) as u32;
            h ^= h >> R;
        }
        _ => {}
    };
    h
}



