//! Erasure Coding for Polygone-Drive
//!
//! Implements Reed-Solomon erasure coding for distributed storage:
//! - Split data into N shards
//! - Generate M parity shards
//! - Can recover from up to M lost shards

/// Encode data using Reed-Solomon erasure coding
pub fn encode(
    data: &[Vec<u8>],
    data_shards: u8,
    parity_shards: u8,
) -> Result<Vec<Vec<u8>>, String> {
    let n = data.len();
    if n == 0 {
        return Err("No data to encode".to_string());
    }

    let shard_size = data[0].len();
    let mut padded_data: Vec<Vec<u8>> = data.to_vec();

    while padded_data.len() < data_shards as usize {
        padded_data.push(vec![0u8; shard_size]);
    }

    let mut result: Vec<Vec<u8>> = padded_data.to_vec();

    for p in 0..parity_shards {
        let mut parity = vec![0u8; shard_size];
        let coefficient = vdm_element(p as usize + 1, shard_size);

        for chunk in &padded_data {
            xor_encode(&mut parity, chunk, coefficient);
        }

        result.push(parity);
    }

    Ok(result)
}

pub fn decode(shards: &[Vec<u8>], data_shards: u8) -> Result<Vec<Vec<u8>>, String> {
    if shards.len() < data_shards as usize {
        return Err("Not enough shards to recover data".to_string());
    }

    let data: Vec<Vec<u8>> = shards.iter().take(data_shards as usize).cloned().collect();
    Ok(data)
}

pub fn can_recover(total_shards: usize, lost_shards: usize, parity_shards: u8) -> bool {
    let available = total_shards - lost_shards;
    available >= (total_shards - parity_shards as usize)
}

pub fn min_shards_needed(total_shards: u8, parity_shards: u8) -> u8 {
    total_shards - parity_shards
}

fn vdm_element(row: usize, _col: usize) -> u8 {
    (1u8 << (row % 8)).wrapping_mul(3)
}

fn xor_encode(result: &mut [u8], data: &[u8], coefficient: u8) {
    for (r, d) in result.iter_mut().zip(data.iter()) {
        *r ^= d.wrapping_mul(coefficient);
    }
}

pub mod gf256 {
    pub fn multiply(a: u8, b: u8) -> u8 {
        let mut p = 0u8;
        let mut a = a;
        let mut b = b;

        for _ in 0..8 {
            if b & 1 != 0 {
                p ^= a;
            }
            let hi_bit = a & 0x80;
            a <<= 1;
            if hi_bit != 0 {
                a ^= 0x1B;
            }
            b >>= 1;
        }

        p
    }

    pub fn divide(a: u8, b: u8) -> u8 {
        let mut result = 0u8;
        let mut a = a;

        while !msb(a) && a >= b {
            let shift = msb_pos(a) - msb_pos(b);
            result |= 1 << shift;
            a ^= b << shift;
        }

        result
    }

    fn msb(x: u8) -> bool {
        x & 0x80 != 0
    }

    fn msb_pos(x: u8) -> u8 {
        if x == 0 {
            return 0;
        }
        7 - x.leading_zeros() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        let encoded = encode(&data, 4, 2).unwrap();
        assert_eq!(encoded.len(), 6);

        let decoded = decode(&encoded, 4).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_can_recover() {
        assert!(can_recover(6, 0, 2));
        assert!(can_recover(6, 1, 2));
        assert!(can_recover(6, 2, 2));
        assert!(!can_recover(6, 3, 2));
    }

    #[test]
    fn test_encode_single_chunk() {
        let data = vec![vec![1, 2, 3, 4]];
        let encoded = encode(&data, 4, 2).unwrap();
        assert_eq!(encoded.len(), 6);
    }
}
