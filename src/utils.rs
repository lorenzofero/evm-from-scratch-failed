use primitive_types::U256;

/// Flips the sign of a number using two's complement
pub fn flip_sign(num: &mut U256) {
    *num = !*num + 1;
}

/// Check if the given number is negative according to 
/// its binary representation, looking at the MSB
pub fn is_negative(num: U256) -> bool {
    num.bit(255)
}
