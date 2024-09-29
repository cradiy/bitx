#![no_std]
extern crate core;
mod bit;
mod x;
pub use bit::Bit;
pub use x::*;


#[cfg(feature = "dev")]
#[macro_export]
macro_rules! mbitx {
    (@new $n:expr) => {
        $crate::Bitx::from_bits([$crate::Bit::default(); $n])
    };
    (@new $n:expr, $value:expr) => {{
        let mut buf = [$crate::Bit::default(); $n];
        for i in 0..$n {
            buf[i] = (($value & (1 << i)) != 0).into();
        }
        $crate::Bitx::from_bits(buf)
    }};
    (@compute $bit:expr) => {
        $bit[..].iter().enumerate().fold(0, |output, (idx, bit)| {
            output | (if **bit { 1  << idx } else { 0 })
        })
    };
    (@compute $bit:expr => $out:ident) => {
        $bit[..].iter().enumerate().fold(0, |output: $out, (idx, bit)| {
            output | (if **bit { 1 << idx } else { 0 })
        })
    };
}
