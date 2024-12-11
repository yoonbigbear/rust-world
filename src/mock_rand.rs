
fn mock_rand(n: u8) -> f32 {

    let base: u32 = 0b0_01111110_00000000000000000000000;

    let large_n = (n as u32) << 15;

    let f32_bits = base | large_n;

    let m = f32::from_bits(f32_bits);

    2.0 * ( m - 0.5 )
}

#[cfg(test)]
mod tests {
    use super::*; // 부모 모듈의 모든 항목을 가져옴

}