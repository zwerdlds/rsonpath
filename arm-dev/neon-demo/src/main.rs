fn main() {
    let x = 8306756954007123088;
    let y = 12567540582320020623;
    let (res, simd) = pmul(x, y);

    println!("{res} calculated with {simd}");
}

// SIMD detection using the official methods.
pub fn pmul(a: u64, b: u64) -> (u128, &'static str) {
    #[cfg(target_arch = "aarch64")]
    {
        use std::arch::is_aarch64_feature_detected;
        if is_aarch64_feature_detected!("neon") && is_aarch64_feature_detected!("aes") {
            // Safety: target_features "neon" and "aes" are available in this block.
            return unsafe { pmul_aarch64_neon(a, b) };
        }
    }
    pmul_nosimd(a, b)
}

// SIMD implementation.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon", enable = "aes")]
unsafe fn pmul_aarch64_neon(a: u64, b: u64) -> (u128, &'static str) {
    use std::arch::aarch64::vmull_p64;

    // Safety: target_features "neon" and "aes" are available in this function.
    let result: u128 = vmull_p64(a, b);
    (result, "aarch64_neon")
}

// Fallback implementation
pub fn pmul_nosimd(a: u64, b: u64) -> (u128, &'static str) {
    let mut tmp: u128 = b as u128;
    let mut result: u128 = 0;
    for i in 0..64 {
        if a & (1 << i) != 0 {
            result ^= tmp;
        }
        tmp <<= 1;
    }
    (result, "nosimd")
}