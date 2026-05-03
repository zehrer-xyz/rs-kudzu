#![forbid(unsafe_op_in_unsafe_fn)]

use kudzu_ffi_support::KudzuSliceU8;

const REMAINDER_M: u64 = 0xc6a4_a793_5bd1_e995;
const REMAINDER_SEED: u64 = 0xe17a_1465;
const REMAINDER_R: u32 = 47;
const BLOCK_MIX: u64 = 0xbf58_476d_1ce4_e5b9;

#[inline]
fn checksum_u64(x: u64) -> u64 {
    x.wrapping_mul(BLOCK_MIX)
}

fn checksum_remainder(bytes: &[u8]) -> u64 {
    let mut h = REMAINDER_SEED ^ ((bytes.len() as u64).wrapping_mul(REMAINDER_M));

    let mut chunks = bytes.chunks_exact(8);
    for chunk in &mut chunks {
        let mut k = u64::from_ne_bytes(chunk.try_into().expect("8-byte chunk"));
        k = k.wrapping_mul(REMAINDER_M);
        k ^= k >> REMAINDER_R;
        k = k.wrapping_mul(REMAINDER_M);

        h ^= k;
        h = h.wrapping_mul(REMAINDER_M);
    }

    let remainder = chunks.remainder();
    match remainder.len() {
        7 => {
            h ^= (remainder[6] as u64) << 48;
            h ^= (remainder[5] as u64) << 40;
            h ^= (remainder[4] as u64) << 32;
            h ^= (remainder[3] as u64) << 24;
            h ^= (remainder[2] as u64) << 16;
            h ^= (remainder[1] as u64) << 8;
            h ^= remainder[0] as u64;
        }
        6 => {
            h ^= (remainder[5] as u64) << 40;
            h ^= (remainder[4] as u64) << 32;
            h ^= (remainder[3] as u64) << 24;
            h ^= (remainder[2] as u64) << 16;
            h ^= (remainder[1] as u64) << 8;
            h ^= remainder[0] as u64;
        }
        5 => {
            h ^= (remainder[4] as u64) << 32;
            h ^= (remainder[3] as u64) << 24;
            h ^= (remainder[2] as u64) << 16;
            h ^= (remainder[1] as u64) << 8;
            h ^= remainder[0] as u64;
        }
        4 => {
            h ^= (remainder[3] as u64) << 24;
            h ^= (remainder[2] as u64) << 16;
            h ^= (remainder[1] as u64) << 8;
            h ^= remainder[0] as u64;
        }
        3 => {
            h ^= (remainder[2] as u64) << 16;
            h ^= (remainder[1] as u64) << 8;
            h ^= remainder[0] as u64;
        }
        2 => {
            h ^= (remainder[1] as u64) << 8;
            h ^= remainder[0] as u64;
        }
        1 => h ^= remainder[0] as u64,
        _ => {}
    }
    if !remainder.is_empty() {
        h = h.wrapping_mul(REMAINDER_M);
    }

    h ^= h >> REMAINDER_R;
    h = h.wrapping_mul(REMAINDER_M);
    h ^= h >> REMAINDER_R;
    h
}

pub fn checksum(bytes: &[u8]) -> u64 {
    let mut result = 5381u64;
    let mut chunks = bytes.chunks_exact(8);
    for chunk in &mut chunks {
        result ^= checksum_u64(u64::from_ne_bytes(chunk.try_into().expect("8-byte chunk")));
    }
    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        result ^= checksum_remainder(remainder);
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn kudzu_common_checksum(buffer: KudzuSliceU8) -> u64 {
    checksum(unsafe { buffer.as_slice() })
}

#[cfg(test)]
mod tests {
    use super::{checksum, checksum_remainder};

    #[test]
    fn empty_checksum_matches_cpp_seed() {
        assert_eq!(checksum(&[]), 5381);
    }

    #[test]
    fn checksum_handles_remainder_only() {
        assert_eq!(checksum(&[1, 2, 3]), 5381 ^ checksum_remainder(&[1, 2, 3]));
    }

    #[test]
    fn checksum_handles_multiple_blocks_and_tail() {
        let bytes = b"rust-owned-checksum";
        assert_eq!(checksum(bytes), 17544447021567405525);
    }
}
