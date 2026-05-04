use blake3;

pub fn hash_data(data: &[u8]) -> [u8; 32] {
    *blake3::hash(data).as_bytes()
}
