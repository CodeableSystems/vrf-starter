use anchor_lang::solana_program::keccak;
use std::convert::TryInto;

pub fn expand(randomness: [u8; 256], n: u32) -> u32 {
    let mut hasher = keccak::Hasher::default();
    hasher.hash(&randomness);
    hasher.hash(&n.to_le_bytes());

    u32::from_le_bytes(
        hasher.result().to_bytes()[0..4]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

pub fn concat(vec: &[u32]) -> u32 {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}

