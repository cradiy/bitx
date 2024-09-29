use bitx::Bitx;
fn main() {
    let mut byte: Bitx<8> = 0b111u8.into();
    byte[1] = false;
    assert_eq!(byte.value(), 0b101)
}
